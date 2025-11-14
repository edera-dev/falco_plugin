use anyhow::Result;
use crate::proto::generated::protect::{
    control::v1::ZoneState, control::v1::control_service_client::ControlServiceClient,
};
use tonic::transport::Channel;


use super::edera_client::zone_watcher::ZoneWatcher as watcher;
use log::{debug, error};
use tokio::{sync::broadcast, task::JoinHandle};

pub struct ZoneWatcher {}

impl ZoneWatcher {
    pub async fn watch(
        self,
        client: ControlServiceClient<Channel>,
    ) -> Result<(broadcast::Receiver<String>, JoinHandle<()>)> {
        let (tx, rx) = broadcast::channel(100);
        let mut autowatch = watcher::new(client).await?;
        let mut events = autowatch.events.subscribe();

        let watcher_handle = tokio::task::spawn(async move {
            let mut last_ready_zones: Vec<String> = Vec::new();
            loop {
                autowatch
                    .wait(&mut events)
                    .await
                    .expect("could not wait for zone events");
                match autowatch.read().await {
                    Ok(zone_snap) => {
                        let mut ready_zones = Vec::new();
                        for zone in zone_snap {
                            let zid = zone.uuid.to_string();
                            // the ControlServiceClient will let you send IDM messages
                            // to zones that are not in Ready state yet.
                            // We don't care about zones that aren't ready yet, so filter them out.
                            if zone.status == ZoneState::Ready {
                                // if it's ready, add it to the list whether we've seen it or not.
                                ready_zones.push(zid.clone());
                                // send it if we haven't seen it before.
                                if !last_ready_zones.contains(&zid) {
                                    debug!("got new zone {:?}", zone);
                                    let _ = tx.send(zid);
                                }
                            }
                        }

                        // purge everything that's not in the current ready-zone list
                        last_ready_zones.retain(|existing_zid| ready_zones.contains(existing_zid));
                    }
                    Err(error) => {
                        error!("failed to read zones from daemon: {error}");
                        continue;
                    }
                };
            }
        });
        Ok((rx, watcher_handle))
    }
}
