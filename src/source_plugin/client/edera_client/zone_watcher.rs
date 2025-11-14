use super::events::EventStream;
use crate::proto::generated::protect::control::v1::{
    ListZonesRequest, Zone, ZoneState, control_service_client::ControlServiceClient,
    watch_events_reply::Event,
};
use anyhow::Result;
use log::warn;
use std::{collections::HashMap, str::FromStr, time::Duration};
use tokio::{select, sync::broadcast::Receiver, time::sleep};
use tonic::transport::Channel;
use uuid::Uuid;

pub struct ZoneWatcher {
    control: ControlServiceClient<Channel>,
    pub events: EventStream,
}

#[derive(Debug, Clone)]
pub struct ZoneMetadata {
    pub uuid: Uuid,
    pub status: ZoneState,
}

impl ZoneWatcher {
    pub async fn new(control: ControlServiceClient<Channel>) -> Result<ZoneWatcher> {
        let client = control.clone();
        Ok(ZoneWatcher {
            control,
            events: EventStream::open(client).await?,
        })
    }

    pub async fn read(&mut self) -> Result<Vec<ZoneMetadata>> {
        let mut all_zones: HashMap<Uuid, Zone> = HashMap::new();

        let mut stream = self
            .control
            .list_zones(ListZonesRequest {})
            .await?
            .into_inner();

        while let Some(reply) = stream.message().await? {
            for zone in reply.zones {
                let Ok(uuid) = Uuid::from_str(&zone.id) else {
                    continue;
                };
                all_zones.insert(uuid, zone);
            }
        }

        let mut results: Vec<ZoneMetadata> = Vec::new();
        for (uuid, zone) in &all_zones {
            let Some(ref status) = zone.status else {
                continue;
            };

            if status.domid == 0 || status.domid == u32::MAX {
                continue;
            }

            results.push(ZoneMetadata {
                uuid: *uuid,
                status: status.state(),
            });
        }
        Ok(results)
    }

    pub async fn wait(&mut self, receiver: &mut Receiver<Event>) -> Result<()> {
        loop {
            select! {
                x = receiver.recv() => match x {
                    Ok(Event::ZoneChanged(_)) => {
                        break;
                    },

                    Ok(Event::WorkloadChanged(_)) => {
                        continue;
                    }

                    Err(error) => {
                        warn!("failed to receive event: {error}");
                    }
                },

                _ = sleep(Duration::from_secs(5)) => {
                    break;
                }
            }
        }
        Ok(())
    }
}
