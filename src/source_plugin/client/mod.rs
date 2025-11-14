use anyhow::Result;
use async_stream::stream;

use crate::proto::generated::protect::control::v1::{
    MonitorZoneKernelEventReply, MonitorZoneKernelEventRequest, ZoneKernelEventStreamStop,
    ZoneKernelEventStreamUpdate, ZoneState,
    control_service_client::ControlServiceClient,
    monitor_zone_kernel_event_request::{self as zk_req},
    watch_events_reply::Event,
};
use edera_client::{client::ControlClientProvider, dial::ControlDialAddress, events::EventStream};

mod zone_creation_watcher;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use tokio::{
    sync::{mpsc, watch},
    task::{JoinHandle, JoinSet},
};
use tokio_stream::{Stream, StreamExt, wrappers::BroadcastStream};
use tonic::{Streaming, transport::Channel};

mod edera_client;
mod enabled_syscalls;

const DAEMON_SOCKET: &str = "unix:///var/lib/edera/protect/daemon.socket";

struct PumpHandle {
    zone_pump_handle: Option<JoinHandle<()>>,
    cancel_tx: Option<watch::Sender<bool>>,
}

impl PumpHandle {
    pub fn new(zone_pump_handle: JoinHandle<()>, cancel_tx: watch::Sender<bool>) -> Self {
        PumpHandle {
            zone_pump_handle: Some(zone_pump_handle),
            cancel_tx: Some(cancel_tx),
        }
    }

    pub async fn turn_off(&mut self) {
        if let Some(cancel) = self.cancel_tx.take() {
            let _ = cancel.send(true);
        }

        if let Some(handle) = self.zone_pump_handle.take() {
            let _ = handle.await;
        }
    }
}

impl Drop for PumpHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.zone_pump_handle.take() {
            handle.abort();
        }
    }
}

pub struct Client {
    zone_watch_handle: Option<JoinHandle<()>>,
    zone_pump_handles: HashMap<String, PumpHandle>,
    socket_client: Option<ControlServiceClient<Channel>>,
    pub mirror_syscalls: bool,
}

impl Drop for Client {
    fn drop(&mut self) {
        if !self.zone_pump_handles.is_empty() {
            info!("terminating zone pumps...");

            for (_zone_id, mut handle) in self.zone_pump_handles.drain() {
                tokio::task::spawn(async move {
                    handle.turn_off().await;
                });
            }
        }

        if let Some(watch_handle) = self.zone_watch_handle.take() {
            debug!("zone watch handle terminating...");
            watch_handle.abort();
        }
    }
}

impl Client {
    pub fn new() -> Client {
        Client {
            zone_watch_handle: None,
            zone_pump_handles: HashMap::new(),
            socket_client: None,
            mirror_syscalls: false,
        }
    }

    pub async fn run(
        mut self,
        sender: mpsc::UnboundedSender<MonitorZoneKernelEventReply>,
        mut resub_rx: mpsc::Receiver<String>,
        zone_exit_tx: mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        let connection: ControlDialAddress = DAEMON_SOCKET.to_string().parse()?;

        let local_client = ControlClientProvider::dial(connection).await?;
        self.socket_client = Some(local_client.clone());

        debug!("starting zone watcher");
        let z_watcher = zone_creation_watcher::ZoneWatcher {};
        let (zone_results, zone_watch_handle) = z_watcher.watch(local_client).await?;
        self.zone_watch_handle = Some(zone_watch_handle);

        let mut zone_results_stream = BroadcastStream::new(zone_results);

        info!("waiting for zones");
        loop {
            tokio::select! {
                msg = zone_results_stream.next() => {
                    match msg {
                        Some(Ok(zone)) => {
                            self.start_zone_pump(zone.clone(), sender.clone(), zone_exit_tx.clone()).await?;
                        }
                        Some(Err(e)) => eprintln!("Error: {}", e),
                        None => {
                            info!("Zone results stream ended");
                            break;
                        }
                    }
                }
                resub_req = resub_rx.recv() => {
                    match resub_req {
                        Some(zone_id) => {
                            info!("Processing resubscribe request for zone {}", zone_id);
                            if let Err(e) = self.resubscribe_zone(&zone_id, sender.clone(), zone_exit_tx.clone()).await {
                                error!("Failed to resubscribe zone {}: {}", zone_id, e);
                            }
                        }
                        None => {
                            info!("Resubscribe channel closed");
                            break;
                        }
                    }
                }
            }
        }
        debug!("done waiting");
        Ok(())
    }

    /// Resubscribe to kernel events for a specific zone.
    /// This stops the existing stream and starts a new one.
    /// Can be useful if the event system needs to be reinitialized for a zone
    /// (due to CPU hotplugging invalidating results, etc etc)
    async fn resubscribe_zone(
        &mut self,
        zone_id: &str,
        sender: mpsc::UnboundedSender<MonitorZoneKernelEventReply>,
        zone_exit_tx: mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        debug!("Resubscribing to zone {}", zone_id);

        // Stop existing pump if it exists
        if let Some(mut handle) = self.zone_pump_handles.remove(zone_id) {
            info!("Stopping existing pump for zone {}", zone_id);
            handle.turn_off().await;
        }

        // Start new pump
        self.start_zone_pump(zone_id.to_string(), sender, zone_exit_tx)
            .await
    }

