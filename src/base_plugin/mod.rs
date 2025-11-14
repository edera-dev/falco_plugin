use crate::proto::generated::protect::control::v1::{
    ZoneKernelFdInfo, ZoneKernelSyscallEvent, ZoneKernelThreadInfo,
    zone_kernel_fd_info_data::InfoType,
};
use anyhow::{Error, Result, anyhow};
use dns_lookup::lookup_addr;
use falco_event::events::Event;
use falco_event::types::IpNet as FalcoIpNet;
use falco_plugin::base::{Json, Plugin};
use falco_plugin::extract::ExtractRequest;
use falco_plugin::schemars::JsonSchema;
use falco_plugin::serde::Deserialize;
use falco_plugin::source::PluginEvent;
use falco_plugin::tables::TablesInput;
use libscap_bindings::types::{
    ppm_event_code as event_codes, ppm_param_type as param_type, scap_fd_type as fd_types,
    scap_l4_proto as l4_types,
};
use log::info;
use prost::Message;
use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::Path;
use tokio::sync::mpsc;

use crate::{parsers, threadstate};

// a handful of libsinsp/libscap consts
// duplicated here
const SE_EINPROGRESS: i64 = 115;
const SE_ETIMEOUT: i64 = 110;
const SE_EAGAIN: i64 = 11;

const STDIN_FD: u64 = 0;
const STDOUT_FD: u64 = 1;
const STDERR_FD: u64 = 2;

// TODO(bml) we do not need a thread table entry now.
// but we will for the container plugin. Defer until then.

// the zone thread table is itself keyed by zoneid,
// and contains a thread table (keyed by tid)
// #[derive(export::Entry)]
// struct ZoneThreadTableEntry {
//     // keyed by tid
//    thread_table: Box<export::Table<u64, ThreadTableEntry>>
// }

// #[derive(export::Entry)]
// struct ThreadTableEntry {
//     exe: export::Readonly<CString>,
//     cmd: export::Readonly<CString>,
//     pidns_init_start_ts: export::Readonly<u64>,
//     vpid: export::Readonly<u64>,
//     vtid: export::Readonly<u64>,
//     ptid: export::Readonly<u64>,
//     args: export::Readonly<CString>,
// }

// #[derive(PartialEq)]
// enum parsers::OpenType {
//     Read,
//     Write,
//     Exec,
//     Create,
// }
// #[derive(Debug, Display)]
// pub enum ProcessingError {
//     ZoneCPUHotplug(String),
// }

// impl std::error::Error for ProcessingError {}

pub struct EderaPlugin {
    pub config: EderaPluginConfig,
    pub threadstate: threadstate::ThreadState,
    pub resub_tx: Option<mpsc::Sender<String>>,
}

#[derive(JsonSchema, Deserialize)]
#[schemars(crate = "falco_plugin::schemars")]
#[serde(crate = "falco_plugin::serde")]
pub struct EderaPluginConfig {
    pub mirror_host_syscalls: Option<bool>,
}

impl Plugin for EderaPlugin {
    const NAME: &'static CStr = c"edera";
    const PLUGIN_VERSION: &'static CStr = c"0.0.1";
    const DESCRIPTION: &'static CStr = c"Edera base plugin (implements event source, extract, and parse for Edera zone syscall events)";
    const CONTACT: &'static CStr = c"contact@edera.dev";
    type ConfigType = Json<EderaPluginConfig>;

    fn new(input: Option<&TablesInput>, Json(config): Self::ConfigType) -> Result<Self, Error> {
        info!("loading plugin");
        let Some(_input) = input else {
            anyhow::bail!("Did not get tables input");
        };
        Ok(EderaPlugin {
            threadstate: threadstate::ThreadState::default(),
            resub_tx: None,
            config,
        })
    }
}

enum EventPathType {
    Singular(String),
    Source(String),
    Target(String),
}

#[derive(Default)]
enum Cached<T> {
    #[default]
    NotFetched,
    Found(T),
    NotFound,
}

#[derive(Default)]
pub struct EderaZoneSyscallContext {
    decoded_evt: ZoneKernelSyscallEvent,
    evt_thread: Cached<ZoneKernelThreadInfo>,
    leader_thread: Cached<ZoneKernelThreadInfo>,
    fdinfo: Cached<ZoneKernelFdInfo>,
}

