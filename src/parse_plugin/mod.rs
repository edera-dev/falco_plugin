use crate::proto::generated::protect::control::v1::ZoneKernelSyscallEvent;
use anyhow::Result;
use falco_event::events::Event;
use falco_plugin::parse::{EventInput as ParseEventInput, ParseInput, ParsePlugin};
use falco_plugin::source::PluginEvent;
use log::{debug, error, warn};
use prost::Message;

use crate::{EderaPlugin, threadstate};

impl ParsePlugin for EderaPlugin {
    type Event<'a> = falco_event::events::RawEvent<'a>;

    // Currently, this relies on a "regular" KV store for zone/thread info, which is not exported to the Falco core as a "table"
    // This is fine, because ATM we only need an internal-state thread table. Later, for the container plugin,
    // we may need to export some zone/thread info to other plugins for consumption, and will have to change that.
    fn parse_event(
        &mut self,
        event: &ParseEventInput<falco_event::events::RawEvent>,
        _parse_input: &ParseInput,
    ) -> Result<()> {
        let event = event.event()?;
        let res: Event<PluginEvent<&[u8]>> = event.load()?;
        let data = res.params.event_data;
        let zone_evt = ZoneKernelSyscallEvent::decode_length_delimited(data).expect("must decode");

        match self.threadstate.process_event(&zone_evt) {
            Ok(_) => Ok(()),
            Err(e) => {
                // if we got an event that indicates a CPU hotplug event happened in-zone while processing,
                // we need to invalidate the threadcache and instruct the zone watcher client to disconnect
                // from the zone and reinitialize the connection for that zone, so we get a new/fresh snapshot
                // post-hotplug.
                if let Some(threadstate::ProcessingError::ZoneCPUHotplug(msg)) =
                    e.downcast_ref::<threadstate::ProcessingError>()
                {
                    let zone_id = zone_evt.zone_id.clone();
                    warn!(
                        "CPU hotplug event detected for zone {}: {} - reinitializing zone event stream",
                        zone_id, msg
                    );
                    if let Some(tx) = &self.resub_tx {
                        if let Err(send_err) = tx.try_send(zone_id.clone()) {
                            error!(
                                "Failed to queue resubscribe for zone {}: {}",
                                zone_id, send_err
                            );
                        } else {
                            debug!("Successfully queued resubscribe for zone {}", zone_id);
                        }
                    }
                    // signaled the reconnect for this zone, carry on
                    return Ok(());
                }

                // just bubble up any other kinds of Err
                Err(e)
            }
        }
    }
}