    /// Start a pump for a specific zone
    async fn start_zone_pump(
        &mut self,
        zone_id: String,
        sender: mpsc::UnboundedSender<MonitorZoneKernelEventReply>,
        zone_exit_tx: mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        let (cancel_tx, cancel_rx) = watch::channel::<bool>(false);
        let done_cancel = cancel_tx.clone();
        let zone_id_local = zone_id.clone();

        let enabled_syscalls = if self.mirror_syscalls {
            info!("mirroring syscalls enabled on host");
            // This assumes falco is running and has already created its `interesting_syscalls` eBPF map
            // on the host, which contains the list of syscall IDs its rules engine cares about,
            // based on loaded rules. This also assumes the process loading us (e.g. falco)
            // has RO `/proc` permissions on the host.
            //
            // By the time falco has finished plugin init and we see zones, the map we scan to get this
            // info should be there.
            let res = enabled_syscalls::read_enabled_syscalls()
                .ok()
                .map(|syscalls| syscalls.into_iter().collect());

            if res.is_none() {
                // if we failed to read the map, fall back on None and the default set will be used
                warn!(
                    "could not read falco's enabled syscalls eBPF map, is the modern eBPF host driver enabled?"
                )
            }
            res
        } else {
            None
        };

        // Get the stored client and sender
        let client = self
            .socket_client
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Client not initialized - run() must be called first"))?
            .clone();

        let zone_pump_handle = tokio::spawn(async move {
            debug!("starting zone event pump for zone {}", zone_id_local);
            let mut res = Self::run_syscall_stream(
                client,
                sender,
                enabled_syscalls,
                zone_id_local.clone(),
                cancel_rx,
                zone_exit_tx,
            )
            .await
            .expect("should happen");
            debug!("waiting for zone event pump stream to terminate");
            res.join_next().await;
            let _ = done_cancel.send(true);
            debug!("zone event pump stream terminated");
        });

        debug!("pushing handle for zone {}", zone_id);
        self.zone_pump_handles
            .insert(zone_id, PumpHandle::new(zone_pump_handle, cancel_tx));

        Ok(())
    }

    async fn run_syscall_stream(
        mut client: ControlServiceClient<Channel>,
        handler: mpsc::UnboundedSender<MonitorZoneKernelEventReply>,
        enabled_syscalls: Option<Vec<u32>>,
        zone_id: String,
        cancel_rx: watch::Receiver<bool>,
        zone_exit_tx: mpsc::UnboundedSender<String>,
    ) -> Result<JoinSet<()>> {
        info!("Listening for kernel events from zone {}", zone_id);
        let event = MonitorZoneKernelEventRequest {
            zone_id: zone_id.clone(),
            request: Some(zk_req::Request::Update(ZoneKernelEventStreamUpdate {
                enabled_syscalls: enabled_syscalls.unwrap_or_default(),
            })),
        };
        let stream = Self::input_stream_kernel_events(event, cancel_rx);

        debug!("built stream");
        let output = client
            .monitor_zone_kernel_events(stream)
            .await?
            .into_inner();

        debug!("sent initial req");
        let events = EventStream::open(client.clone()).await.inspect_err(|_| {
            error!("Error connecting to event stream!");
        })?;

        let mut set = JoinSet::new();
        set.spawn(async move {
            if let Err(e) = Self::emit_events(output, handler).await {
                error!("error emitting events: {e}")
            }
        });
        set.spawn(async move {
            let _ = Self::zone_exit_hook(zone_id.clone(), events).await;
            let _ = zone_exit_tx.send(zone_id.clone());
            debug!("zone exit hook fired");
        });
        Ok(set)
    }

    async fn emit_events(
        mut stream: Streaming<MonitorZoneKernelEventReply>,
        handler: mpsc::UnboundedSender<MonitorZoneKernelEventReply>,
    ) -> Result<()> {
        while let Some(reply) = stream.next().await {
            let reply = reply?;
            handler.send(reply)?;
        }
        debug!("sent initial req, bailed from emit loop");
        Ok(())
    }

    fn input_stream_kernel_events(
        initial: MonitorZoneKernelEventRequest,
        mut cancel_rx: watch::Receiver<bool>,
    ) -> impl Stream<Item = MonitorZoneKernelEventRequest> {
        let stream = stream! {
            // save our stop event here.
            let term = MonitorZoneKernelEventRequest {
                zone_id: initial.zone_id.clone(),
                request: Some(zk_req::Request::Stop(ZoneKernelEventStreamStop{})),
            };

            yield initial;

            // Wait for "true" to be pushed, or the channel to close with Err().
            // Either way, carry on and stop.
            let _ = cancel_rx.wait_for(|cancelled| *cancelled).await;

            debug!("yielding stop event");
            // Yield the stop event when cancelled
            yield term;
        };

        stream
    }

    pub async fn zone_exit_hook(id: String, events: EventStream) -> Option<i32> {
        let mut stream = events.subscribe();
        while let Ok(event) = stream.recv().await {
            let Event::ZoneChanged(changed) = event else {
                continue;
            };
            let Some(zone) = changed.zone else {
                continue;
            };

            let Some(status) = zone.status else {
                continue;
            };

            if zone.id != id {
                continue;
            }

            if let Some(exit_status) = status.exit_status {
                return Some(exit_status.code);
            }

            let state = status.state();
            if state == ZoneState::Destroying || state == ZoneState::Destroyed {
                return Some(10);
            }
        }
        None
    }
}