impl EderaPlugin {
    pub fn extract_zone_id(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            Ok(CString::new(zone_evt.zone_id.clone()).expect("should cstring"))
        })
    }

    pub fn extract_type(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            Ok(CString::new(zone_evt.event_name.clone()).expect("should cstring"))
        })
    }

    pub fn extract_type_is(&mut self, mut req: ExtractRequest<Self>, arg: &CStr) -> Result<bool> {
        Ok(self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            arg.to_bytes() == zone_evt.event_name.as_bytes()
        }))
    }

    pub fn extract_category(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            Ok(CString::new(zone_evt.event_category.clone()).expect("should cstring"))
        })
    }

    pub fn extract_cpu(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| Ok(zone_evt.cpuid as u64))
    }

    pub fn extract_latency(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);

        let enter_ts = self
            .threadstate
            .with_zoneinfo(&ctx.decoded_evt.zone_id, |zinfo| {
                zinfo
                    .get_enter_event(&ctx.decoded_evt)
                    .map(|enter_evt| enter_evt.timestamp)
            });
        let Some(enter_t) = enter_ts else {
            return Ok(0);
        };

        Ok(ctx.decoded_evt.timestamp - enter_t)
    }

    pub fn extract_latency_s(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);

        let enter_ts = self
            .threadstate
            .with_zoneinfo(&ctx.decoded_evt.zone_id, |zinfo| {
                zinfo
                    .get_enter_event(&ctx.decoded_evt)
                    .map(|enter_evt| enter_evt.timestamp)
            });
        let Some(enter_t) = enter_ts else {
            return Ok(0);
        };

        Ok(ctx.decoded_evt.timestamp - enter_t / 1_000_000_000)
    }

    pub fn extract_latency_ns(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);

        let enter_ts = self
            .threadstate
            .with_zoneinfo(&ctx.decoded_evt.zone_id, |zinfo| {
                zinfo
                    .get_enter_event(&ctx.decoded_evt)
                    .map(|enter_evt| enter_evt.timestamp)
            });
        let Some(enter_t) = enter_ts else {
            return Ok(0);
        };

        Ok(ctx.decoded_evt.timestamp - enter_t % 1_000_000_000)
    }

    pub fn extract_latency_human(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);

        let enter_ts = self
            .threadstate
            .with_zoneinfo(&ctx.decoded_evt.zone_id, |zinfo| {
                zinfo
                    .get_enter_event(&ctx.decoded_evt)
                    .map(|enter_evt| enter_evt.timestamp)
            });

        let Some(enter_t) = enter_ts else {
            return Ok(CString::new("0s")?);
        };

        let latency_ns = ctx.decoded_evt.timestamp - enter_t;

        let formatted = if latency_ns < 1_000 {
            format!("{}ns", latency_ns)
        } else if latency_ns < 1_000_000 {
            format!("{:.1}us", latency_ns as f64 / 1_000.0)
        } else if latency_ns < 1_000_000_000 {
            format!("{:.1}ms", latency_ns as f64 / 1_000_000.0)
        } else {
            format!("{:.1}s", latency_ns as f64 / 1_000_000_000.0)
        };

        Ok(CString::new(formatted)?)
    }

    pub fn extract_args(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            let mut concat_str = String::new();
            concat_str.push_str("| ");
            for param in &zone_evt.event_params {
                concat_str.push_str(
                    format!("{} {}", param.param_type_pretty, param.param_pretty).as_str(),
                );
                concat_str.push_str("| ");
            }
            Ok(CString::new(concat_str).expect("should cstring"))
        })
    }

    pub fn extract_arg(&mut self, mut req: ExtractRequest<Self>, arg: u64) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if let Some(param) = zone_evt.event_params.get(arg as usize) {
                Ok(CString::new(param.param_pretty.clone()).expect("should cstring"))
            } else {
                Ok(CString::new("").unwrap())
            }
        })
    }

    pub fn extract_dir(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            // scap events use the ">"\"<" symbol for this :shrug:
            if parsers::is_enter(zone_evt) {
                Ok(CString::new(">").expect("should cstring"))
            } else {
                Ok(CString::new("<").expect("should cstring"))
            }
        })
    }

    /// distinct from `error` in that not all syscall errors indicate true failure & etc
    pub fn extract_failed(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if parsers::has_retval(zone_evt) {
                // The return value is always the first parameter of the syscall event
                // It could have different names depending on the event type `res`,`fd`, etc.
                let retval =
                    i64::from_ne_bytes(zone_evt.event_params[0].param_data.as_slice().try_into()?);
                if (retval != 0)
                    && (retval != SE_EINPROGRESS)
                    && (retval != SE_EAGAIN)
                    && (retval != SE_ETIMEOUT)
                {
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Err(anyhow!("event should have return code, but none found!"))
            }
        })
    }

    pub fn extract_retval_str(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if parsers::has_retval(zone_evt) {
                // The return value is always the first parameter of the syscall event
                // It could have different names depending on the event type `res`,`fd`, etc.
                let retval =
                    i64::from_ne_bytes(zone_evt.event_params[0].param_data.as_slice().try_into()?);
                if retval >= 0 {
                    Ok(CString::new("SUCCESS").expect("should cstring"))
                } else {
                    Ok(CString::new(zone_evt.event_params[0].param_pretty.clone())
                        .expect("should cstring"))
                }
            } else {
                Err(anyhow!("event should have return code, but none found!"))
            }
        })
    }

    pub fn extract_buffer_len(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            // sinsp checks for a FD object but I think that's unnecessary?
            if parsers::is_io(zone_evt)
                && !parsers::is_enter(zone_evt)
                && parsers::has_retval(zone_evt)
            {
                // The return value is always the first parameter of the syscall event
                // It could have different names depending on the event type `res`,`fd`, etc.
                let retval =
                    i64::from_ne_bytes(zone_evt.event_params[0].param_data.as_slice().try_into()?);
                if retval >= 0 {
                    return Ok(retval as u64);
                }
                Ok(0)
            } else {
                Err(anyhow!("event should have return code, but none found!"))
            }
        })
    }

    /// Note that, to be thorough, this checks all params of type bytebuffer for a match,
    /// and returns the first match found
    pub fn extract_buffer_contains_hex(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: &CStr,
    ) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            for param in &zone_evt.event_params {
                if param.param_type == param_type::PT_BYTEBUF as u32
                    && parsers::vec_contains_hex_bytes(&param.param_data, arg)
                {
                    return Ok(true);
                }
            }
            Ok(false)
        })
    }

    pub fn extract_is_io(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| Ok(parsers::is_io(zone_evt)))
    }

    pub fn extract_is_io_read(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| Ok(parsers::is_io_read(zone_evt)))
    }

    pub fn extract_is_io_write(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| Ok(parsers::is_io_write(zone_evt)))
    }

    pub fn extract_is_wait(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| Ok(parsers::is_wait(zone_evt)))
    }

    pub fn extract_io_dir(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            // scap events use the "r"\"w" symbol for this :shrug:
            if parsers::is_io(zone_evt) {
                if parsers::is_io_read(zone_evt) {
                    Ok(CString::new("r").expect("should cstring"))
                } else {
                    Ok(CString::new("w").expect("should cstring"))
                }
            } else {
                Ok(CString::new("").unwrap())
            }
        })
    }

    pub fn extract_is_open_read(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if let Ok(res) = parsers::get_openstate(zone_evt) {
                return Ok(res == parsers::OpenType::Read);
            }
            Ok(false)
        })
    }

    pub fn extract_is_open_write(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if let Ok(res) = parsers::get_openstate(zone_evt) {
                return Ok(res == parsers::OpenType::Write);
            }
            Ok(false)
        })
    }

    pub fn extract_is_open_exec(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if let Ok(res) = parsers::get_openstate(zone_evt) {
                return Ok(res == parsers::OpenType::Exec);
            }
            Ok(false)
        })
    }

    pub fn extract_is_open_create(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if let Ok(res) = parsers::get_openstate(zone_evt) {
                return Ok(res == parsers::OpenType::Create);
            }
            Ok(false)
        })
    }

    pub fn extract_count(&mut self, _: ExtractRequest<Self>) -> Result<u64> {
        // docs and code confirm this always returns 1. might be legacy.
        Ok(1)
    }

    pub fn extract_count_error(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, Self::get_count_error)
    }

    // the libsinsp version of this prefers to interrogate local FD entries to get the info here,
    // but also has a fallback if no FD present. So we do the fallback using only event info here,
    // since it's not worth extracting full FD context from zones.
    pub fn extract_count_error_file(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            let etype = event_codes::from_repr(zone_evt.event_type)
                .ok_or(anyhow!("could not parse event type"))
                .expect("should parse");
            if parsers::is_open_file(etype) || etype == event_codes::PPME_SYSCALL_CREAT_X {
                Self::get_count_error(zone_evt)
            } else {
                Ok(0)
            }
        })
    }

    // the libsinsp version of this prefers to interrogate local FD entries to get the info here,
    // but also has a fallback if no FD present. So we do the fallback using only event info here,
    // since it's not worth extracting full FD context from zones.
    pub fn extract_count_error_net(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            let etype = event_codes::from_repr(zone_evt.event_type)
                .ok_or(anyhow!("could not parse event type"))
                .expect("should parse");
            if parsers::is_open_net(etype) {
                Self::get_count_error(zone_evt)
            } else {
                Ok(0)
            }
        })
    }

    pub fn extract_count_error_mem(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if zone_evt.event_category.contains("MEMORY") {
                Self::get_count_error(zone_evt)
            } else {
                Ok(0)
            }
        })
    }

    // If it's not file OR net OR memory, and is an error
    pub fn extract_count_error_other(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            let etype = event_codes::from_repr(zone_evt.event_type)
                .ok_or(anyhow!("could not parse event type"))
                .expect("should parse");
            if !parsers::is_open_file(etype)
                && !parsers::is_open_net(etype)
                && !zone_evt.event_category.contains("MEMORY")
            {
                Self::get_count_error(zone_evt)
            } else {
                Ok(0)
            }
        })
    }

    pub fn extract_count_exit(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        self.with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
            if parsers::is_enter(zone_evt) {
                Ok(1)
            } else {
                Ok(0)
            }
        })
    }

    fn get_count_error(evt: &ZoneKernelSyscallEvent) -> Result<u64> {
        if parsers::has_retval(evt) {
            let retval = i64::from_ne_bytes(evt.event_params[0].param_data.as_slice().try_into()?);
            if retval < 0 { Ok(1) } else { Ok(0) }
        } else {
            Ok(0)
        }
    }

    fn with_zone_threadinfo_ctx<F, R>(&mut self, req: &mut ExtractRequest<Self>, f: F) -> Option<R>
    where
        F: FnOnce(&ZoneKernelThreadInfo) -> Option<R>,
    {
        match self.get_or_cache_evt_thread(req) {
            Cached::Found(t) => f(t),
            _ => None,
        }
    }

    fn get_or_cache_evt_ctx<'a>(
        &mut self,
        req: &'a mut ExtractRequest<Self>,
    ) -> &'a EderaZoneSyscallContext {
        (if let Some(ctx) = req.context {
            ctx
        } else {
            req.context.get_or_insert_with(|| {
                let evt = req.event.event().expect("must be event");
                let res: Event<PluginEvent<&[u8]>> = evt.load().expect("must load");
                let data = res.params.event_data;
                let zone_evt =
                    ZoneKernelSyscallEvent::decode_length_delimited(data).expect("must decode");
                EderaZoneSyscallContext {
                    decoded_evt: zone_evt,
                    ..Default::default()
                }
            })
        }) as _
    }

    /// this is a separate step from caching the overall event context because depending on
    /// what kind of extract request we get, that may not be necessary.
    fn get_or_cache_evt_thread<'a>(
        &mut self,
        req: &'a mut ExtractRequest<Self>,
    ) -> &'a Cached<ZoneKernelThreadInfo> {
        self.get_or_cache_evt_ctx(req);
        let context = req.context.as_mut().expect("context set by prev. call");

        let zid = &context.decoded_evt.zone_id;
        let tid = &context.decoded_evt.thread_id;
        if matches!(context.evt_thread, Cached::NotFetched) {
            context.evt_thread = match self
                .threadstate
                .with_threadinfo(zid, tid, |tinfo| Some(tinfo.clone()))
            {
                Some(t) => Cached::Found(t),
                None => Cached::NotFound,
            };
        }

        &context.evt_thread
    }

    /// this is a separate step from caching the overall event context because depending on
    /// what kind of extract request we get, that may not be necessary.
    fn get_or_cache_evt_leader_thread<'a>(
        &mut self,
        req: &'a mut ExtractRequest<Self>,
    ) -> &'a Cached<ZoneKernelThreadInfo> {
        // we can't get the leader thread for this thread unless we HAVE this thread,
        // so explicitly cache that first.
        self.get_or_cache_evt_thread(req);
        let context = req.context.as_mut().expect("context set by prev. call");

        let zid = &context.decoded_evt.zone_id;
        let tid = &context.decoded_evt.thread_id;
        if matches!(context.leader_thread, Cached::NotFetched) {
            context.leader_thread = match self.get_main_thread(zid, tid) {
                Some(t) => Cached::Found(t),
                None => Cached::NotFound,
            };
        }
        &context.leader_thread
    }

    /// this is a separate step from caching the overall event context because depending on
    /// what kind of extract request we get, that may not be necessary.
    fn get_or_cache_evt_fdinfo<'a>(
        &mut self,
        req: &'a mut ExtractRequest<Self>,
    ) -> &'a Cached<ZoneKernelFdInfo> {
        // we usually can't get the fdlist unless we have the leader thread,
        // so explicitly try to cache that first.
        self.get_or_cache_evt_leader_thread(req);
        let context = req.context.as_mut().expect("context set by prev. call");

        let zid = &context.decoded_evt.zone_id;
        if matches!(context.fdinfo, Cached::NotFetched) {
            context.fdinfo = match self.threadstate.with_zoneinfo(zid, |zinfo| {
                zinfo.get_enterexit_event_fdinfo(&context.decoded_evt)
            }) {
                Some(t) => Cached::Found(t),
                None => Cached::NotFound,
            };
        }
        &context.fdinfo
    }

    /// libsinsp uses a dual-pointer search moving at different rates for this
    /// which I'm not replicating because there are other spots that need optimization first.
    fn find_oldest_ancestor_thread<P>(
        &self,
        zid: &str,
        tid: &u64,
        predicate: P,
    ) -> Option<ZoneKernelThreadInfo>
    where
        P: Fn(&ZoneKernelThreadInfo) -> bool,
    {
        let mut current_tid = *tid;
        let mut res: Option<ZoneKernelThreadInfo> = None;

        loop {
            let Some(thread) = self.get_main_thread(zid, &current_tid) else {
                break;
            };

            if predicate(&thread) {
                res = Some(thread.clone());
            }

            // Try to move to parent, or break if no parent exists
            if let Some(ptid) = thread.ptid {
                current_tid = ptid;
            } else {
                break;
            }
        }

        res
    }

    /// libsinsp gloms all fds into the leader thread, regardless of whether CLONE_FILES is
    /// set or not. This is a nice lookup optimization, and harmless in practice.
    ///
    /// We do something similar - we prefer to stick the FDs into the leader thread's FD list.
    /// If for some reason we are dealing with a thread that does not have a leader,
    /// we fall back to sticking them in the local thread.
    ///
    /// So when looking for FDs we only have to check 2 places here - our own thread,
    /// and the leader.
    fn find_fds(&self, zid: &str, tid: &u64, fdi: &u64) -> Option<ZoneKernelFdInfo> {
        let mut next_tid_to_check = *tid;

        if let Some(fdinfo) = self.threadstate.with_threadinfo(zid, tid, |child_tinfo| {
            next_tid_to_check = child_tinfo.pid();
            child_tinfo.fdlist.get(fdi).cloned()
        }) {
            Some(fdinfo)
        } else {
            self.threadstate
                .with_threadinfo(zid, &next_tid_to_check, |leader_tinfo| {
                    leader_tinfo.fdlist.get(fdi).cloned()
                })
        }
    }

    fn with_nth_parent_proc_thread<F, R>(
        &mut self,
        zid: &str,
        tid: &u64,
        depth: usize,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&ZoneKernelThreadInfo) -> Option<R>,
    {
        let mut current_tid = self.get_main_thread(zid, tid)?.tid?;

        // Walk up the parent chain N times
        for _ in 0..depth {
            let tinfo = self.get_main_thread(zid, &current_tid)?;
            current_tid = tinfo.ptid?;
        }

        // Get the final ancestor thread's info and apply the FnOnce
        let thread = self.get_main_thread(zid, &current_tid)?;
        f(&thread)
    }

    // BEGIN self-proc extract
    pub fn extract_abspath(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.cwd.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_exe(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.exe.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_exepath(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.exepath.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_name(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.comm.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_args(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.args.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_cmdline(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                CString::new(format!("{} {}", tinfo.comm.clone(), tinfo.args.clone())).ok()
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_argcount(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                Some(tinfo.args.split('\0').collect::<Vec<&str>>().len() as u64)
            })
            .unwrap_or(0))
    }

    pub fn extract_proc_exeline(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                CString::new(format!("{} {}", tinfo.exe.clone(), tinfo.args.clone())).ok()
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_env(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.env.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_arglen(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                Some(
                    tinfo
                        .args
                        .chars()
                        .filter(|&c| !matches!(c, '\0' | ' '))
                        .count() as u64,
                )
            })
            .unwrap_or(0))
    }

    pub fn extract_proc_cwd(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.cwd.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_loginshellid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let loginshellthread = self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.find_oldest_ancestor_thread(&zid, &tid, |maybe_shell_tinfo| {
                    maybe_shell_tinfo.comm.ends_with("sh")
                })
            });
        // 0 is The Wrong value to return here but the Falco plugin API doesn't have signed ints.
        Ok(loginshellthread.and_then(|lshell| lshell.pid).unwrap_or(0))
    }

    pub fn extract_proc_pid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| tinfo.pid)
            .unwrap_or(0))
    }

    pub fn extract_proc_tty(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.tty))
            .unwrap_or(0)
            .into())
    }

    pub fn extract_proc_vpid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| tinfo.vpid)
            .unwrap_or(0))
    }

    pub fn extract_proc_sid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.sid))
            .unwrap_or(0))
    }

    pub fn extract_proc_sname(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our sid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.sid == tinfo.sid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.comm.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_sexe(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our sid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.sid == tinfo.sid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.exe.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_sexepath(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our sid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.sid == tinfo.sid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.exepath.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_vpgid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.vpgid))
            .unwrap_or(0))
    }

    pub fn extract_proc_vpgidname(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our vpgid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.vpgid == tinfo.vpgid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.comm.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_vpgidexe(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our vpgid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.vpgid == tinfo.vpgid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.exe.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_vpgidexepath(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our vpgid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.vpgid == tinfo.vpgid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.exepath.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_pgid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.pgid))
            .unwrap_or(0))
    }

    pub fn extract_proc_pgidname(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our vpgid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.pgid == tinfo.pgid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.comm.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_pgidexe(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our vpgid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.pgid == tinfo.pgid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.exe.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_pgidexepath(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        // might be in a pid namespace, so cannot just do TID lookups, have to walk up
        // and look for the oldest ancestor with our vpgid
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.threadstate.with_threadinfo(&zid, &tid, |tinfo| {
                    self.find_oldest_ancestor_thread(&zid, &tid, |maybe_an| {
                        maybe_an.pgid == tinfo.pgid
                    })
                })
            })
            .and_then(|slead| CString::new(slead.exepath.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_duration(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| Some(zone_evt.clone()))
            .and_then(|evt| {
                self.threadstate
                    .with_threadinfo(&evt.zone_id, &evt.thread_id, |tinfo| {
                        if tinfo.clone_ts != 0 {
                            Some(evt.timestamp - tinfo.clone_ts)
                        } else {
                            None
                        }
                    })
            })
            .unwrap_or(0))
    }

    pub fn extract_proc_pid_ts(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.clone_ts))
            .unwrap_or(0))
    }

    pub fn extract_proc_exe_writable(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.exe_writable))
            .unwrap_or(false))
    }

    pub fn extract_proc_exe_upper_layer(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.exe_upper_layer))
            .unwrap_or(false))
    }

    pub fn extract_proc_exe_lower_layer(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.exe_lower_layer))
            .unwrap_or(false))
    }

    pub fn extract_proc_exe_from_memfd(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.exe_from_memfd))
            .unwrap_or(false))
    }

    pub fn extract_proc_sid_leader(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                Some(tinfo.vpid.is_some_and(|vpid| vpid == tinfo.sid))
            })
            .unwrap_or(false))
    }

    pub fn extract_proc_vpgid_leader(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                Some(tinfo.vpid.is_some_and(|vpid| vpid == tinfo.vpgid))
            })
            .unwrap_or(false))
    }

    pub fn extract_proc_pgid_leader(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                Some(tinfo.pid.is_some_and(|pid| pid == tinfo.pgid))
            })
            .unwrap_or(false))
    }

    pub fn extract_proc_exe_ino(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.exe_ino))
            .unwrap_or(0))
    }

    pub fn extract_proc_exe_ino_ctime(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.exe_ino_ctime))
            .unwrap_or(0))
    }

    pub fn extract_proc_exe_ino_mtime(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.exe_ino_mtime))
            .unwrap_or(0))
    }

    pub fn extract_proc_exe_ino_ctime_duration_proc_start(
        &mut self,
        mut req: ExtractRequest<Self>,
    ) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                Some(tinfo.exe_ino_ctime_duration_clone_ts)
            })
            .unwrap_or(0))
    }

    pub fn extract_proc_exe_ino_ctime_duration_pidns_start(
        &mut self,
        mut req: ExtractRequest<Self>,
    ) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                Some(tinfo.exe_ino_ctime_duration_pidns_start)
            })
            .unwrap_or(0))
    }

    pub fn extract_thread_cap_permitted(
        &mut self,
        mut req: ExtractRequest<Self>,
    ) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                CString::new(Self::caps_to_string(tinfo.cap_permitted)).ok()
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_thread_cap_inheritable(
        &mut self,
        mut req: ExtractRequest<Self>,
    ) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                CString::new(Self::caps_to_string(tinfo.cap_inheritable)).ok()
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_thread_cap_effective(
        &mut self,
        mut req: ExtractRequest<Self>,
    ) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                CString::new(Self::caps_to_string(tinfo.cap_effective)).ok()
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_fdopencount(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.fdlist.len() as u64))
            .unwrap_or(0))
    }

    pub fn extract_proc_fdlimit(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.fdlimit as u64))
            .unwrap_or(0))
    }

    pub fn extract_proc_fdusage(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                let fdopenct = tinfo.fdlist.len() as u64;
                let fdlimit = tinfo.fdlimit as u64;
                if (fdopenct) < (fdlimit) {
                    Some((fdopenct * 100) / fdlimit)
                } else {
                    Some(100)
                }
            })
            .unwrap_or(0))
    }

    pub fn extract_proc_vmsize(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.vmsize_kb as u64))
            .unwrap_or(0))
    }

    pub fn extract_proc_vmrss(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.vmrss_kb as u64))
            .unwrap_or(0))
    }

    pub fn extract_proc_vmswap(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.vmswap_kb as u64))
            .unwrap_or(0))
    }

    pub fn extract_thread_pfmajor(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.pfmajor))
            .unwrap_or(0))
    }

    pub fn extract_thread_pfminor(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.pfminor))
            .unwrap_or(0))
    }

    pub fn extract_thread_tid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| tinfo.tid)
            .unwrap_or(0))
    }

    pub fn extract_thread_vtid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| tinfo.vtid)
            .unwrap_or(0))
    }

    pub fn extract_thread_ismain(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| Some(parsers::thread_main(tinfo)))
            .unwrap_or(false))
    }

    pub fn extract_thread_exectime(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| Some(zone_evt.clone()))
            .and_then(|evt| {
                self.threadstate.with_zoneinfo(&evt.zone_id, |zinfo| {
                    zinfo.get_exectime(&evt.cpuid).and_then(|exec_time| {
                        if evt.event_type == event_codes::PPME_SCHEDSWITCH_1_E as u32
                            || evt.event_type == event_codes::PPME_SCHEDSWITCH_6_E as u32
                        {
                            Some(exec_time.last_switch_ts - exec_time.previous_switch_ts)
                        } else {
                            // TODO(bml) libsinsp only reports this for explicit switch events,
                            // we could actually do better, but for now maintain strict compat
                            None
                        }
                    })
                })
            })
            .unwrap_or(0))
    }

    pub fn extract_thread_total_exectime(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| Some(zone_evt.clone()))
            .and_then(|evt| {
                self.threadstate.with_zoneinfo(&evt.zone_id, |zinfo| {
                    zinfo.get_exectime(&evt.cpuid).and_then(|exec_time| {
                        if evt.event_type == event_codes::PPME_SCHEDSWITCH_1_E as u32
                            || evt.event_type == event_codes::PPME_SCHEDSWITCH_6_E as u32
                        {
                            Some(exec_time.cumulative_switch_time)
                        } else {
                            // TODO(bml) libsinsp only reports this for explicit switch events,
                            // we could actually do better, but for now maintain strict compat
                            None
                        }
                    })
                })
            })
            .unwrap_or(0))
    }

    pub fn extract_thread_cgroups(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| CString::new(tinfo.cgroups.clone()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_thread_cgroup(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: &CStr,
    ) -> Result<CString> {
        let subsystem = arg.to_str()?;
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                tinfo
                    .cgroups
                    .split('\0')
                    .filter(|s| !s.is_empty())
                    .find_map(|entry| {
                        // Each entry is "subsystem=path"
                        if let Some((name, path)) = entry.split_once('=')
                            && (name == subsystem || name == format!("{}_cgroup", subsystem))
                        {
                            CString::new(path.to_string()).ok();
                        }
                        None
                    })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_nthreads(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| Some(zone_evt.clone()))
            .and_then(|evt| {
                self.threadstate.with_zoneinfo(&evt.zone_id, |zinfo| {
                    Some(zinfo.get_thread_groupsize(&evt.thread_id, false))
                })
            })
            .unwrap_or(0))
    }

    pub fn extract_proc_nchilds(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| Some(zone_evt.clone()))
            .and_then(|evt| {
                self.threadstate.with_zoneinfo(&evt.zone_id, |zinfo| {
                    Some(zinfo.get_thread_groupsize(&evt.thread_id, true))
                })
            })
            .unwrap_or(0))
    }

    /// same as proc_vmsize but only checks the main thread
    pub fn extract_thread_vmsize(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                if parsers::thread_main(tinfo) {
                    Some(tinfo.vmsize_kb as u64)
                } else {
                    Some(0)
                }
            })
            .unwrap_or(0))
    }

    /// same as proc_vmrss but only checks the main thread
    pub fn extract_thread_vmrss(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_threadinfo_ctx(&mut req, |tinfo| {
                if parsers::thread_main(tinfo) {
                    Some(tinfo.vmrss_kb as u64)
                } else {
                    Some(0)
                }
            })
            .unwrap_or(0))
    }

    pub fn extract_proc_stdxx_type(
        &mut self,
        mut req: ExtractRequest<Self>,
        fd: &u64,
    ) -> Result<CString> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| self.find_fds(&zid, &tid, fd))
            .and_then(|fdinfo| CString::new(fdinfo.fd_type.to_string()).ok())
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_stdxx_name(
        &mut self,
        mut req: ExtractRequest<Self>,
        fd: &u64,
    ) -> Result<CString> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| self.find_fds(&zid, &tid, fd))
            .and_then(|fdinfo| {
                match fdinfo
                    .info
                    .as_ref()
                    .and_then(|data| data.info_type.as_ref())
                {
                    Some(InfoType::FileName(fname)) => CString::new(fname.to_string()).ok(),
                    Some(InfoType::RegularFile(finfo)) => CString::new(finfo.name.clone()).ok(),
                    _ => None,
                }
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_proc_stdin_type(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        self.extract_proc_stdxx_type(req, &STDIN_FD)
    }

    pub fn extract_proc_stdout_type(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        self.extract_proc_stdxx_type(req, &STDOUT_FD)
    }

    pub fn extract_proc_stderr_type(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        self.extract_proc_stdxx_type(req, &STDERR_FD)
    }

    pub fn extract_proc_stdin_name(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        self.extract_proc_stdxx_name(req, &STDIN_FD)
    }

    pub fn extract_proc_stdout_name(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        self.extract_proc_stdxx_name(req, &STDOUT_FD)
    }

    pub fn extract_proc_stderr_name(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        self.extract_proc_stdxx_name(req, &STDERR_FD)
    }
    // END self-proc extract
    //
    // BEGIN fd extract
    pub fn extract_fd_num(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(match self.get_or_cache_evt_fdinfo(&mut req) {
            Cached::Found(fdinfo) => fdinfo.fd,
            _ => None,
        }
        .unwrap_or(0))
    }

    pub fn extract_fd_type(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(match self.get_or_cache_evt_fdinfo(&mut req) {
            Cached::Found(fdinfo) => {
                if let Some(fdt) = fd_types::from_repr(fdinfo.fd_type) {
                    CString::new(Self::fdtype_to_string(fdt)).ok()
                } else {
                    None
                }
            }
            _ => None,
        }
        .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_fd_typechar(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(match self.get_or_cache_evt_fdinfo(&mut req) {
            Cached::Found(fdinfo) => {
                if let Some(fdt) = fd_types::from_repr(fdinfo.fd_type) {
                    let fchar = Self::fdtype_to_string(fdt)
                        .chars()
                        .nth(0)
                        .expect("enum should have chars");
                    CString::new(fchar.to_string()).ok()
                } else {
                    None
                }
            }
            _ => None,
        }
        .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_fd_name(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(match self.get_or_cache_evt_fdinfo(&mut req) {
            Cached::Found(fdinfo) => CString::new(Self::fdinfo_to_string(fdinfo)).ok(),
            _ => None,
        }
        .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_fd_dir(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };

        let fpath = match &finfo.info_type {
            Some(InfoType::RegularFile(file)) => Some(file.name.as_str()),
            Some(InfoType::FileName(name)) => Some(name.as_str()),
            _ => None,
        };

        let Some(fpath) = fpath else {
            return Ok(default());
        };

        let result = fd_types::from_repr(fdinfo.fd_type).and_then(|fdt| match fdt {
            fd_types::SCAP_FD_DIRECTORY => CString::new(fpath).ok(),
            fd_types::SCAP_FD_FILE | fd_types::SCAP_FD_FILE_V2 => {
                let path = std::path::Path::new(fpath);
                let dir = path.parent()?;
                CString::new(dir.to_str()?).ok()
            }
            _ => None,
        });

        Ok(result.unwrap_or_else(default))
    }

    pub fn extract_fd_filename(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };

        let fpath = match &finfo.info_type {
            Some(InfoType::RegularFile(file)) => Some(file.name.as_str()),
            Some(InfoType::FileName(name)) => Some(name.as_str()),
            _ => None,
        };

        let Some(fpath) = fpath else {
            return Ok(default());
        };

        let result = fd_types::from_repr(fdinfo.fd_type).and_then(|fdt| match fdt {
            fd_types::SCAP_FD_FILE | fd_types::SCAP_FD_FILE_V2 => {
                let path = std::path::Path::new(fpath);
                let filename = path.file_name()?;
                CString::new(filename.to_str()?).ok()
            }
            _ => None,
        });

        Ok(result.unwrap_or_else(default))
    }

    /// If ipproto socket, returns all the IPs (source/dest) available as a list, for the filtering engine
    /// to match against
    pub fn extract_fd_ip(&mut self, mut req: ExtractRequest<Self>) -> Result<Vec<IpAddr>> {
        let mut retval = Vec::new();

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(retval);
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(retval);
        };

        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                retval.push(IpAddr::V4(sock.source_ip.parse::<Ipv4Addr>()?));
                retval.push(IpAddr::V4(sock.dest_ip.parse::<Ipv4Addr>()?));
            }
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                retval.push(IpAddr::V4(sock.local_ip.parse::<Ipv4Addr>()?));
                retval.push(IpAddr::V4(sock.local_ip.parse::<Ipv4Addr>()?));
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                retval.push(IpAddr::V6(sock.source_ip.parse::<Ipv6Addr>()?));
                retval.push(IpAddr::V6(sock.dest_ip.parse::<Ipv6Addr>()?));
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                retval.push(IpAddr::V6(sock.local_ip.parse::<Ipv6Addr>()?));
                retval.push(IpAddr::V6(sock.local_ip.parse::<Ipv6Addr>()?));
            }
            _ => {}
        };

        Ok(retval)
    }

    /// Returns client IP, if present
    pub fn extract_fd_cip(&mut self, mut req: ExtractRequest<Self>) -> Result<IpAddr> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Err(anyhow!("no info found for FD"));
        };
        if let Some(ip) = Self::get_fd_cip(fdinfo) {
            Ok(ip)
        } else {
            Err(anyhow!("no client IP for FD"))
        }
    }

    /// Returns server IP, if present
    pub fn extract_fd_sip(&mut self, mut req: ExtractRequest<Self>) -> Result<IpAddr> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Err(anyhow!("no info found for FD"));
        };
        if let Some(ip) = Self::get_fd_sip(fdinfo) {
            Ok(ip)
        } else {
            Err(anyhow!("no server IP for FD"))
        }
    }

    /// Returns local IP, if present
    pub fn extract_fd_lip(&mut self, mut req: ExtractRequest<Self>) -> Result<IpAddr> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Err(anyhow!("no info found for FD"));
        };
        if let Some(ip) = Self::get_fd_lip(fdinfo) {
            Ok(ip)
        } else {
            Err(anyhow!("no local IP for FD"))
        }
    }

    /// Returns remote IP, if present
    pub fn extract_fd_rip(&mut self, mut req: ExtractRequest<Self>) -> Result<IpAddr> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Err(anyhow!("no info found for FD"));
        };
        let Some(finfo) = fdinfo.info.as_ref() else {
            return Err(anyhow!("no info found for FD"));
        };
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => Ok(IpAddr::V4(sock.dest_ip.parse::<Ipv4Addr>()?)),
            Some(InfoType::Ipv6Socket(sock)) => Ok(IpAddr::V6(sock.dest_ip.parse::<Ipv6Addr>()?)),
            _ => Err(anyhow!("no server IP for FD")),
        }
    }

    /// If ipproto socket, returns all the ports (source/dest) available as a list, for the filtering engine
    /// to match against
    pub fn extract_fd_port(&mut self, mut req: ExtractRequest<Self>) -> Result<Vec<u64>> {
        let mut retval = Vec::new();

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(retval);
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(retval);
        };

        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                retval.push(sock.source_port.into());
                retval.push(sock.dest_port.into());
            }
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                retval.push(sock.local_port.into());
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                retval.push(sock.source_port.into());
                retval.push(sock.dest_port.into());
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                retval.push(sock.local_port.into());
            }
            _ => {}
        };

        Ok(retval)
    }

    /// Returns client port, if present
    pub fn extract_fd_cport(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(0);
        };
        if let Some(port) = Self::get_fd_cport(fdinfo) {
            Ok(port as u64)
        } else {
            Ok(0)
        }
    }

    /// Returns server ports, if present
    pub fn extract_fd_sport(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(0);
        };

        if let Some(port) = Self::get_fd_sport(fdinfo) {
            Ok(port as u64)
        } else {
            Ok(0)
        }
    }

    /// Returns local port, if present
    pub fn extract_fd_lport(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(0);
        };
        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(0);
        };
        match &finfo.info_type {
            Some(InfoType::Ipv4ServerSocket(sock)) => Ok(sock.local_port.into()),
            Some(InfoType::Ipv6ServerSocket(sock)) => Ok(sock.local_port.into()),
            _ => Ok(0),
        }
    }

    /// Returns remote port, if present
    pub fn extract_fd_rport(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(0);
        };
        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(0);
        };
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => Ok(sock.dest_port.into()),
            Some(InfoType::Ipv6Socket(sock)) => Ok(sock.dest_port.into()),
            _ => Ok(0),
        }
    }

    /// Returns l4 proto
    pub fn extract_fd_l4proto(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };
        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                Ok(CString::new(Self::l4proto_to_string(sock.protocol as u32))?)
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                Ok(CString::new(Self::l4proto_to_string(sock.protocol as u32))?)
            }
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                Ok(CString::new(Self::l4proto_to_string(sock.protocol as u32))?)
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                Ok(CString::new(Self::l4proto_to_string(sock.protocol as u32))?)
            }
            _ => Ok(default()),
        }
    }

    pub fn extract_fd_sockfamily(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };
        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(_))
            | Some(InfoType::Ipv6Socket(_))
            | Some(InfoType::Ipv4ServerSocket(_))
            | Some(InfoType::Ipv6ServerSocket(_)) => Ok(CString::new("ip")?),
            Some(InfoType::UnixSocket(_)) => Ok(CString::new("unix")?),
            _ => Ok(default()),
        }
    }

    pub fn extract_fd_is_server(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(false);
        };
        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(false);
        };
        match &finfo.info_type {
            Some(InfoType::Ipv4ServerSocket(_)) | Some(InfoType::Ipv6ServerSocket(_)) => {
                // TODO(bml) libsinsp also tries to figure out if the dest IP
                // of a non-server socket FD type points to a local container here,
                // but for now I'm skipping that.
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// generates a libsinsp-style "UID" consisting of the the thread id + FD #.
    pub fn extract_fd_uid(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");
        let Cached::Found(threadinfo) = self.get_or_cache_evt_thread(&mut req) else {
            return Ok(default());
        };

        let Some(tid) = threadinfo.tid else {
            return Ok(default());
        };

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };

        let Some(fdi) = fdinfo.fd else {
            return Ok(default());
        };

        Ok(CString::new(format! {"{}{}", tid, fdi})?)
    }

    pub fn extract_fd_proto(&mut self, mut req: ExtractRequest<Self>) -> Result<Vec<CString>> {
        let mut retval = Vec::new();

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(retval);
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(retval);
        };

        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                let sport = sock.source_port;
                let dport = sock.dest_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                retval.push(CString::new(parsers::lookup_service(sport, &proto))?);
                retval.push(CString::new(parsers::lookup_service(dport, &proto))?);
            }
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                let lport = sock.local_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                retval.push(CString::new(parsers::lookup_service(lport, &proto))?);
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                let sport = sock.source_port;
                let dport = sock.dest_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                retval.push(CString::new(parsers::lookup_service(sport, &proto))?);
                retval.push(CString::new(parsers::lookup_service(dport, &proto))?);
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                let lport = sock.local_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                retval.push(CString::new(parsers::lookup_service(lport, &proto))?);
            }
            _ => {}
        };

        Ok(retval)
    }

    pub fn extract_fd_cproto(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };

        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                let sport = sock.source_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(sport, &proto))?)
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                let sport = sock.source_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(sport, &proto))?)
            }
            _ => Ok(default()),
        }
    }

    pub fn extract_fd_sproto(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };

        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                let dport = sock.dest_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(dport, &proto))?)
            }
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                let lport = sock.local_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(lport, &proto))?)
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                let dport = sock.dest_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(dport, &proto))?)
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                let lport = sock.local_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(lport, &proto))?)
            }
            _ => Ok(default()),
        }
    }

    pub fn extract_fd_lproto(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };

        match &finfo.info_type {
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                let lport = sock.local_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(lport, &proto))?)
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                let lport = sock.local_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(lport, &proto))?)
            }
            _ => Ok(default()),
        }
    }

    pub fn extract_fd_rproto(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");

        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(default());
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(default());
        };

        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                let dport = sock.dest_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(dport, &proto))?)
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                let dport = sock.dest_port;
                let proto = Self::l4proto_to_string(sock.protocol as u32);
                Ok(CString::new(parsers::lookup_service(dport, &proto))?)
            }
            _ => Ok(default()),
        }
    }

    /// Exactly the same as the `extract_fd_xip` version, except wrapped in a falco-specific
    /// type so the plugin API knows to do a network comparison rather than a direct IP match
    pub fn extract_fd_net(&mut self, req: ExtractRequest<Self>) -> Result<Vec<FalcoIpNet>> {
        let mut retval = Vec::new();
        let Ok(ips) = self.extract_fd_ip(req) else {
            return Ok(retval);
        };

        retval = ips.iter().map(|ip| FalcoIpNet(*ip)).collect();
        Ok(retval)
    }

    /// Exactly the same as the `extract_fd_xip` version, except wrapped in a falco-specific
    /// type so the plugin API knows to do a network comparison rather than a direct IP match
    pub fn extract_fd_cnet(&mut self, req: ExtractRequest<Self>) -> Result<FalcoIpNet> {
        let Ok(ip) = self.extract_fd_cip(req) else {
            return Err(anyhow!("no IP found"));
        };

        Ok(FalcoIpNet(ip))
    }

    /// Exactly the same as the `extract_fd_xip` version, except wrapped in a falco-specific
    /// type so the plugin API knows to do a network comparison rather than a direct IP match
    pub fn extract_fd_snet(&mut self, req: ExtractRequest<Self>) -> Result<FalcoIpNet> {
        let Ok(ip) = self.extract_fd_sip(req) else {
            return Err(anyhow!("no IP found"));
        };

        Ok(FalcoIpNet(ip))
    }

    /// Exactly the same as the `extract_fd_xip` version, except wrapped in a falco-specific
    /// type so the plugin API knows to do a network comparison rather than a direct IP match
    pub fn extract_fd_lnet(&mut self, req: ExtractRequest<Self>) -> Result<FalcoIpNet> {
        let Ok(ip) = self.extract_fd_lip(req) else {
            return Err(anyhow!("no IP found"));
        };

        Ok(FalcoIpNet(ip))
    }

    /// Exactly the same as the `extract_fd_xip` version, except wrapped in a falco-specific
    /// type so the plugin API knows to do a network comparison rather than a direct IP match
    pub fn extract_fd_rnet(&mut self, req: ExtractRequest<Self>) -> Result<FalcoIpNet> {
        let Ok(ip) = self.extract_fd_rip(req) else {
            return Err(anyhow!("no IP found"));
        };

        Ok(FalcoIpNet(ip))
    }

    pub fn extract_fd_connected(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(false);
        };
        Ok(parsers::is_socket_connected(fdinfo))
    }

    /// Note that this resolves an IP from the zone using the hostside DNS resolution config.
    /// Which naturally means we may not be able to resolve a name that is resolvable in-zone.
    /// This is best-effort.
    pub fn extract_fd_cip_name(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");
        let Ok(cip) = self.extract_fd_cip(req) else {
            return Ok(default());
        };

        Ok(CString::new(lookup_addr(&cip)?)?)
    }

    /// Note that this resolves an IP from the zone using the hostside DNS resolution config.
    /// Which naturally means we may not be able to resolve a name that is resolvable in-zone.
    /// This is best-effort.
    pub fn extract_fd_sip_name(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");
        let Ok(sip) = self.extract_fd_sip(req) else {
            return Ok(default());
        };

        Ok(CString::new(lookup_addr(&sip)?)?)
    }

    /// Note that this resolves an IP from the zone using the hostside DNS resolution config.
    /// Which naturally means we may not be able to resolve a name that is resolvable in-zone.
    /// This is best-effort.
    pub fn extract_fd_lip_name(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");
        let Ok(lip) = self.extract_fd_lip(req) else {
            return Ok(default());
        };

        Ok(CString::new(lookup_addr(&lip)?)?)
    }

    /// Note that this resolves an IP from the zone using the hostside DNS resolution config.
    /// Which naturally means we may not be able to resolve a name that is resolvable in-zone.
    /// This is best-effort.
    pub fn extract_fd_rip_name(&mut self, req: ExtractRequest<Self>) -> Result<CString> {
        let default = || CString::new("NA").expect("default value must parse");
        let Ok(rip) = self.extract_fd_rip(req) else {
            return Ok(default());
        };

        Ok(CString::new(lookup_addr(&rip)?)?)
    }

    pub fn extract_fd_dev(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(0);
        };

        let Some(finfo) = fdinfo.info.as_ref() else {
            return Ok(0);
        };

        match &finfo.info_type {
            Some(InfoType::RegularFile(finfo)) => Ok(finfo.device as u64),
            _ => Ok(0),
        }
    }

    pub fn extract_fd_dev_major(&mut self, req: ExtractRequest<Self>) -> Result<u64> {
        let Ok(dev) = self.extract_fd_dev(req) else {
            return Ok(0);
        };

        // see new_encode_dev in include/linux/kdev_t.h
        Ok((dev & 0xfff00) >> 8)
    }

    pub fn extract_fd_dev_minor(&mut self, req: ExtractRequest<Self>) -> Result<u64> {
        let Ok(dev) = self.extract_fd_dev(req) else {
            return Ok(0);
        };

        // see new_encode_dev in include/linux/kdev_t.h
        Ok((dev & 0xff) | ((dev >> 12) & 0xfff00))
    }

    pub fn extract_fd_ino(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(0);
        };

        Ok(fdinfo.inode)
    }

    pub fn extract_fd_nameraw(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        Ok(CString::new(Self::extract_raw_fdname_from_event(
            &ctx.decoded_evt,
        ))?)
    }

    /// exactly like extract_fd_type but takes an arg and returns the type of that FD as a list.
    /// If arg is None, returns all types of fds held by the current thread.
    pub fn extract_fd_types(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<Vec<CString>> {
        let result = self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                if let Some(fdi) = arg {
                    self.find_fds(&zid, &tid, &fdi).and_then(|fdinfo| {
                        let fdt = fd_types::from_repr(fdinfo.fd_type)?;
                        Some(vec![CString::new(Self::fdtype_to_string(fdt)).ok()?])
                    })
                } else {
                    self.threadstate.with_zoneinfo(&zid, |zinfo| {
                        let tinfo = zinfo.get_threadinfo(&tid)?;
                        if let Some(pid) = tinfo.pid {
                            let lead_tinfo = zinfo.get_threadinfo(&pid)?;
                            Self::all_fdtypes_uniq(&lead_tinfo.fdlist).ok()
                        } else {
                            Self::all_fdtypes_uniq(&tinfo.fdlist).ok()
                        }
                    })
                }
            })
            .unwrap_or_default();

        Ok(result)
    }

    pub fn extract_fd_upper_layer(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(false);
        };

        Ok(parsers::is_upper_layer(fdinfo))
    }

    pub fn extract_fd_lower_layer(&mut self, mut req: ExtractRequest<Self>) -> Result<bool> {
        let Cached::Found(fdinfo) = self.get_or_cache_evt_fdinfo(&mut req) else {
            return Ok(false);
        };

        Ok(parsers::is_lower_layer(fdinfo))
    }
    // END fd extract

    // BEGIN fs path extract
    pub fn extract_fs_path_nameraw(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        self.get_fs_path_nameraw(&ctx.decoded_evt)
    }

    pub fn extract_fs_path_name(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let Ok(rawpath) = self.get_fs_path_nameraw(&ctx.decoded_evt) else {
            return Ok(CString::new("NA").expect("default value must parse"));
        };

        let b_string = rawpath.clone().into_string()?;

        let p = Path::new(&b_string);

        if p.is_absolute() {
            Ok(rawpath)
        } else {
            let Some(cwd) =
                self.with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.cwd.clone()))
            else {
                return Ok(rawpath);
            };

            let base = Path::new(&cwd);
            if !p.starts_with(base) {
                Ok(CString::new(
                    Path::new(&cwd).join(p).to_string_lossy().into_owned(),
                )?)
            } else {
                Ok(CString::new(p.to_string_lossy().into_owned())?)
            }
        }
    }

    pub fn extract_fs_path_sourceraw(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        self.get_fs_path_sourceraw(&ctx.decoded_evt)
    }

    pub fn extract_fs_path_source(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let Ok(rawpath) = self.get_fs_path_sourceraw(&ctx.decoded_evt) else {
            return Ok(CString::new("NA").expect("default value must parse"));
        };

        let b_string = rawpath.clone().into_string()?;

        let p = Path::new(&b_string);

        if p.is_absolute() {
            Ok(rawpath)
        } else {
            let Some(cwd) =
                self.with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.cwd.clone()))
            else {
                return Ok(rawpath);
            };

            let base = Path::new(&cwd);
            if !p.starts_with(base) {
                Ok(CString::new(
                    Path::new(&cwd).join(p).to_string_lossy().into_owned(),
                )?)
            } else {
                Ok(CString::new(p.to_string_lossy().into_owned())?)
            }
        }
    }

    pub fn extract_fs_path_targetraw(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        self.get_fs_path_targetraw(&ctx.decoded_evt)
    }

    pub fn extract_fs_path_target(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let Ok(rawpath) = self.get_fs_path_targetraw(&ctx.decoded_evt) else {
            return Ok(CString::new("NA").expect("default value must parse"));
        };

        let b_string = rawpath.clone().into_string()?;

        let p = Path::new(&b_string);

        if p.is_absolute() {
            Ok(rawpath)
        } else {
            let Some(cwd) =
                self.with_zone_threadinfo_ctx(&mut req, |tinfo| Some(tinfo.cwd.clone()))
            else {
                return Ok(rawpath);
            };

            let base = Path::new(&cwd);
            if !p.starts_with(base) {
                Ok(CString::new(
                    Path::new(&cwd).join(p).to_string_lossy().into_owned(),
                )?)
            } else {
                Ok(CString::new(p.to_string_lossy().into_owned())?)
            }
        }
    }
    // END fs path extract

    // BEGIN fdlist/PPOLL extract
    pub fn extract_fdlist_nums(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let fdlist = Self::get_fdlist_from_poll_evt(&ctx.decoded_evt);

        if fdlist.is_empty() {
            return Ok(CString::new("NA").expect("default value must parse"));
        }

        Ok(CString::new(
            fdlist
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )?)
    }

    pub fn extract_fdlist_names(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let fdlist = Self::get_fdlist_from_poll_evt(&ctx.decoded_evt);

        if fdlist.is_empty() {
            return Ok(CString::new("NA").expect("default value must parse"));
        }

        Ok(CString::new(
            fdlist
                .iter()
                .filter_map(|n| {
                    self.find_fds(&ctx.decoded_evt.zone_id, &ctx.decoded_evt.thread_id, n)
                        .map(|fdinfo| Self::fdinfo_to_string(&fdinfo))
                })
                .collect::<Vec<_>>()
                .join(","),
        )?)
    }

    pub fn extract_fdlist_cips(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let fdlist = Self::get_fdlist_from_poll_evt(&ctx.decoded_evt);

        if fdlist.is_empty() {
            return Ok(CString::new("NA").expect("default value must parse"));
        }

        Ok(CString::new(
            fdlist
                .iter()
                .filter_map(|n| {
                    if let Some(fdinfo) =
                        self.find_fds(&ctx.decoded_evt.zone_id, &ctx.decoded_evt.thread_id, n)
                    {
                        Self::get_fd_cip(&fdinfo)
                    } else {
                        None
                    }
                })
                .map(|ip| ip.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )?)
    }

    pub fn extract_fdlist_sips(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let fdlist = Self::get_fdlist_from_poll_evt(&ctx.decoded_evt);

        if fdlist.is_empty() {
            return Ok(CString::new("NA").expect("default value must parse"));
        }

        Ok(CString::new(
            fdlist
                .iter()
                .filter_map(|n| {
                    if let Some(fdinfo) =
                        self.find_fds(&ctx.decoded_evt.zone_id, &ctx.decoded_evt.thread_id, n)
                    {
                        Self::get_fd_sip(&fdinfo)
                    } else {
                        None
                    }
                })
                .map(|ip| ip.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )?)
    }

    pub fn extract_fdlist_cports(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let fdlist = Self::get_fdlist_from_poll_evt(&ctx.decoded_evt);

        if fdlist.is_empty() {
            return Ok(CString::new("NA").expect("default value must parse"));
        }

        Ok(CString::new(
            fdlist
                .iter()
                .filter_map(|n| {
                    if let Some(fdinfo) =
                        self.find_fds(&ctx.decoded_evt.zone_id, &ctx.decoded_evt.thread_id, n)
                    {
                        Self::get_fd_cport(&fdinfo)
                    } else {
                        None
                    }
                })
                .map(|port| port.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )?)
    }

    pub fn extract_fdlist_sports(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        let ctx = self.get_or_cache_evt_ctx(&mut req);
        let fdlist = Self::get_fdlist_from_poll_evt(&ctx.decoded_evt);

        if fdlist.is_empty() {
            return Ok(CString::new("NA").expect("default value must parse"));
        }

        Ok(CString::new(
            fdlist
                .iter()
                .filter_map(|n| {
                    if let Some(fdinfo) =
                        self.find_fds(&ctx.decoded_evt.zone_id, &ctx.decoded_evt.thread_id, n)
                    {
                        Self::get_fd_sport(&fdinfo)
                    } else {
                        None
                    }
                })
                .map(|port| port.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )?)
    }
    // END fdlist/PPOLL extract

    // BEGIN parent-proc extract
    pub fn extract_parent_proc_exe(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, 1, |atinfo| {
                    CString::new(atinfo.exe.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_parent_proc_exepath(
        &mut self,
        mut req: ExtractRequest<Self>,
    ) -> Result<CString> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, 1, |atinfo| {
                    CString::new(atinfo.exepath.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_parent_proc_name(&mut self, mut req: ExtractRequest<Self>) -> Result<CString> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, 1, |atinfo| {
                    CString::new(atinfo.comm.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_parent_proc_cmdline(
        &mut self,
        mut req: ExtractRequest<Self>,
    ) -> Result<CString> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, 1, |atinfo| {
                    CString::new(format!("{} {}", atinfo.comm.clone(), atinfo.args.clone())).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_parent_proc_pid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, 1, |atinfo| atinfo.pid)
            })
            .unwrap_or(0))
    }

    pub fn extract_parent_proc_vpid(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, 1, |atinfo| atinfo.vpid)
            })
            .unwrap_or(0))
    }

    pub fn extract_parent_proc_duration(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| Some(zone_evt.clone()))
            .and_then(|evt| {
                self.with_nth_parent_proc_thread(&evt.zone_id, &evt.thread_id, 1, |atinfo| {
                    if atinfo.clone_ts != 0 {
                        Some(evt.timestamp - atinfo.clone_ts)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(0))
    }

    pub fn extract_parent_proc_pid_ts(&mut self, mut req: ExtractRequest<Self>) -> Result<u64> {
        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| Some(zone_evt.clone()))
            .and_then(|evt| {
                self.with_nth_parent_proc_thread(&evt.zone_id, &evt.thread_id, 1, |atinfo| {
                    Some(atinfo.clone_ts)
                })
            })
            .unwrap_or(0))
    }
    // END parent-proc extract

    // BEGIN nth-proc extract
    pub fn extract_nth_proc_exe(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<CString> {
        // default to 0 (self) if no arg supplied
        let depth: usize = arg.unwrap_or(0).try_into()?;

        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, depth, |atinfo| {
                    CString::new(atinfo.exe.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_nth_proc_exepath(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<CString> {
        // default to 0 (self) if no arg supplied
        let depth: usize = arg.unwrap_or(0).try_into()?;

        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, depth, |atinfo| {
                    CString::new(atinfo.exepath.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_nth_proc_name(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<CString> {
        // default to 0 (self) if no arg supplied
        let depth: usize = arg.unwrap_or(0).try_into()?;

        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, depth, |atinfo| {
                    CString::new(atinfo.comm.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_nth_proc_args(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<CString> {
        // default to 0 (self) if no arg supplied
        let depth: usize = arg.unwrap_or(0).try_into()?;

        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, depth, |atinfo| {
                    CString::new(atinfo.args.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_nth_proc_cmdline(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<CString> {
        // default to 0 (self) if no arg supplied
        let depth: usize = arg.unwrap_or(0).try_into()?;

        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, depth, |atinfo| {
                    CString::new(format!("{} {}", atinfo.comm.clone(), atinfo.args.clone())).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_nth_proc_env(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<CString> {
        // default to 0 (self) if no arg supplied
        let depth: usize = arg.unwrap_or(0).try_into()?;

        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, depth, |atinfo| {
                    CString::new(atinfo.env.clone()).ok()
                })
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    pub fn extract_nth_proc_pid(
        &mut self,
        mut req: ExtractRequest<Self>,
        arg: Option<u64>,
    ) -> Result<u64> {
        // default to 0 (self) if no arg supplied
        let depth: usize = arg.unwrap_or(0).try_into()?;

        Ok(self
            .with_zone_syscall_evt_ctx(&mut req, |zone_evt| {
                let zid = zone_evt.zone_id.clone();
                let tid = zone_evt.thread_id;
                Some((zid, tid))
            })
            .and_then(|(zid, tid)| {
                self.with_nth_parent_proc_thread(&zid, &tid, depth, |atinfo| atinfo.pid)
            })
            .unwrap_or(0))
    }

    // END nth-proc extract

    /// Given a given thread ID, determine the thread group leader and return a copy of that thread's info.
    /// Note that the given thread ID may itself be the thread group leader, in which case
    /// a copy of the current thread's info will be returned.
    fn get_main_thread(&self, zone_id: &str, thread_id: &u64) -> Option<ZoneKernelThreadInfo> {
        self.threadstate.with_zoneinfo(zone_id, |zinfo| {
            let tinfo = zinfo.get_threadinfo(thread_id)?;

            if parsers::thread_main(tinfo) {
                Some(tinfo.clone())
            } else {
                let ptid = tinfo.pid?;

                zinfo.get_threadinfo(&ptid).cloned()
            }
        })
    }

    fn caps_to_string(caps: u64) -> String {
        let mut res = String::new();

        // Get all known capabilities
        let all_caps = caps::all();

        for cap in all_caps {
            let cap_index = cap.index(); // Get the u8 index (0-40)
            let current_cap: u64 = 1 << cap_index;

            if caps & current_cap != 0 {
                if !res.is_empty() {
                    res.push(' ');
                }
                res.push_str(&cap.to_string());
            }
        }

        res
    }

    fn all_fdtypes_uniq(fdlist: &HashMap<u64, ZoneKernelFdInfo>) -> Result<Vec<CString>> {
        let mut uniq = HashSet::new();
        for val in fdlist.values() {
            if let Some(fdt) = fd_types::from_repr(val.fd_type) {
                uniq.insert(CString::new(Self::fdtype_to_string(fdt))?);
            }
        }
        Ok(uniq.into_iter().collect())
    }

    fn fdtype_to_string(fdt: fd_types) -> String {
        match fdt {
            fd_types::SCAP_FD_FILE => "file".to_string(),

            fd_types::SCAP_FD_DIRECTORY => "directory".to_string(),

            fd_types::SCAP_FD_IPV4_SOCK | fd_types::SCAP_FD_IPV4_SERVSOCK => "ipv4".to_string(),
            fd_types::SCAP_FD_IPV6_SOCK | fd_types::SCAP_FD_IPV6_SERVSOCK => "ipv6".to_string(),

            fd_types::SCAP_FD_UNIX_SOCK => "unix".to_string(),

            fd_types::SCAP_FD_FIFO => "pipe".to_string(),

            fd_types::SCAP_FD_EVENT => "event".to_string(),

            fd_types::SCAP_FD_SIGNALFD => "signalfd".to_string(),

            fd_types::SCAP_FD_EVENTPOLL => "eventpoll".to_string(),
            fd_types::SCAP_FD_TIMERFD => "timferfd".to_string(),
            fd_types::SCAP_FD_INOTIFY => "inotify".to_string(),
            fd_types::SCAP_FD_NETLINK => "netlink".to_string(),
            fd_types::SCAP_FD_BPF => "bpf".to_string(),
            fd_types::SCAP_FD_USERFAULTFD => "userfaultfd".to_string(),
            fd_types::SCAP_FD_MEMFD => "memfd".to_string(),
            fd_types::SCAP_FD_PIDFD => "pidfd".to_string(),
            fd_types::SCAP_FD_IOURING => "io_uring".to_string(),
            _ => "NA".to_string(),
        }
    }

    fn l4proto_to_string(l4proto: u32) -> String {
        let proto = l4_types::from_repr(l4proto)
            .ok_or(anyhow!("could not parse proto type"))
            .expect("should parse");
        match proto {
            l4_types::SCAP_L4_TCP => "tcp".into(),
            l4_types::SCAP_L4_UDP => "udp".into(),
            l4_types::SCAP_L4_ICMP => "icmp".into(),
            l4_types::SCAP_L4_RAW => "raw".into(),
            _ => "NA".into(),
        }
    }

    fn fdinfo_to_string(fdinfo: &ZoneKernelFdInfo) -> String {
        let Some(ref info) = fdinfo.info else {
            return "NA".to_string();
        };

        match &info.info_type {
            Some(InfoType::Ipv4Socket(sockinf)) => {
                format!(
                    "{}:{} {}:{}",
                    sockinf.source_ip, sockinf.source_port, sockinf.dest_ip, sockinf.dest_port
                )
            }
            Some(InfoType::Ipv6Socket(sockinf)) => {
                format!(
                    "{}:{} {}:{}",
                    sockinf.source_ip, sockinf.source_port, sockinf.dest_ip, sockinf.dest_port
                )
            }
            Some(InfoType::Ipv4ServerSocket(sockinf)) => {
                format!("{}:{}", sockinf.local_ip, sockinf.local_port)
            }
            Some(InfoType::Ipv6ServerSocket(sockinf)) => {
                format!("{}:{}", sockinf.local_ip, sockinf.local_port)
            }
            Some(InfoType::UnixSocket(sockinf)) => {
                format!(
                    "{} {}:{}",
                    sockinf.name, sockinf.source, sockinf.destination
                )
            }
            Some(InfoType::RegularFile(finfo)) => finfo.name.to_string(),
            Some(InfoType::FileName(name)) => name.to_string(),
            _ => "NA".to_string(),
        }
    }

    pub fn extract_raw_fdname_from_event(event: &ZoneKernelSyscallEvent) -> String {
        use event_codes::*;
        let etype = event_codes::from_repr(event.event_type)
            .ok_or(anyhow!("could not parse event type"))
            .expect("should parse");

        if parsers::is_enter(event) {
            return "NA".into();
        }

        match etype {
            PPME_SYSCALL_OPEN_X
            | PPME_SOCKET_ACCEPT_X
            | PPME_SOCKET_ACCEPT_5_X
            | PPME_SOCKET_ACCEPT4_X
            | PPME_SOCKET_ACCEPT4_5_X
            | PPME_SOCKET_ACCEPT4_6_X
            | PPME_SYSCALL_CREAT_X => event.event_params[1].param_pretty.clone(),

            PPME_SOCKET_CONNECT_X => event.event_params[1].param_pretty.clone(),

            // TODO(bml) libsinsp does a more elaborate lookup for this, but this
            // should be good enough for now.
            PPME_SYSCALL_OPENAT_X => event.event_params[1].param_pretty.clone(),

            PPME_SYSCALL_OPENAT_2_X | PPME_SYSCALL_OPENAT2_X => {
                event.event_params[2].param_pretty.clone()
            }

            PPME_SYSCALL_OPEN_BY_HANDLE_AT_X => event.event_params[3].param_pretty.clone(),
            _ => "NA".into(),
        }
    }

    fn get_fs_path_nameraw(&mut self, event: &ZoneKernelSyscallEvent) -> Result<CString> {
        // OPENAT_X is specialcased to get the path from the ENTER event in libsinsp.
        // For all others, it's pulled from the exit event.
        if event.event_type == event_codes::PPME_SYSCALL_OPENAT_X as u32 {
            Ok(self
                .threadstate
                .with_zoneinfo(&event.zone_id, |zinfo| {
                    zinfo.get_enter_event(event).and_then(|enter_evt| {
                        enter_evt
                            .event_params
                            .iter()
                            .find(|param| param.name == "name")
                            .and_then(|param| CString::new(param.param_pretty.clone()).ok())
                    })
                })
                .unwrap_or(CString::new("NA").expect("default value must parse")))
        } else {
            Ok(Self::get_paths_from_evt_params(event)
                .into_iter()
                .find_map(|path_type| {
                    if let EventPathType::Singular(path) = path_type {
                        CString::new(path).ok()
                    } else {
                        None
                    }
                })
                .unwrap_or(CString::new("NA").expect("default value must parse")))
        }
    }

    fn get_fs_path_sourceraw(&mut self, event: &ZoneKernelSyscallEvent) -> Result<CString> {
        Ok(Self::get_paths_from_evt_params(event)
            .into_iter()
            .find_map(|path_type| {
                if let EventPathType::Source(path) = path_type {
                    CString::new(path).ok()
                } else {
                    None
                }
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    fn get_fs_path_targetraw(&mut self, event: &ZoneKernelSyscallEvent) -> Result<CString> {
        Ok(Self::get_paths_from_evt_params(event)
            .into_iter()
            .find_map(|path_type| {
                if let EventPathType::Target(path) = path_type {
                    CString::new(path).ok()
                } else {
                    None
                }
            })
            .unwrap_or(CString::new("NA").expect("default value must parse")))
    }

    /// for syscall events with paths we care about, return the paths directly from the event args.
    /// Some syscalls have just A Path as an arg, some have Source/Dest paths.
    /// Depending on the event in question, get all the paths by type and return them.
    fn get_paths_from_evt_params(event: &ZoneKernelSyscallEvent) -> Vec<EventPathType> {
        let mut paths = Vec::new();
        use event_codes::*;
        let etype = event_codes::from_repr(event.event_type)
            .ok_or(anyhow!("could not parse event type"))
            .expect("should parse");
        match etype {
            // Single path operations
            PPME_SYSCALL_MKDIR_2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "path") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_MKDIRAT_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "path") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_RMDIR_2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "path") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_UNLINK_2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "path") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_UNLINKAT_2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "name") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_OPEN_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "name") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_OPENAT_2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "name") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_OPENAT2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "name") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_FCHMODAT_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "filename") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_CHMOD_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "filename") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_CHOWN_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "path") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_LCHOWN_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "path") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_FCHOWNAT_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "pathname") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_QUOTACTL_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "special") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_UMOUNT_X | PPME_SYSCALL_UMOUNT_1_X | PPME_SYSCALL_UMOUNT2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "name") {
                    paths.push(EventPathType::Singular(param.param_pretty.clone()));
                }
            }

            PPME_SYSCALL_RENAME_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "oldpath") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "newpath") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_RENAMEAT_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "oldpath") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "newpath") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_RENAMEAT2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "oldpath") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "newpath") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_LINK_2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "newpath") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "oldpath") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_LINKAT_2_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "newpath") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "oldpath") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_SYMLINK_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "linkpath") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "target") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_SYMLINKAT_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "linkpath") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "target") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            PPME_SYSCALL_MOUNT_X => {
                if let Some(param) = event.event_params.iter().find(|p| p.name == "dev") {
                    paths.push(EventPathType::Source(param.param_pretty.clone()));
                }
                if let Some(param) = event.event_params.iter().find(|p| p.name == "dir") {
                    paths.push(EventPathType::Target(param.param_pretty.clone()));
                }
            }
            _ => {}
        }
        paths
    }

    /// For `poll` syscalls (enter or exit), gets the list of FD numbers being polled. These can be used for various lookups,
    /// if we have info on the returned FDs.
    fn get_fdlist_from_poll_evt(event: &ZoneKernelSyscallEvent) -> Vec<u64> {
        let mut poll_fds = Vec::new();
        use event_codes::*;
        let etype = event_codes::from_repr(event.event_type)
            .ok_or(anyhow!("could not parse event type"))
            .expect("should parse");
        let fddata = match etype {
            PPME_SYSCALL_PPOLL_E => event.event_params[0].param_data.clone(),
            PPME_SYSCALL_PPOLL_X => event.event_params[1].param_data.clone(),
            _ => return poll_fds,
        };

        if fddata.len() < 2 {
            return poll_fds;
        }
        // Read number of file descriptors (first 2 bytes)
        let nfds = u16::from_ne_bytes(fddata[0..2].try_into().unwrap_or_default()) as usize;
        let mut pos = 2;

        for _ in 0..nfds {
            if pos + 10 > fddata.len() {
                break;
            }

            // Read fd number (8 bytes)
            let fd: u64 = i64::from_ne_bytes(fddata[pos..pos + 8].try_into().unwrap_or_default())
                .try_into()
                .unwrap_or_default();
            poll_fds.push(fd);

            // Move to next fd entry (10 bytes total: 8 for fd + 2 for flags)
            pos += 10;
        }
        poll_fds
    }

    fn get_fd_cip(fdinfo: &ZoneKernelFdInfo) -> Option<IpAddr> {
        let finfo = fdinfo.info.as_ref()?;
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                Some(IpAddr::V4(sock.source_ip.parse::<Ipv4Addr>().ok()?))
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                Some(IpAddr::V6(sock.source_ip.parse::<Ipv6Addr>().ok()?))
            }
            _ => None,
        }
    }

    fn get_fd_sip(fdinfo: &ZoneKernelFdInfo) -> Option<IpAddr> {
        let finfo = fdinfo.info.as_ref()?;
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => {
                Some(IpAddr::V4(sock.dest_ip.parse::<Ipv4Addr>().ok()?))
            }
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                Some(IpAddr::V4(sock.local_ip.parse::<Ipv4Addr>().ok()?))
            }
            Some(InfoType::Ipv6Socket(sock)) => {
                Some(IpAddr::V6(sock.dest_ip.parse::<Ipv6Addr>().ok()?))
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                Some(IpAddr::V6(sock.local_ip.parse::<Ipv6Addr>().ok()?))
            }
            _ => None,
        }
    }

    fn get_fd_lip(fdinfo: &ZoneKernelFdInfo) -> Option<IpAddr> {
        let finfo = fdinfo.info.as_ref()?;
        match &finfo.info_type {
            Some(InfoType::Ipv4ServerSocket(sock)) => {
                Some(IpAddr::V4(sock.local_ip.parse::<Ipv4Addr>().ok()?))
            }
            Some(InfoType::Ipv6ServerSocket(sock)) => {
                Some(IpAddr::V6(sock.local_ip.parse::<Ipv6Addr>().ok()?))
            }
            _ => None,
        }
    }

    fn get_fd_cport(fdinfo: &ZoneKernelFdInfo) -> Option<u32> {
        let finfo = fdinfo.info.as_ref()?;
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => Some(sock.source_port),
            Some(InfoType::Ipv6Socket(sock)) => Some(sock.source_port),
            _ => None,
        }
    }

    fn get_fd_sport(fdinfo: &ZoneKernelFdInfo) -> Option<u32> {
        let finfo = fdinfo.info.as_ref()?;
        match &finfo.info_type {
            Some(InfoType::Ipv4Socket(sock)) => Some(sock.dest_port),
            Some(InfoType::Ipv4ServerSocket(sock)) => Some(sock.local_port),
            Some(InfoType::Ipv6Socket(sock)) => Some(sock.dest_port),
            Some(InfoType::Ipv6ServerSocket(sock)) => Some(sock.local_port),
            _ => None,
        }
    }

    fn with_zone_syscall_evt_ctx<F, R>(&mut self, req: &mut ExtractRequest<Self>, f: F) -> R
    where
        F: FnOnce(&ZoneKernelSyscallEvent) -> R,
    {
        let context = self.get_or_cache_evt_ctx(req);

        f(&context.decoded_evt)
    }
}
