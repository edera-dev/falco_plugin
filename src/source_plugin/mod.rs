use crate::proto::generated::protect::control::v1::{
    MonitorZoneKernelEventReply, monitor_zone_kernel_event_reply::Reply,
};
use anyhow::{Error, Result, anyhow};
use falco_event::events::EventToBytes;
use falco_plugin::FailureReason;
use falco_plugin::source::{EventBatch, EventInput, SourcePlugin, SourcePluginInstance};
use log::{debug, info, warn};
use prost::Message;
use std::ffi::{CStr, CString};
use std::io::BufWriter;
use std::thread;
use std::time::Duration;
use tokio::{runtime, sync::mpsc, sync::oneshot};

use crate::EderaPlugin;

mod client;

pub struct EderaSourcePluginInstance {
    runtime: Option<tokio::runtime::Runtime>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    event_rx: Option<mpsc::UnboundedReceiver<MonitorZoneKernelEventReply>>,
    zone_exit_rx: Option<mpsc::UnboundedReceiver<String>>,
}

impl Drop for EderaSourcePluginInstance {
    fn drop(&mut self) {
        debug!("drop instance");
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_timeout(std::time::Duration::from_secs(5));
        }
        debug!("drop instance done");
    }
}

impl EderaSourcePluginInstance {
    pub fn take_events(
        &mut self,
        max_count: u32,
        batch: &mut EventBatch,
        plugin: &mut EderaPlugin,
    ) -> Result<u32> {
        let mut drained_count = 0;
        if let Some(rx) = &mut self.event_rx {
            for _ in 0..max_count {
                match rx.try_recv() {
                    Ok(event) => {
                        match event.reply {
                            // extract the internal syscall struct (if present) to
                            // pull out the timestamp and use it as the scap event's timestamp.
                            Some(Reply::Syscall(evt)) => {
                                let encoded = evt.encode_length_delimited_to_vec();
                                let mut wrapped_evt = Self::plugin_event(encoded.as_slice());
                                // set the wrapped evt TS to the original evt ts
                                wrapped_evt.metadata.ts = evt.timestamp;
                                batch.add(wrapped_evt).expect("event should add");
                                drained_count += 1;
                            }
                            Some(Reply::Threadsnap(snap)) => {
                                // We will silently *take* snapshot events here,
                                // filtering them out of the plugin event stream at this point,
                                // and populate our internal thread table with them, rather than
                                // adding them to the batch and passing them back to `scap`.
                                debug!(
                                    "thread snapshot: zone_id {:?} entry_count: {}",
                                    snap.zone_id,
                                    snap.thread_info.len(),
                                );
                                plugin.threadstate.init_zone_with_snap(snap);
                            }
                            None => {
                                warn!("got empty event!")
                            }
                        }
                    }
                    Err(mpsc::error::TryRecvError::Empty) => break,
                    Err(_) => return Err(anyhow!("channel closed")),
                }
            }
        }

        // now process any deleted zones and purge threadstate for them
        // we want to handle these before dealing with any subsequent events
        if let Some(zone_dead_rx) = &mut self.zone_exit_rx {
            while let Ok(term_zone_id) = zone_dead_rx.try_recv() {
                debug!("zone {term_zone_id} is dead, dropping from threadsnap");
                plugin.threadstate.drop_zone_from_snap(&term_zone_id);
            }
        }

        Ok(drained_count)
    }
}

impl SourcePlugin for EderaPlugin {
    type Instance = EderaSourcePluginInstance;
    const EVENT_SOURCE: &'static CStr = c"edera_zone";
    const PLUGIN_ID: u32 = 26;
    type Event<'a> = falco_event::events::RawEvent<'a>;

    fn open(&mut self, _params: Option<&str>) -> Result<Self::Instance, Error> {
        let runtime = runtime::Builder::new_multi_thread().enable_all().build()?;
        debug!("spawning event task");
        // TODO(bml) unbounded is probably best here but backpressure might be good in the future.
        let (evt_tx, evt_rx) = mpsc::unbounded_channel::<MonitorZoneKernelEventReply>();
        let (zone_exit_tx, zone_exit_rx) = mpsc::unbounded_channel::<String>();
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let (resub_tx, resub_rx) = mpsc::channel::<String>(1);
        let mirror_syscalls = self.config.mirror_host_syscalls;
        // we need to store the resub tx channel in the core plugin type, so both the Source and
        // Parse plugin impls can see it.
        self.resub_tx = Some(resub_tx.clone());

        // Spawn the async task
        runtime.spawn(async move {
            let mut client = client::Client::new();

            if let Some(mirror_flag) = mirror_syscalls {
                client.mirror_syscalls = mirror_flag;
            }

            // Run the client with a select on shutdown signal
            tokio::select! {
                result = client.run(evt_tx, resub_rx, zone_exit_tx) => {
                    debug!("client run exited");
                    // Handle the result if needed
                    if let Err(e) = result {
                        info!("Client error: {e}");
                        eprintln!("Client error: {}", e);
                    }
                }
                _ = shutdown_rx => {
                    // Shutdown signal received
                    info!("Shutting down client");
                    println!("Shutting down client...");
                }
            }
        });

        Ok(Self::Instance {
            runtime: Some(runtime),
            shutdown_tx: Some(shutdown_tx),
            event_rx: Some(evt_rx),
            zone_exit_rx: Some(zone_exit_rx),
        })
    }

    fn event_to_string(&mut self, event: &EventInput<Self::Event<'_>>) -> Result<CString, Error> {
        let plugin_event = event.event()?;
        let mut buf = BufWriter::new(Vec::new());
        plugin_event.write(&mut buf)?;
        let bytes = buf.into_inner()?;
        Ok(CString::new(bytes)?)
    }
}

impl SourcePluginInstance for EderaSourcePluginInstance {
    type Plugin = EderaPlugin;

    fn next_batch(
        &mut self,
        plugin: &mut Self::Plugin,
        batch: &mut EventBatch,
    ) -> Result<(), Error> {
        if let Ok(evts) = self.take_events(40, batch, plugin) {
            if evts == 0 {
                thread::sleep(Duration::from_millis(10));
                return Err(anyhow!("no events right now").context(FailureReason::Timeout));
            }
            debug!("emitted {evts} events");
        } else {
            return Err(anyhow!("eof").context(FailureReason::Eof));
        }
        // Falco docs say this is polled in a forloop by their main thread and
        // "shouldn't return too fast or too slow, 10-100 MS is fine"
        // so IDK here you go
        thread::sleep(Duration::from_millis(10));
        Ok(())
    }
}
