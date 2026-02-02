use anyhow::{Result, anyhow};
use libc::{
    AF_INET, AF_INET6, IPPROTO_ICMP, IPPROTO_IP, IPPROTO_RAW, IPPROTO_TCP, IPPROTO_UDP, SOCK_DGRAM,
    SOCK_RAW, SOCK_STREAM,
};

use crate::proto::generated::protect::control::v1::{
    ZoneKernelEventParam, ZoneKernelFdInfo, ZoneKernelFdInfoData, ZoneKernelIpv4SocketInfo,
    ZoneKernelIpv6SocketInfo, ZoneKernelPidFd, ZoneKernelRegularFileInfo, ZoneKernelSyscallEvent,
    ZoneKernelThreadInfo, ZoneKernelThreadSnapshotEvent, ZoneKernelUnixSocketInfo,
    zone_kernel_fd_info_data::InfoType,
};
use libscap_bindings::{
    consts as ppm_consts,
    types::{
        ppm_event_code as event_codes, ppm_param_type as param_type, scap_fd_type as fd_types,
        scap_l4_proto as l4_types,
    },
};
use log::{debug, error, trace, warn};
use std::collections::HashMap;
use std::ffi::CStr;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::time::Duration;
use strum_macros::Display;

use crate::parsers;

// a handful of libsinsp/libscap consts
// duplicated here
const SE_EINPROGRESS: i64 = 115;
const FLAGS_ROLE_CLIENT: u32 = 1 << 2;
const FLAGS_ROLE_SERVER: u32 = 1 << 3;
const FLAGS_SOCKET_CONNECTED: u32 = 1 << 13;
const FLAGS_IS_CLONED: u32 = 1 << 14;
const FLAGS_CONNECTION_PENDING: u32 = 1 << 15;
const FLAGS_CONNECTION_FAILED: u32 = 1 << 16;
const FLAGS_OVERLAY_UPPER: u32 = 1 << 17;
const FLAGS_OVERLAY_LOWER: u32 = 1 << 18;
const FILE_DESCRIPTOR_PARAM_ID: usize = 2;
const SOCKET_TUPLE_PARAM_ID: usize = 4;
const PPM_CL_CLONE_NEWUSER: u32 = 1 << 20;

#[derive(Debug, Display)]
pub enum ProcessingError {
    ZoneCPUHotplug(String),
}

impl std::error::Error for ProcessingError {}

#[derive(Default)]
pub struct ThreadState {
    zone_info: HashMap<String, ZoneInfo>,
}

#[derive(Default, Clone)]
pub struct ExecTime {
    pub last_switch_ts: u64,
    pub previous_switch_ts: u64,
    pub cumulative_switch_time: u64,
}

#[derive(Default)]
pub struct ZoneInfo {
    seen_threadsnaps: HashMap<u64, ZoneKernelThreadInfo>,
    // TODO(bml) consider dropping this
    dropped_fds_by_thread: HashMap<u64, u64>,
    last_proc_switch_times_by_cpuid: HashMap<u32, ExecTime>,
    boot_epoch: u64,
}

impl ZoneInfo {
    pub fn get_threadinfo(&self, thread_id: &u64) -> Option<&ZoneKernelThreadInfo> {
        self.seen_threadsnaps.get(thread_id)
    }

    /// Get FD info for an event.
    ///
    /// Modern BPF driver only captures exit events, so this extracts FD information
    /// from event parameters and looks up the corresponding FD info.
    ///
    /// Returns None if the event doesn't use an FD or if the FD info is not found.
    pub fn get_event_fdinfo(&self, event: &ZoneKernelSyscallEvent) -> Option<ZoneKernelFdInfo> {
        // Only exit events are captured by modern_bpf driver
        if parsers::has_fd(event) {
            parsers::get_fdi(event).and_then(|fdi| {
                self.with_leader_fdlist_ctx(event, |fdlist| fdlist.get(&fdi).cloned())
            })
        } else {
            None
        }
    }

    pub fn get_thread_groupsize(&self, thread_id: &u64, exclude_leader: bool) -> u64 {
        self.seen_threadsnaps
            .get(thread_id)
            .and_then(|tinfo| tinfo.pid)
            .map(|tgroup_pid| {
                self.seen_threadsnaps
                    .values()
                    .filter(|t| {
                        t.pid.is_some_and(|tpid| tpid == tgroup_pid)
                            && !(parsers::thread_main(t) && exclude_leader)
                    })
                    .count() as u64
            })
            .unwrap_or(0)
    }

    pub fn get_exectime(&self, cpuid: &u32) -> Option<ExecTime> {
        self.last_proc_switch_times_by_cpuid.get(cpuid).cloned()
    }

    pub fn get_leader_tid(tinfo: &ZoneKernelThreadInfo) -> Option<u64> {
        tinfo.pid.or(tinfo.tid)
    }

    fn get_fd(&self, tinfo: &ZoneKernelThreadInfo, fd: &u64) -> Option<&ZoneKernelFdInfo> {
        let owner_id = Self::get_leader_tid(tinfo)?;
        self.seen_threadsnaps
            .get(&owner_id)
            .and_then(|owner| owner.fdlist.get(fd))
    }

    /// libsinsp always stores all FDs with the parent/main thread,
    /// so child threads do not have their own FD list. In fact they
    /// go so far as to spuriously mark all threads as CLONE_FILES
    /// even if it wasn't set by Linux, to guarantee this view.
    ///
    /// That's a little wonky but 99% of the time it's correct,
    /// and for tracking purposes in the rules engine it never makes a difference.
    /// And it makes for cheaper fd lookups.
    ///
    /// So that is why *ALL* event parser funcs in this class should add/remove to the leader
    /// thread's FD list, and that's what these helpers are for.
    fn with_mut_leader_fdinfo_ctx<F, R>(
        &mut self,
        tinfo: &ZoneKernelThreadInfo,
        fd: &u64,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&mut ZoneKernelFdInfo) -> Option<R>,
    {
        let owner_id = Self::get_leader_tid(tinfo)?;
        self.seen_threadsnaps
            .get_mut(&owner_id)
            .and_then(|owner| owner.fdlist.get_mut(fd))
            .and_then(f)
    }

    fn with_leader_fdlist_ctx<F, R>(&self, event: &ZoneKernelSyscallEvent, f: F) -> Option<R>
    where
        F: FnOnce(&HashMap<u64, ZoneKernelFdInfo>) -> Option<R>,
    {
        if let Some(owner_id) = self
            .seen_threadsnaps
            .get(&event.thread_id)
            .and_then(Self::get_leader_tid)
        {
            self.seen_threadsnaps
                .get(&owner_id)
                .and_then(|owner| f(&owner.fdlist))
        } else {
            None
        }
    }

    fn with_mut_leader_fdlist_ctx<F, R>(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&mut HashMap<u64, ZoneKernelFdInfo>) -> Option<R>,
    {
        if let Some(owner_id) = self
            .seen_threadsnaps
            .get_mut(&event.thread_id)
            .and_then(|tinfo| Self::get_leader_tid(tinfo))
        {
            self.seen_threadsnaps
                .get_mut(&owner_id)
                .and_then(|owner| f(&mut owner.fdlist))
        } else {
            None
        }
    }

    fn with_mut_leader_ctx<F, R>(&mut self, event: &ZoneKernelSyscallEvent, f: F) -> Option<R>
    where
        F: FnOnce(&mut ZoneKernelThreadInfo) -> Option<R>,
    {
        if let Some(owner_id) = self
            .seen_threadsnaps
            .get_mut(&event.thread_id)
            .and_then(|tinfo| Self::get_leader_tid(tinfo))
        {
            self.seen_threadsnaps.get_mut(&owner_id).and_then(f)
        } else {
            None
        }
    }

    /// Returns `None` if no threadinfo exists in the zone thread table for the threadid combo
    /// specified by the event. Otherwise, runs a FnOnce over a mutable reference to the threadinfo
    fn with_mut_threadinfo_ctx<F, R>(&mut self, event: &ZoneKernelSyscallEvent, f: F) -> Option<R>
    where
        F: FnOnce(&mut ZoneKernelThreadInfo) -> Option<R>,
    {
        self.seen_threadsnaps.get_mut(&event.thread_id).and_then(f)
    }

    /// Returns `None` if no threadinfo exists in the zone thread table for the threadid
    /// specified by the event. Otherwise, runs a FnOnce over an immutable reference to the threadinfo
    fn with_threadinfo_ctx<F, R>(&self, event: &ZoneKernelSyscallEvent, f: F) -> Option<R>
    where
        F: FnOnce(&ZoneKernelThreadInfo) -> Option<R>,
    {
        self.seen_threadsnaps.get(&event.thread_id).and_then(f)
    }

    /// Runs a FnOnce over a mutable reference to the zone thread table,
    /// if a thread table exists for that zone.
    fn with_mut_threadsnaps_ctx<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut HashMap<u64, ZoneKernelThreadInfo>) -> Option<R>,
    {
        f(&mut self.seen_threadsnaps)
    }

    fn parse_rw_exit(&mut self, event: &ZoneKernelSyscallEvent, etype: event_codes) -> Result<()> {
        use event_codes::*;
        if (etype == PPME_SOCKET_SENDMMSG_X || etype == PPME_SOCKET_RECVMMSG_X)
            && event.event_params.is_empty()
        {
            return Ok(());
        }

        let tinfo = self.seen_threadsnaps.get(&event.thread_id).cloned();

        if (etype == PPME_SOCKET_SEND_X
            || etype == PPME_SOCKET_SENDTO_X
            || etype == PPME_SOCKET_SENDMSG_X)
            && (tinfo.is_some() && self.maybe_get_event_fdinfo(event).is_none())
        {
            trace!("inferring socket for event: {:?}", event);
            self.infer_sendto_sendmsg_fd(event)
        }

        // we should have inferred the FD in the previous step. If not, give up.
        if self.maybe_get_event_fdinfo(event).is_none() {
            return Ok(());
        }

        let retval = parsers::get_retval(event);

        let Some(fdi) = parsers::get_fdi(event) else {
            warn!("this event reads fds but could not get fdi");
            return Ok(());
        };

        let Some(tinfo) = tinfo else {
            warn!("cannot find tinfo rw_exit");
            return Ok(());
        };

        if retval.is_some_and(|v| v >= 0) {
            if parsers::reads_fd(event) {
                // update the state flags of the FD
                let update_fd_tuple = self.with_mut_leader_fdinfo_ctx(&tinfo, &fdi, |mut_fdinfo| {
                    if mut_fdinfo.fd_type == fd_types::SCAP_FD_IPV4_SOCK as i32
                        || mut_fdinfo.fd_type == fd_types::SCAP_FD_IPV6_SOCK as i32
                    {
                        mut_fdinfo.state_flags &=
                            !(FLAGS_CONNECTION_PENDING | FLAGS_CONNECTION_FAILED);
                        mut_fdinfo.state_flags |= FLAGS_SOCKET_CONNECTED;
                    }

                    let tupleparam = if etype == PPME_SOCKET_RECVFROM_X {
                        2
                    } else if etype == PPME_SOCKET_RECVMSG_X {
                        3
                    } else if etype == PPME_SOCKET_RECVMMSG_X || etype == PPME_SOCKET_RECV_X {
                        4
                    } else {
                        0
                    };

                    if tupleparam > 0
                        && (mut_fdinfo.info.is_none() || !parsers::is_tcp_socket(mut_fdinfo))
                    {
                        // recvfrom contains tuple info.
                        // If the fd still doesn't contain tuple info (because the socket is a
                        // datagram one or because some event was lost),
                        // add it here.
                        return Some(Self::update_sockfd_from_tuple(
                            mut_fdinfo,
                            &event.event_params[tupleparam as usize],
                        ));
                    }
                    None
                });

                let updated_fdinfo = self
                    .maybe_get_event_fdinfo(event)
                    .expect("fdinfo must exist");

                let updated_fdinfo_type = updated_fdinfo.fd_type;

                if update_fd_tuple.is_some_and(|val| val)
                    && (updated_fdinfo_type == fd_types::SCAP_FD_IPV4_SOCK as i32
                        || updated_fdinfo_type == fd_types::SCAP_FD_IPV6_SOCK as i32)
                {
                    // if we rewrote the fdinfo, then determine the role and update
                    // easier to do this outside the mut ref closure
                    if parsers::is_role_none(updated_fdinfo) {
                        // default to server port
                        let mut is_server = true;
                        if let Some((sport, dport)) =
                            updated_fdinfo
                                .info
                                .as_ref()
                                .and_then(|info| match &info.info_type {
                                    Some(InfoType::Ipv4Socket(s)) => {
                                        Some((s.source_port, s.dest_port))
                                    }
                                    _ => None,
                                })
                        {
                            // try to infer
                            is_server = self
                                .with_threadinfo_ctx(event, |tinfo| {
                                    Some(parsers::is_server_port(tinfo, sport, dport, true))
                                })
                                .expect("threadinfo must be there");
                        }

                        self.with_mut_leader_fdinfo_ctx(&tinfo, &fdi, |mut_fdinfo| {
                            if is_server {
                                Self::set_role_server(mut_fdinfo);
                            } else {
                                Self::set_role_client(mut_fdinfo);
                                Self::swap_addresses(mut_fdinfo);
                            }
                            Some(())
                        });
                    }
                }

                // Since you can use Unix sockets to pass FDs between processes,
                // libsinsp here tries to parse out the SCM_RIGHTS info from
                // RECVMSG to extract any FDs there.
                //
                // I'm TODOing this because the current libsinsp approach
                // always does a hard `/proc` host query for these and doesn't rely
                // on the thread/fd table, for Reasons (it's tricky).
                //
                // Passing FDs around with unix sockets is something of a corner case tho,
                // so for now skipping.
                if updated_fdinfo_type == fd_types::SCAP_FD_UNIX_SOCK as i32 {
                    debug!("TODO unix socket SCM_RIGHTS FD parsing");
                }
            } else {
                //does not read from FD
                if etype == PPME_SOCKET_SEND_X
                    || etype == PPME_SOCKET_SENDTO_X
                    || etype == PPME_SOCKET_SENDMSG_X
                    || etype == PPME_SOCKET_SENDMMSG_X
                {
                    // update the state flags of the FD
                    let update_fd_tuple =
                        self.with_mut_leader_fdinfo_ctx(&tinfo, &fdi, |mut_fdinfo| {
                            if mut_fdinfo.info.is_none() || !parsers::is_tcp_socket(mut_fdinfo) {
                                // send, sendto, sendmsg and sendmmsg contain tuple info in the exit event.
                                // If the fd still doesn't contain tuple info (because the socket is a datagram one
                                // or because some event was lost), add it here.
                                let tupleparam = 4;
                                return Some(Self::update_sockfd_from_tuple(
                                    mut_fdinfo,
                                    &event.event_params[tupleparam as usize],
                                ));
                            }

                            None
                        });

                    let updated_fdinfo = self
                        .maybe_get_event_fdinfo(event)
                        .expect("fdinfo must exist");

                    let updated_fdinfo_type = updated_fdinfo.fd_type;

                    if update_fd_tuple.is_some_and(|val| val)
                        && (updated_fdinfo_type == fd_types::SCAP_FD_IPV4_SOCK as i32
                            || updated_fdinfo_type == fd_types::SCAP_FD_IPV6_SOCK as i32)
                    {
                        // if we rewrote the fdinfo, then determine the role and update
                        // easier to do this outside the mut ref closure
                        if parsers::is_role_none(updated_fdinfo) {
                            // default to client port
                            let mut is_server = false;
                            if let Some((sport, dport)) =
                                updated_fdinfo.info.as_ref().and_then(|info| {
                                    match &info.info_type {
                                        Some(InfoType::Ipv4Socket(s)) => {
                                            Some((s.source_port, s.dest_port))
                                        }
                                        _ => None,
                                    }
                                })
                            {
                                // try to infer
                                is_server = self
                                    .with_threadinfo_ctx(event, |tinfo| {
                                        Some(parsers::is_server_port(tinfo, sport, dport, false))
                                    })
                                    .expect("threadinfo must be there");
                            }

                            self.with_mut_leader_fdinfo_ctx(&tinfo, &fdi, |mut_fdinfo| {
                                if is_server {
                                    Self::set_role_server(mut_fdinfo);
                                    Self::swap_addresses(mut_fdinfo);
                                } else {
                                    Self::set_role_client(mut_fdinfo);
                                }
                                Some(())
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// returns a reference to the ZoneKernelFdInfo in the table, for this event's specified thread,
    /// and it's specified FD number.
    ///
    /// Not all events carry an FD number (see parsers::has_fd) and we may not have an entry for this one in the table.
    /// In both of those cases, this will return None
    fn maybe_get_event_fdinfo(&self, event: &ZoneKernelSyscallEvent) -> Option<&ZoneKernelFdInfo> {
        let tinfo = self.seen_threadsnaps.get(&event.thread_id);

        if let Some(fdi) = parsers::get_fdi(event)
            && let Some(thread_info) = tinfo
        {
            return self.get_fd(thread_info, &fdi);
        };

        None
    }

    /// If we receive a call to 'send()/sendto()/sendmsg()' and the event's m_fdinfo is nullptr,
    /// then we likely missed the call to 'socket()' that created the file
    /// descriptor.  In that case, we'll guess that it's a SOCK_DGRAM/UDP socket
    /// and create the fdinfo based on that.
    ///
    /// Precondition: we have a thread entry for the event's thread,
    /// but no fd entry for the fd in that thread entry.
    fn infer_sendto_sendmsg_fd(&mut self, event: &ZoneKernelSyscallEvent) {
        if let Some(retval) = parsers::get_retval(event)
            && retval < 0
        {
            //syscall failed, can't trust.
            return;
        }

        if event.event_params[FILE_DESCRIPTOR_PARAM_ID].param_type != param_type::PT_FD as u32 {
            warn!("unexpected param type for sendto/sendmsg exit, cannot infer FD");
        }

        let fd: Option<u64> = event.event_params[FILE_DESCRIPTOR_PARAM_ID]
            .param_data
            .as_slice()
            .try_into()
            .ok()
            .map(i64::from_ne_bytes)
            .and_then(|i| i.try_into().ok());

        let family = i32::from_ne_bytes(
            event.event_params[SOCKET_TUPLE_PARAM_ID]
                .param_data
                .as_slice()
                .try_into()
                .unwrap_or_default(),
        );

        let domain = if family == AF_INET {
            ppm_consts::PPM_AF_INET
        } else if family == AF_INET6 {
            ppm_consts::PPM_AF_INET6
        } else {
            debug!("not an inet family for sendto/sendmsg socket type, ignoring");
            ppm_consts::PPM_AF_UNSPEC
        };

        // libsinsp: "Here we're assuming send*() means SOCK_DGRAM/UDP, but it
        // can be used with TCP.  We have no way to know for sure at
        // this point."
        self.add_socket_fd(event, fd, domain, SOCK_DGRAM, IPPROTO_UDP)
    }

    fn add_socket_fd(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        fdi: Option<u64>,
        domain: u32,
        s_type: i32,
        protocol: i32,
    ) {
        use ppm_consts::*;

        if fdi.is_none() {
            error!("cannot add socket, invalid fd");
            return;
        }

        let is_v4 = domain == PPM_AF_INET;
        let mut fdinfo = ZoneKernelFdInfo {
            fd: fdi,
            ..Default::default()
        };

        match domain {
            PPM_AF_UNIX => {
                fdinfo.fd_type = fd_types::SCAP_FD_UNIX_SOCK as i32;
            }
            PPM_AF_INET | PPM_AF_INET6 => {
                fdinfo.fd_type = if is_v4 {
                    fd_types::SCAP_FD_IPV4_SOCK as i32
                } else {
                    fd_types::SCAP_FD_IPV6_SOCK as i32
                };

                let ip_proto = match protocol {
                    IPPROTO_TCP => {
                        if s_type == SOCK_RAW {
                            l4_types::SCAP_L4_RAW
                        } else {
                            l4_types::SCAP_L4_TCP
                        }
                    }
                    IPPROTO_UDP => {
                        if s_type == SOCK_RAW {
                            l4_types::SCAP_L4_RAW
                        } else {
                            l4_types::SCAP_L4_UDP
                        }
                    }
                    IPPROTO_IP => {
                        if (s_type & 0xff) == SOCK_STREAM {
                            l4_types::SCAP_L4_TCP
                        } else if (s_type & 0xff) == SOCK_DGRAM {
                            l4_types::SCAP_L4_UDP
                        } else {
                            warn!("unexpected socket protocol type");
                            l4_types::SCAP_L4_UNKNOWN
                        }
                    }
                    IPPROTO_ICMP => {
                        if s_type == SOCK_RAW {
                            l4_types::SCAP_L4_RAW
                        } else {
                            l4_types::SCAP_L4_ICMP
                        }
                    }
                    IPPROTO_RAW => l4_types::SCAP_L4_RAW,
                    _ => l4_types::SCAP_L4_UNKNOWN,
                };

                if is_v4 {
                    let sock_info = ZoneKernelIpv4SocketInfo {
                        protocol: ip_proto as i32,
                        ..Default::default()
                    };
                    fdinfo.info = Some(ZoneKernelFdInfoData {
                        info_type: Some(InfoType::Ipv4Socket(sock_info)),
                    })
                } else {
                    let sock_info = ZoneKernelIpv6SocketInfo {
                        protocol: ip_proto as i32,
                        ..Default::default()
                    };
                    fdinfo.info = Some(ZoneKernelFdInfoData {
                        info_type: Some(InfoType::Ipv6Socket(sock_info)),
                    })
                }
            }
            PPM_AF_NETLINK => {
                fdinfo.fd_type = fd_types::SCAP_FD_NETLINK as i32;
            }
            _ => {
                fdinfo.fd_type = fd_types::SCAP_FD_UNKNOWN as i32;
                debug!(
                    "unknown FD type domain: {:?} protocol: {:?}",
                    domain, protocol
                )
            }
        }

        self.with_mut_leader_fdlist_ctx(event, |fdlist| {
            fdlist.insert(fdi.expect("previously checked"), fdinfo)
        });
    }

    /// Parses the exit events of OPEN-type syscalls, collecting new FD info, and updating the FD table for a given
    /// thread in the thread table when a new file descriptor is obtained by that thread.
    fn parse_create_open_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        etype: event_codes,
    ) -> Result<()> {
        use event_codes::*;

        let Some(tinfo) = self.seen_threadsnaps.get(&event.thread_id) else {
            return Ok(());
        };

        // With modern_bpf (exit-only events), there are no enter events to retrieve.
        // All data comes from the exit event parameters.
        let fd = parsers::get_retval(event);

        let exit_name: String;
        let mut exit_flags: u32;
        let exit_dirfd: i64;
        let exit_sdir: Option<PathBuf>;
        let mut exit_dev: Option<u32> = None;
        let mut exit_ino: Option<u64> = None;

        match etype {
            PPME_SYSCALL_OPEN_X => {
                exit_name = event.event_params[1].param_pretty.clone();
                exit_flags =
                    u32::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);
                if event.event_params.len() > 4 {
                    exit_dev = Some(u32::from_ne_bytes(
                        event.event_params[4].param_data.as_slice().try_into()?,
                    ));
                    if event.event_params.len() > 5 {
                        exit_ino = Some(u64::from_ne_bytes(
                            event.event_params[5].param_data.as_slice().try_into()?,
                        ));
                    }
                }

                // Note: Old libsinsp code would override with enter event data for TOCTOU mitigation.
                // With modern_bpf (exit-only events), we just use the exit event data directly.

                exit_sdir = Some(PathBuf::from(&tinfo.cwd));
            }
            PPME_SYSCALL_CREAT_X => {
                exit_name = event.event_params[1].param_pretty.clone();
                exit_flags = 0;

                if event.event_params.len() > 3 {
                    exit_dev = Some(u32::from_ne_bytes(
                        event.event_params[3].param_data.as_slice().try_into()?,
                    ));
                    if event.event_params.len() > 4 {
                        exit_ino = Some(u64::from_ne_bytes(
                            event.event_params[4].param_data.as_slice().try_into()?,
                        ));
                        if event.event_params.len() > 5 {
                            let creat_flags = u32::from_ne_bytes(
                                event.event_params[5].param_data.as_slice().try_into()?,
                            );
                            if (creat_flags & ppm_consts::PPM_FD_UPPER_LAYER_CREAT) != 0 {
                                exit_flags |= ppm_consts::PPM_FD_UPPER_LAYER;
                            } else if (creat_flags & ppm_consts::PPM_FD_LOWER_LAYER_CREAT) != 0 {
                                exit_flags |= ppm_consts::PPM_FD_LOWER_LAYER;
                            }
                        }
                    }
                }

                // Note: Old libsinsp code would override with enter event data for TOCTOU mitigation.
                // With modern_bpf (exit-only events), we just use the exit event data directly.

                exit_sdir = Some(PathBuf::from(&tinfo.cwd));
            }
            PPME_SYSCALL_OPENAT_X => {
                // This is the old OPENAT format that required enter event data.
                // Modern_bpf generates PPME_SYSCALL_OPENAT_2_X instead (see openat.bpf.c line 20).
                // If we somehow receive this event type, we can't process it without enter events.
                warn!(
                    "Received unsupported PPME_SYSCALL_OPENAT_X event (requires enter event data)"
                );
                return Ok(());
            }
            PPME_SYSCALL_OPENAT_2_X | PPME_SYSCALL_OPENAT2_X => {
                exit_name = event.event_params[2].param_pretty.clone();
                exit_flags =
                    u32::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?);
                exit_dirfd =
                    i64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);

                if etype == PPME_SYSCALL_OPENAT_2_X && event.event_params.len() > 5 {
                    exit_dev = Some(u32::from_ne_bytes(
                        event.event_params[5].param_data.as_slice().try_into()?,
                    ));
                    exit_ino = if event.event_params.len() > 6 {
                        Some(u64::from_ne_bytes(
                            event.event_params[6].param_data.as_slice().try_into()?,
                        ))
                    } else {
                        None
                    };
                } else if etype == PPME_SYSCALL_OPENAT2_X && event.event_params.len() > 6 {
                    exit_dev = Some(u32::from_ne_bytes(
                        event.event_params[6].param_data.as_slice().try_into()?,
                    ));
                    exit_ino = if event.event_params.len() > 7 {
                        Some(u64::from_ne_bytes(
                            event.event_params[7].param_data.as_slice().try_into()?,
                        ))
                    } else {
                        None
                    };
                }

                // Note: Old libsinsp code would override with enter event data for TOCTOU mitigation.
                // With modern_bpf (exit-only events), we just use the exit event data directly.

                exit_sdir = self.parse_dirfd(event, &exit_name, exit_dirfd);
            }
            PPME_SYSCALL_OPEN_BY_HANDLE_AT_X => {
                exit_flags =
                    u32::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);
                exit_name = event.event_params[3].param_pretty.clone();
                // The driver implementation always serves an absolute path for open_by_handle_at using
                // dpath traversal; hence there is no need to interpret the path relative to mountfd.
                exit_sdir = None;

                if event.event_params.len() > 4 {
                    exit_dev = Some(u32::from_ne_bytes(
                        event.event_params[4].param_data.as_slice().try_into()?,
                    ));
                    exit_ino = if event.event_params.len() > 5 {
                        Some(u64::from_ne_bytes(
                            event.event_params[5].param_data.as_slice().try_into()?,
                        ))
                    } else {
                        None
                    };
                }

                // The driver implementation always serves an absolute path for open_by_handle_at using
                // dpath traversal; hence there is no need to interpret the path relative to mountfd.
            }
            _ => {
                debug!("skipped open exit event: {:?}", event);
                return Ok(());
            }
        }

        let fullpath = if let Some(path) = exit_sdir {
            // join will handle cases where `exit_name` is already an abs path.
            path.join(exit_name)
        } else {
            PathBuf::from(exit_name)
        };

        if let Some(fd_) = fd
            && fd_ >= 0
        {
            let valid_fd = fd_ as u64; // we just checked
            let fullpath_str = fullpath.to_string_lossy().to_string();

            let fdinfo = ZoneKernelFdInfo {
                fd: Some(valid_fd),
                inode: exit_ino.unwrap_or(0),
                fd_type: if exit_flags & ppm_consts::PPM_O_DIRECTORY != 0 {
                    fd_types::SCAP_FD_DIRECTORY as i32
                } else {
                    fd_types::SCAP_FD_FILE_V2 as i32
                },
                info: Some(ZoneKernelFdInfoData {
                    info_type: Some(InfoType::RegularFile(ZoneKernelRegularFileInfo {
                        open_flags: exit_flags,
                        mount_id: 0,
                        device: exit_dev.unwrap_or(0),
                        name: fullpath_str,
                    })),
                }),
                open_flags: exit_flags,
                state_flags: {
                    let mut flags = 0;
                    if exit_flags & ppm_consts::PPM_FD_UPPER_LAYER != 0 {
                        flags |= FLAGS_OVERLAY_UPPER;
                    }
                    if exit_flags & ppm_consts::PPM_FD_LOWER_LAYER != 0 {
                        flags |= FLAGS_OVERLAY_LOWER;
                    }
                    flags
                },
            };

            self.with_mut_leader_fdlist_ctx(event, |fdlist| {
                fdlist.insert(valid_fd, fdinfo);
                Some(())
            });
        }

        Ok(())
    }

    /// These syscalls act on FDs we should have already seen,
    /// so this is largely a no-op for us.
    /// TODO(bml) test, but I think we can skip this.
    fn parse_fchmod_fchown_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        _etype: event_codes,
    ) -> Result<()> {
        let tinfo = self.seen_threadsnaps.get(&event.thread_id);
        // if no thread info, bail
        if tinfo.is_none() {
            debug!("no thread found for fchmod/fchown");
            return Ok(());
        }

        let param_idx = 1;

        if event.event_params[param_idx].param_type != param_type::PT_FD as u32 {
            warn!("unexpected param type for fchmod/fchown exit, cannot infer FD");
        }

        Ok(())
    }

    /// This function was used in old libsinsp to retrieve enter event data.
    /// With modern_bpf (exit-only events), this is a no-op.
    fn parse_fspath_related_exit(
        &mut self,
        _event: &ZoneKernelSyscallEvent,
        _etype: event_codes,
    ) -> Result<()> {
        Ok(())
    }

    fn parse_unshare_setns_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        etype: event_codes,
    ) -> Result<()> {
        use event_codes::*;
        let tinfo = self.seen_threadsnaps.get(&event.thread_id);
        // if no thread info, bail
        if tinfo.is_none() {
            debug!("no thread found for unshare/setns: {:?}", event);
            return Ok(());
        }

        if parsers::get_retval(event).is_some_and(|val| val < 0) {
            warn!("no retval found for unshare/setns: {:?}", event);
            return Ok(());
        }

        let flags = match etype {
            PPME_SYSCALL_UNSHARE_X => {
                u32::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?)
            }
            PPME_SYSCALL_SETNS_X => {
                u32::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?)
            }
            _ => 0,
        };

        if flags & PPM_CL_CLONE_NEWUSER == 0 {
            return Ok(());
        }

        // on unshare, threads start with "all caps"
        self.with_mut_threadinfo_ctx(event, |mut_tinfo| {
            let max_caps = parsers::get_all_caps_bitmask();
            mut_tinfo.cap_permitted = max_caps;
            mut_tinfo.cap_effective = max_caps;
            mut_tinfo.cap_inheritable = max_caps;
            Some(())
        });

        Ok(())
    }

    fn parse_memfd_create_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            // if no thread info, bail
            debug!("no thread found for memfd: {:?}", event);
            return Ok(());
        };

        let Some(fdi) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no retval found for memfd: {:?}", event);
            return Ok(());
        };

        let name = CStr::from_bytes_until_nul(&event.event_params[1].param_data)?.to_str()?;
        let flags = u32::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);

        let fdinfo = ZoneKernelFdInfo {
            fd: Some(fdi as u64),
            fd_type: fd_types::SCAP_FD_MEMFD as i32,
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::FileName(name.to_string())),
            }),
            open_flags: flags,
            ..Default::default()
        };

        self.with_mut_leader_fdlist_ctx(event, |fdlist| fdlist.insert(fdi as u64, fdinfo));
        Ok(())
    }

    fn parse_clone_fork_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        let Some(childtid) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            debug!("clone() failed, nothing to do: {:?}", event);
            return Ok(());
        };

        // libsinsp: "Please note that if the child is in a namespace different from the init one
        // we should never use this `childtid` otherwise we will use a thread id referred to
        // an internal namespace and not to the init one!"
        if childtid == 0 {
            self.parse_clone_fork_exit_child(event)
        } else {
            self.parse_clone_fork_exit_caller(event, childtid as u64)
        }
    }

    fn parse_clone_fork_exit_child(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        let child_tid = event.thread_id;

        let tinfo = self.seen_threadsnaps.get(&child_tid);

        // First, see if we already have an entry in the thread table. If there is, and it's not recent,
        // assume it's an old/stale entry we missed the exit event for and drop it.
        if let Some(tinfo) = tinfo.filter(|thread| thread.clone_ts != 0) {
            let event_ts: u128 = event.timestamp.into();
            let thread_clone_ts: u128 = tinfo.clone_ts.into();

            if event_ts - thread_clone_ts < Duration::new(2, 0).as_nanos() {
                //thread is valid and already in the table, nothing to do.
                // libsinsp: "Note that if we are in a container the caller
                // will never generate the child thread-info because it doesn't have
                // enough info. In all other cases the thread info created by the caller
                // should be already valid."
                return Ok(());
            } else {
                // drop the stale thread, recreate below
                //
                // TODO(bml) need to handle thread reapers/reparenting!
                self.with_mut_threadsnaps_ctx(|threads| threads.remove(&child_tid));
            }
        };

        let mut child_tinfo = ZoneKernelThreadInfo {
            lastexec_ts: 0,
            tid: Some(child_tid),
            pid: Some(
                i64::from_ne_bytes(event.event_params[4].param_data.as_slice().try_into()?)
                    .try_into()?,
            ),
            ptid: Some(
                i64::from_ne_bytes(event.event_params[5].param_data.as_slice().try_into()?)
                    .try_into()?,
            ),
            flags: u32::from_ne_bytes(event.event_params[15].param_data.as_slice().try_into()?),
            ..Default::default()
        };

        // if one of these params is present, it is guaranteed that the other will be too.
        if event.event_params.len() >= 19 && !event.event_params[18].param_data.is_empty() {
            child_tinfo.vtid = Some(
                i64::from_ne_bytes(event.event_params[18].param_data.as_slice().try_into()?)
                    .try_into()?,
            );
            child_tinfo.vpid = Some(
                i64::from_ne_bytes(event.event_params[19].param_data.as_slice().try_into()?)
                    .try_into()?,
            );
        } else {
            child_tinfo.vtid = Some(child_tid);
            child_tinfo.vpid = None;
        }

        // libsinsp: "We add this custom `PPM_CL_CLONE_INVERTED` flag.
        // It means that we received the child event before the caller one and
        // it will notify the caller that it has to do nothing because we already
        // populated the thread info in the child."
        child_tinfo.flags |= ppm_consts::PPM_CL_CLONE_INVERTED;

        // Determine if we are a new thread in a leader group (in which case, copy info from the leader thread)
        // or a new leader thread (in which case, copy info from the parent thread)
        // libsinsp: "Note that the lookup thread could be different from the caller one!
        // If they are different we cannot completely trust the info we obtain from lookup thread
        // becuase they could be stale! For example the caller may have called `prctl` changing its
        // comm, while the lookup thread still have the old `comm`."
        let is_thread_leader = (child_tinfo.flags & ppm_consts::PPM_CL_CLONE_THREAD) == 0;

        let lookup_tid = if is_thread_leader {
            child_tinfo.ptid
        } else {
            // TODO(bml) libsinsp does
            // child_tinfo->m_flags |= PPM_CL_CLONE_FILES;
            // here, but that seems incorrect/we may not need that.
            // in linux, the thread leader's pid always equals the thread leader's tid,
            // so we can lookup the thread via the pid.
            child_tinfo.pid
        };

        Self::populate_command_info_from_event(&mut child_tinfo, event)?;

        let Some(l_tid) = lookup_tid else {
            debug!("no parent tid found for child");
            return Ok(());
        };
        let mut lookup_tinfo = self.seen_threadsnaps.get_mut(&l_tid);

        // if for some reason we inserted an empty tinfo for this tid/it's invalid,
        // we can reconstruct some of the threadinfo here, if this isn't a leader thread
        let valid_lookup = if let Some(l_tinfo) = lookup_tinfo
            .as_mut()
            .filter(|thread| thread.tid != lookup_tid)
        {
            if !is_thread_leader {
                l_tinfo.tid = lookup_tid;
                l_tinfo.pid = child_tinfo.pid;
                l_tinfo.ptid = child_tinfo.ptid;
                l_tinfo.vpid = child_tinfo.vpid;
                l_tinfo.vtid = child_tinfo.vtid;
                l_tinfo.exe = child_tinfo.exe.clone();
                l_tinfo.comm = child_tinfo.comm.clone();
                l_tinfo.args = child_tinfo.args.clone();
            }

            // TODO(bml) libsinsp does a hard procscan here to fill out the rest.
            // Skipping that for now.
            false
        } else {
            true
        };

        // libsinsp:
        // "Please note that these data could be wrong if the lookup thread
        // is not the caller! for example, if the child is created by a thread
        // the thread could have different info with respect to the thread leader,
        // for example `comm` could be different! This is a sort of best effort
        // enrichment..."
        if let Some(l_tinfo) = lookup_tinfo.as_ref()
            && valid_lookup
        {
            child_tinfo.exepath = l_tinfo.exepath.clone();
            child_tinfo.exe_writable = l_tinfo.exe_writable;
            child_tinfo.exe_upper_layer = l_tinfo.exe_upper_layer;
            child_tinfo.exe_lower_layer = l_tinfo.exe_lower_layer;
            child_tinfo.exe_from_memfd = l_tinfo.exe_from_memfd;
            child_tinfo.root = l_tinfo.root.clone();
            child_tinfo.sid = l_tinfo.sid;
            child_tinfo.vpgid = l_tinfo.vpgid;
            child_tinfo.pgid = l_tinfo.pgid;
            child_tinfo.tty = l_tinfo.tty;
            child_tinfo.loginuid = l_tinfo.loginuid;
            child_tinfo.cap_permitted = l_tinfo.cap_permitted;
            child_tinfo.cap_inheritable = l_tinfo.cap_inheritable;
            child_tinfo.cap_effective = l_tinfo.cap_effective;
            child_tinfo.exe_ino = l_tinfo.exe_ino;
            child_tinfo.exe_ino_ctime = l_tinfo.exe_ino_ctime;
            child_tinfo.exe_ino_mtime = l_tinfo.exe_ino_mtime;
            child_tinfo.exe_ino_ctime_duration_clone_ts = l_tinfo.exe_ino_ctime_duration_clone_ts;

            if is_thread_leader {
                // from libsinsp:

                // "We populate fdtable, cwd and env only if we are
                // a new leader thread, all not leader threads will use the same information
                // of the main thread.
                //
                // Copy the fd list:
                // XXX this is a gross oversimplification that will need to be fixed.
                // What we do is: if the child is NOT a thread, we copy all the parent fds.
                // The right thing to do is looking at PPM_CL_CLONE_FILES, but there are
                // syscalls like open and pipe2 that can override PPM_CL_CLONE_FILES with the O_CLOEXEC
                // flag."
                child_tinfo.fdlist = l_tinfo.fdlist.clone();
                for fdinfo in child_tinfo.fdlist.values_mut() {
                    fdinfo.state_flags |= FLAGS_IS_CLONED;
                }

                child_tinfo.env = l_tinfo.env.clone();
                child_tinfo.cwd = l_tinfo.cwd.clone();
            } else {
                child_tinfo.lastexec_ts = l_tinfo.lastexec_ts;
            }
        }

        child_tinfo.fdlimit =
            i64::from_ne_bytes(event.event_params[7].param_data.as_slice().try_into()?);

        Self::populate_vmem_params_from_event(&mut child_tinfo, event)?;

        // TODO(bml) libsinsp also populates a container-linked userlist here, but I think
        // that should probably be handled elsewhere for now.
        child_tinfo.uid =
            u32::from_ne_bytes(event.event_params[16].param_data.as_slice().try_into()?);
        child_tinfo.gid =
            u32::from_ne_bytes(event.event_params[17].param_data.as_slice().try_into()?);

        if event.event_params.len() >= 14 && !event.event_params[14].param_data.is_empty() {
            // this is cheezy but it works, since the threadinfo type,
            // like the C type, deals in a string with embedded null-terminated substrings
            let cgroups: Vec<String> = event.event_params[14]
                .param_data
                .split(|&b| b == 0)
                .filter(|s| !s.is_empty())
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect();
            child_tinfo.cgroups = cgroups.join("\0");
        }

        child_tinfo.clone_ts = event.timestamp;

        let boot_epoch = self.boot_epoch;

        if (child_tinfo.flags & ppm_consts::PPM_CL_CHILD_IN_PIDNS != 0)
            || (child_tinfo.flags & ppm_consts::PPM_CL_CLONE_NEWPID != 0)
            || child_tinfo
                .vtid
                .zip(child_tinfo.tid)
                .is_some_and(|(vtid, tid)| vtid != tid)
        {
            if event.event_params.len() >= 20 && !event.event_params[20].param_data.is_empty() {
                child_tinfo.pidns_init_start_ts =
                    u64::from_ne_bytes(event.event_params[20].param_data.as_slice().try_into()?)
                        + boot_epoch;
            }
        } else {
            child_tinfo.pidns_init_start_ts = boot_epoch;
        }

        if let Some(c_tid) = child_tinfo.tid {
            self.seen_threadsnaps.insert(c_tid, child_tinfo);
        } else {
            debug!("child has no tid");
        }

        Ok(())
    }

    fn parse_clone_fork_exit_caller(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        child_tid: u64,
    ) -> Result<()> {
        let caller_tid = event.thread_id;

        // the caller tinfo, if we have it, is liable to be the most complete/accurate.
        // if we don't have it, we can upward-populate some of the info from the child tinfo,
        // but that is typically best-effort enrichment.
        let mut valid_caller = true;

        let Some(found_caller) = self.with_mut_threadinfo_ctx(event, |caller_tinfo| {
            if parsers::thread_invalid(caller_tinfo) {
                valid_caller = false;
                caller_tinfo.tid = Some(caller_tid);
                caller_tinfo.pid = Some(
                    i64::from_ne_bytes(
                        event.event_params[4]
                            .param_data
                            .as_slice()
                            .try_into()
                            .ok()?,
                    )
                    .try_into()
                    .ok()?,
                );
                caller_tinfo.ptid = Some(
                    i64::from_ne_bytes(
                        event.event_params[5]
                            .param_data
                            .as_slice()
                            .try_into()
                            .ok()?,
                    )
                    .try_into()
                    .ok()?,
                );
                // if one of these params is present, it is guaranteed that the other will be too.
                if event.event_params.len() >= 19 && !event.event_params[18].param_data.is_empty() {
                    caller_tinfo.vtid = Some(i64::from_ne_bytes(
                        event.event_params[18]
                            .param_data
                            .as_slice()
                            .try_into()
                            .ok()?,
                    ) as u64);
                    caller_tinfo.vpid = Some(i64::from_ne_bytes(
                        event.event_params[19]
                            .param_data
                            .as_slice()
                            .try_into()
                            .ok()?,
                    ) as u64);
                } else {
                    caller_tinfo.vtid = Some(caller_tid);
                    caller_tinfo.vpid = None;
                }
            }
            Some(caller_tinfo.clone())
        }) else {
            return Ok(());
        };

        let flags = u32::from_ne_bytes(event.event_params[15].param_data.as_slice().try_into()?);

        // Try to augur if the child thread this caller spawned will be running in a namespace/container.
        // if so, bail and let parse_clone_fork_exit_child() handle the child thread directly, as it will
        // have more complete namespace-local info than the caller does.
        if (flags & ppm_consts::PPM_CL_CHILD_IN_PIDNS != 0)
            || (flags & ppm_consts::PPM_CL_CLONE_NEWPID != 0)
            || (flags & ppm_consts::PPM_CL_CLONE_PARENT != 0)
            || found_caller.vtid.is_some_and(|vtid| vtid != caller_tid)
        {
            // let the child thread handle itself in-namespace
            return Ok(());
        }

        let chid = child_tid;

        let valid_child_found = self.with_mut_threadsnaps_ctx(|threads| {
            if let Some(child_tinfo) = threads.get(&chid) {
                if (child_tinfo.flags & ppm_consts::PPM_CL_CLONE_INVERTED) != 0 {
                    return Some(true);
                }
                // Flag is NOT set, so remove it
                threads.remove(&chid);
            }
            None
        });

        if valid_child_found.is_some_and(|found| found) {
            // child thread already present, nothing to do.
            return Ok(());
        }

        // At this point, we assume child tinfo isn't present, and it's our responsibility to add it.
        let mut new_child_tinfo = ZoneKernelThreadInfo {
            tid: Some(child_tid),
            flags,
            lastexec_ts: 0,
            ..Default::default()
        };

        // thread leader case
        if (new_child_tinfo.flags & ppm_consts::PPM_CL_CLONE_THREAD) == 0 {
            // libsinsp: "We populate fdtable, cwd and env only if we are
            // a new leader thread, all not leader threads will use the same information
            // of the main thread.
            if valid_caller {
                new_child_tinfo.fdlist = found_caller.fdlist.clone();
                for fdinfo in new_child_tinfo.fdlist.values_mut() {
                    fdinfo.state_flags |= FLAGS_IS_CLONED;
                }

                new_child_tinfo.env = found_caller.env.clone();
                new_child_tinfo.cwd = found_caller.cwd.clone();
            }

            new_child_tinfo.pid = new_child_tinfo.tid;
            new_child_tinfo.ptid = new_child_tinfo.tid;
        } else {
            new_child_tinfo.pid = found_caller.pid;
            new_child_tinfo.ptid = found_caller.ptid;

            // libsinsp: "Please note this is not the right behavior, it is something we do to be compliant with
            // `/proc` scan.
            //  In our approximation threads will never have their `fdtable` they will use the main
            //  thread one, for this reason, we keep the main thread alive until we have some threads in
            //  the group."
            // TODO(bml) consequently I don't think we need this, but revisit during cleanup
            // child_tinfo->m_flags |= PPM_CL_CLONE_FILES;

            new_child_tinfo.lastexec_ts = found_caller.lastexec_ts;
        }

        // at this point due to above check, we should be guaranteed this is not a child thread in a container,
        // that is, a different namespace than the caller.

        new_child_tinfo.vtid = new_child_tinfo.tid;
        new_child_tinfo.vpid = new_child_tinfo.pid;

        Self::populate_command_info_from_event(&mut new_child_tinfo, event)?;

        new_child_tinfo.fdlimit =
            i64::from_ne_bytes(event.event_params[7].param_data.as_slice().try_into()?);

        Self::populate_vmem_params_from_event(&mut new_child_tinfo, event)?;

        // TODO(bml) libsinsp also populates a container-linked userlist here, but I think
        // that should probably be handled elsewhere for now.
        new_child_tinfo.uid =
            u32::from_ne_bytes(event.event_params[16].param_data.as_slice().try_into()?);
        new_child_tinfo.gid =
            u32::from_ne_bytes(event.event_params[17].param_data.as_slice().try_into()?);

        if event.event_params.len() >= 14 && !event.event_params[14].param_data.is_empty() {
            // this is cheezy but it works, since the threadinfo type,
            // like the C type, deals in a string with embedded null-terminated substrings
            let cgroups: Vec<String> = event.event_params[14]
                .param_data
                .split(|&b| b == 0)
                .filter(|s| !s.is_empty())
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect();
            new_child_tinfo.cgroups = cgroups.join("\0");
        }

        new_child_tinfo.clone_ts = event.timestamp;
        new_child_tinfo.pidns_init_start_ts = self.boot_epoch;

        // if the caller is valid, populate child info with it.
        if valid_caller {
            new_child_tinfo.exepath = found_caller.exepath.clone();
            new_child_tinfo.exe_writable = found_caller.exe_writable;
            new_child_tinfo.exe_upper_layer = found_caller.exe_upper_layer;
            new_child_tinfo.exe_lower_layer = found_caller.exe_lower_layer;
            new_child_tinfo.exe_from_memfd = found_caller.exe_from_memfd;
            new_child_tinfo.root = found_caller.root.clone();
            new_child_tinfo.sid = found_caller.sid;
            new_child_tinfo.vpgid = found_caller.vpgid;
            new_child_tinfo.pgid = found_caller.pgid;
            new_child_tinfo.tty = found_caller.tty;
            new_child_tinfo.loginuid = found_caller.loginuid;
            new_child_tinfo.cap_permitted = found_caller.cap_permitted;
            new_child_tinfo.cap_inheritable = found_caller.cap_inheritable;
            new_child_tinfo.cap_effective = found_caller.cap_effective;
            new_child_tinfo.exe_ino = found_caller.exe_ino;
            new_child_tinfo.exe_ino_ctime = found_caller.exe_ino_ctime;
            new_child_tinfo.exe_ino_mtime = found_caller.exe_ino_mtime;
            new_child_tinfo.exe_ino_ctime_duration_clone_ts =
                found_caller.exe_ino_ctime_duration_clone_ts
        } else {
            //if the caller was not valid, we can embellish the caller itself with some child info we have.
            self.with_mut_threadsnaps_ctx(|threads| {
                let caller_tinfo = threads
                    .get_mut(&caller_tid)
                    .expect("must be here at this point");
                caller_tinfo.exe = new_child_tinfo.exe.clone();
                caller_tinfo.comm = new_child_tinfo.comm.clone();
                caller_tinfo.args = event.event_params[2]
                    .param_data
                    .split(|&b| b == 0)
                    .filter(|s| !s.is_empty())
                    .map(|s| String::from_utf8_lossy(s).into_owned())
                    .collect();
                caller_tinfo.args = new_child_tinfo.args.clone();
                Some(())
            });
        }

        if let Some(nctid) = new_child_tinfo.tid {
            self.with_mut_threadsnaps_ctx(|threads| threads.insert(nctid, new_child_tinfo));
        } else {
            error!("invalid child tid, cannot insert");
        }

        Ok(())
    }

    fn parse_pidfd_open_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            // if no thread info, bail
            debug!("no thread found for open pidfd: {:?}", event);
            return Ok(());
        };

        let Some(fdi) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no retval found for open pidfd: {:?}", event);
            return Ok(());
        };

        if event.event_params[1].param_type != param_type::PT_PID as u32 {
            warn!("unexpected param type for open pidfd, cannot infer PID");
        }

        let pid = u64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        let flags = u32::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);

        let fdinfo = ZoneKernelFdInfo {
            fd: Some(fdi as u64),
            fd_type: fd_types::SCAP_FD_PIDFD as i32,
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::Pidfd(ZoneKernelPidFd { pid })),
            }),
            open_flags: flags,
            ..Default::default()
        };

        self.with_mut_leader_fdlist_ctx(event, |fdlist| fdlist.insert(fdi as u64, fdinfo));
        Ok(())
    }

    fn parse_pidfd_getfd_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            // if no thread info, bail
            debug!("no thread found for getfd pidfd: {:?}", event);
            return Ok(());
        };

        let Some(fdi) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no retval found for pidfd/getfd: {:?}", event);
            return Ok(());
        };

        if event.event_params[1].param_type != param_type::PT_PID as u32 {
            warn!("unexpected param type for getfd pidfd, cannot infer PID");
        }
        if event.event_params[2].param_type != param_type::PT_PID as u32 {
            warn!("unexpected param type for getfd pidfd, cannot infer PID");
        }

        let pidfd: u64 =
            i64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?)
                .try_into()?;
        let targetfd: u64 =
            i64::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?)
                .try_into()?;

        // What we need to do here:
        // 1. Get the pid out of the pidfd in our current threadtable
        // 1. Use that pid (as tid for thread leader) to lookup the owner thread.
        // 1. Copy the fdinfo of the target out of that thread to our own thread.
        let target_tid = self.with_mut_leader_fdlist_ctx(event, |fdlist| {
            if let Some(pidfd_fdinfo) = fdlist.get(&pidfd)
                && pidfd_fdinfo.fd_type == fd_types::SCAP_FD_PIDFD as i32
            {
                pidfd_fdinfo
                    .info
                    .as_ref()
                    .and_then(|info| match info.info_type.as_ref() {
                        Some(InfoType::Pidfd(pid)) => Some(pid.pid),
                        _ => {
                            warn!("expected a pidfd type fd");
                            None
                        }
                    })
            } else {
                warn!("no pidfd found for {:?}", pidfd);
                None
            }
        });

        if let Some(t_tid) = target_tid
            && let Some(target_fdinfo) = self.with_mut_threadsnaps_ctx(|threadsnaps| {
                threadsnaps
                    .get(&t_tid)
                    .and_then(Self::get_leader_tid)
                    .and_then(|ltid| threadsnaps.get_mut(&ltid))
                    .and_then(|leader| leader.fdlist.get(&targetfd).cloned())
            })
        {
            self.with_mut_leader_fdlist_ctx(event, |fdlist| {
                fdlist.insert(fdi as u64, target_fdinfo)
            });
        }
        Ok(())
    }

    fn parse_execve_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        let Some(tinfo) = self.seen_threadsnaps.get_mut(&event.thread_id) else {
            // if no thread info, bail
            debug!("no thread found for execve: {:?}", event);
            return Ok(());
        };

        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no retval found for execve: {:?}", event);
            return Ok(());
        };

        // libsinsp: "In some corner cases an execve is thrown by a secondary thread when
        // the main thread is already dead. In these cases the secondary thread
        // will become a main thread (it will change its tid) and here we will have
        // an execve exit event called by a main thread that is dead.
        // What we need to do is to set the main thread as alive again and then
        // a new PROC_EXIT event will kill it again.
        // This is what happens with `stress-ng --exec`."
        if parsers::thread_dead(tinfo) {
            Self::mark_thread_undead(tinfo);
        }

        if let Some((tgid, active_tid)) = self.with_mut_threadinfo_ctx(event, |tinfo| {
            tinfo.pid = Some(u64::from_ne_bytes(
                event.event_params[4]
                    .param_data
                    .as_slice()
                    .try_into()
                    .ok()?,
            ));

            let invalid_thread = parsers::thread_invalid(tinfo);
            // if this is an invalid/incomplete thread entry, enrich what we can from the execve parent
            if invalid_thread {
                tinfo.ptid = Some(u64::from_ne_bytes(
                    event.event_params[5]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                ));
                if tinfo.flags & ppm_consts::PPM_CL_CHILD_IN_PIDNS == 0 {
                    tinfo.vtid = tinfo.tid;
                    tinfo.vpid = tinfo.pid;
                }
            }

            tinfo.fdlimit = i64::from_ne_bytes(
                event.event_params[7]
                    .param_data
                    .as_slice()
                    .try_into()
                    .ok()?,
            );

            Self::populate_vmem_params_from_event(tinfo, event).ok()?;
            Self::populate_command_info_from_event(tinfo, event).ok()?;

            if event.event_params.len() >= 14 && !event.event_params[14].param_data.is_empty() {
                // this is cheezy but it works, since the threadinfo type,
                // like the C type, deals in a string with embedded null-terminated substrings
                let cgroups: Vec<String> = event.event_params[14]
                    .param_data
                    .split(|&b| b == 0)
                    .filter(|s| !s.is_empty())
                    .map(|s| String::from_utf8_lossy(s).into_owned())
                    .collect();
                tinfo.cgroups = cgroups.join("\0");
            }

            if event.event_params.len() >= 15 && !event.event_params[15].param_data.is_empty() {
                let env: Vec<String> = event.event_params[15]
                    .param_data
                    .split(|&b| b == 0)
                    .filter(|s| !s.is_empty())
                    .map(|s| String::from_utf8_lossy(s).into_owned())
                    .collect();
                tinfo.env = env.join("\0");
                // TODO(bml) libsinsp here tries to detect if `env` was truncated on the
                // syscall and if so hits `/proc` for a full list, before giving up.
                // Skipping this for now.
            }

            if event.event_params.len() >= 16 && !event.event_params[16].param_data.is_empty() {
                tinfo.tty = u32::from_ne_bytes(
                    event.event_params[16]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            if event.event_params.len() >= 17 && !event.event_params[17].param_data.is_empty() {
                tinfo.vpgid = u64::from_ne_bytes(
                    event.event_params[17]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            if event.event_params.len() >= 18 && !event.event_params[18].param_data.is_empty() {
                tinfo.loginuid = u32::from_ne_bytes(
                    event.event_params[18]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            if event.event_params.len() >= 19 && !event.event_params[19].param_data.is_empty() {
                tinfo.flags = u32::from_ne_bytes(
                    event.event_params[19]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );

                tinfo.exe_writable = (tinfo.flags & ppm_consts::PPM_EXE_WRITABLE) != 0;
                tinfo.exe_upper_layer = (tinfo.flags & ppm_consts::PPM_EXE_UPPER_LAYER) != 0;
                tinfo.exe_from_memfd = (tinfo.flags & ppm_consts::PPM_EXE_FROM_MEMFD) != 0;
                tinfo.exe_lower_layer = (tinfo.flags & ppm_consts::PPM_EXE_LOWER_LAYER) != 0;
            }

            // capabilities.
            // if one of these is populated, it is guaranteed the others will be
            if event.event_params.len() >= 20 && !event.event_params[20].param_data.is_empty() {
                tinfo.cap_inheritable = u64::from_ne_bytes(
                    event.event_params[20]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );

                tinfo.cap_permitted = u64::from_ne_bytes(
                    event.event_params[21]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );

                tinfo.cap_effective = u64::from_ne_bytes(
                    event.event_params[22]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            // exe ino stuff
            // if one of these is populated, it is guaranteed the others will be
            if event.event_params.len() >= 23 && !event.event_params[23].param_data.is_empty() {
                tinfo.exe_ino = u64::from_ne_bytes(
                    event.event_params[23]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );

                tinfo.exe_ino_ctime = u64::from_ne_bytes(
                    event.event_params[24]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );

                tinfo.exe_ino_mtime = u64::from_ne_bytes(
                    event.event_params[25]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );

                if tinfo.clone_ts != 0 {
                    tinfo.exe_ino_ctime_duration_clone_ts =
                        tinfo.clone_ts.wrapping_sub(tinfo.exe_ino_ctime);
                }

                if tinfo.pidns_init_start_ts != 0 && tinfo.exe_ino_ctime > tinfo.pidns_init_start_ts
                {
                    tinfo.exe_ino_ctime_duration_pidns_start =
                        tinfo.exe_ino_ctime.wrapping_sub(tinfo.pidns_init_start_ts);
                }
            }

            if event.event_params.len() >= 26 && !event.event_params[26].param_data.is_empty() {
                tinfo.uid = u32::from_ne_bytes(
                    event.event_params[26]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            if event.event_params.len() >= 28 && !event.event_params[28].param_data.is_empty() {
                tinfo.pgid = u64::from_ne_bytes(
                    event.event_params[28]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            if event.event_params.len() >= 29 && !event.event_params[29].param_data.is_empty() {
                tinfo.gid = u32::from_ne_bytes(
                    event.event_params[29]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            // shell pipe flags
            let spf = tinfo.flags
                & (ppm_consts::PPM_CL_PIPE_SRC
                    | ppm_consts::PPM_CL_PIPE_DST
                    | ppm_consts::PPM_CL_IS_MAIN_THREAD);
            let inverted = (tinfo.flags & ppm_consts::PPM_CL_CLONE_INVERTED) != 0;
            // clear flags
            tinfo.flags = ppm_consts::PPM_CL_ACTIVE;
            tinfo.flags |= spf;

            if inverted {
                tinfo.flags |= ppm_consts::PPM_CL_CLONE_INVERTED;
            }

            // procname has changed, so set that flag.
            tinfo.flags |= ppm_consts::PPM_CL_NAME_CHANGED;

            Some((tinfo.pid, tinfo.tid))
        }) {
            // in Linux, `execve` causes all threads in a thread group (except the leader thread)
            // to be terminated immediately.
            // So loop through the thread list here and drop any threads that have the same pid as us,
            // but where pid != tid.
            // ALSO retain "this" thread, even if it's not the main thread, as per libsinsp:
            // "Also make sure the thread to be removed is not the one
            // associated with the event. Under normal conditions this
            // should not happen, since the kernel will reassign tid before
            // returning from the exec syscall. But there are crash reports,
            // indicating possibility the original tid is kept in place, but
            // the syscall still returns a success."
            self.with_mut_threadsnaps_ctx(|threadsnaps| {
                threadsnaps.retain(|_tid, tinfo| {
                    tinfo.pid != tgid || (parsers::thread_main(tinfo) || tinfo.tid == active_tid)
                });
                Some(())
            });
        }

        Ok(())
    }

    // Note that even though this indicates the thread has terminated, we do not remove the threadinfo
    // from our internal state immediately, as the rules engine handling this event may still need
    // the threadinfo to exist.
    //
    // So, we mark the thread as dead, and do cleanup/removal OOB.
    fn parse_thread_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        etype: event_codes,
    ) -> Result<()> {
        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!(
                "no thread found for thread exit, thread already deleted: {:?}",
                event
            );
            return Ok(());
        }

        let reaper_tid = self.with_mut_threadinfo_ctx(event, |tinfo| {
            Self::mark_thread_dead(tinfo);
            if etype == event_codes::PPME_PROCEXIT_1_E && event.event_params.len() > 4 {
                i64::from_ne_bytes(
                    event.event_params[4]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                )
                .try_into()
                .ok()
            } else {
                None
            }
        });

        // Move any children of this thread to the reaper thread, if the event defines one.
        // Otherwise, orphan them. Note that as per libsinsp you cannot derive reaper thread
        // info from /proc scans, so this event is very much best effort/the only place to get reaper info
        self.with_mut_threadsnaps_ctx(|threadsnaps| {
            for tinfo in threadsnaps.values_mut() {
                if tinfo.ptid.is_some() && tinfo.ptid == tinfo.tid {
                    tinfo.ptid = reaper_tid; // if reapertid is Some(u64), set that. Otherwise, set to None/orphan.
                }
            }
            Some(())
        });

        // at this point (procexit) go ahead and purge any threads previously
        // marked dead that are _not_ this thread, as a cleanup.
        // (do not want to purge this thread yet, as it is "the thread we are processing this event for"
        // and it needs to be kept until the next event comes thru (at most) for rules engine context)
        self.purge_other_dead_threads(event);

        Ok(())
    }

    fn parse_pipe_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        etype: event_codes,
    ) -> Result<()> {
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            // if no thread info, bail
            debug!("no thread found for pipe event {:?}", event);
            return Ok(());
        };

        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            // syscall failed, nothing to do
            warn!("no retval found for pipe event: {:?}", event);
            return Ok(());
        };

        let fd1: u64 = i64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?)
            .try_into()?;
        let fd2: u64 = i64::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?)
            .try_into()?;
        let ino: u64 = u64::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?);

        let openflags = if etype == event_codes::PPME_SYSCALL_PIPE2_X {
            u32::from_ne_bytes(event.event_params[4].param_data.as_slice().try_into()?)
        } else {
            0
        };

        let fdinfo1 = ZoneKernelFdInfo {
            fd: Some(fd1),
            fd_type: fd_types::SCAP_FD_FIFO as i32,
            inode: ino,
            open_flags: openflags,
            ..Default::default()
        };

        let mut fdinfo2 = fdinfo1.clone();

        fdinfo2.fd = Some(fd2);

        self.with_mut_leader_fdlist_ctx(event, |fdlist| {
            fdlist.insert(fd1, fdinfo1);
            fdlist.insert(fd2, fdinfo2)
        });

        Ok(())
    }

    fn parse_socket_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        if self.seen_threadsnaps.get_mut(&event.thread_id).is_none() {
            debug!("no thread found for socket event {:?}", event);
            return Ok(());
        }

        let Some(fdi) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            // syscall failed, nothing to do
            warn!("no retval found for socket event: {:?}", event);
            return Ok(());
        };

        let domain = u32::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        // this deeply stupid casting is a libsinsp/CPP relic AFAICT.
        let s_type: i32 =
            u32::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?)
                .try_into()?;
        let protocol: i32 =
            u32::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?)
                .try_into()?;

        self.add_socket_fd(event, Some(fdi as u64), domain, s_type, protocol);

        Ok(())
    }

    fn parse_bind_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            // syscall failed, nothing to do
            warn!("no retval found for bind event: {:?}", event);
            return Ok(());
        };

        if event.event_params[1].param_data.is_empty() {
            // not much we can do if address info not here.
            warn!("no address info found for bind enter event: {:?}", event);
            return Ok(());
        }

        // for this event type, we have to look back at the corresponding enter event to extract the FD.
        let enter_fdi: Option<u64> = parsers::get_fdi(event);

        if let Some(e_fdi) = enter_fdi {
            self.with_mut_leader_fdlist_ctx(event, |fdlist| {
                if let Some(fdinfo) = fdlist.get_mut(&e_fdi) {
                    Self::update_sockfd_from_tuple(fdinfo, &event.event_params[1]);
                    Self::set_role_server(fdinfo);
                    Some(())
                } else {
                    None
                }
            });
        } else {
            return Ok(());
        }
        Ok(())
    }

    fn parse_connect_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!("no thread found for connect event {:?}", event);
            return Ok(());
        }

        let mut overwrite_stale = false;

        // for this event type, we have to look back at the corresponding enter event to extract the FD.
        let enter_fdi: Option<u64> = parsers::get_fdi(event);

        // this event type also has the fd in the exit, so if we (for some reason)
        // could not snag it from the enter event, then grab it here as a fallback
        let e_fdi = enter_fdi.or_else(|| {
            if event.event_params[2].param_type != param_type::PT_FD as u32 {
                warn!("unexpected param type for connect exit, cannot infer FD");
                None
            } else {
                overwrite_stale = true; // we probably missed the enter event
                event.event_params[2]
                    .param_data
                    .as_slice()
                    .try_into()
                    .ok()
                    .map(i64::from_ne_bytes)
                    .and_then(|i| i.try_into().ok())
            }
        });

        let Some(fdi) = e_fdi else {
            // can't do anything
            return Ok(());
        };

        self.with_mut_leader_fdlist_ctx(event, |fdlist| {
            if let Some(fdinfo) = fdlist.get_mut(&fdi) {
                if let Some(retval) = parsers::get_retval(event) {
                    if retval == -SE_EINPROGRESS {
                        fdinfo.state_flags &= !(FLAGS_SOCKET_CONNECTED | FLAGS_CONNECTION_FAILED);
                        fdinfo.state_flags |= FLAGS_CONNECTION_PENDING;
                    } else if retval < 0 {
                        Self::set_socket_failed(fdinfo);
                    } else {
                        Self::set_socket_connected(fdinfo);
                    }
                }

                // TODO(bml) libsinsp uses a similar routine for this and conditionally overwrites.
                // I'm going to unconditionally overwrite for now, we can make it conditional if it
                // ends up mattering.
                Self::update_sockfd_from_tuple(fdinfo, &event.event_params[1]);
                Self::set_role_client(fdinfo);
                Some(())
            } else {
                None
            }
        });

        Ok(())
    }

    fn parse_accept_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        if self.seen_threadsnaps.get_mut(&event.thread_id).is_none() {
            debug!("no thread found for connect event {:?}", event);
            return Ok(());
        }

        let Some(fdi) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            // syscall failed, nothing to do
            warn!("no retval found for socket event: {:?}", event);
            return Ok(());
        };

        let mut fdinfo = ZoneKernelFdInfo {
            fd: Some(fdi as u64),
            ..Default::default()
        };
        Self::update_sockfd_from_tuple(&mut fdinfo, &event.event_params[1]);

        Self::set_role_server(&mut fdinfo);
        Self::set_socket_connected(&mut fdinfo);

        self.with_mut_leader_fdlist_ctx(event, |fdlist| {
            fdlist.insert(fdi as u64, fdinfo);
            Some(())
        });

        Ok(())
    }

    fn parse_close_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            debug!("no success retval found for close event: {:?}", event);
            return Ok(());
        };

        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!("no thread found for fcntl event {:?}", event);
            return Ok(());
        }

        // for this event type, we have to look back at the corresponding enter event to extract the FD.
        if let Some(enter_fdi) = parsers::get_fdi(event) {
            self.mark_fd_dropped(&event.thread_id, &enter_fdi);
        }
        Ok(())
    }

    fn parse_fcntl_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!("no thread found for fcntl event {:?}", event);
            return Ok(());
        }

        // if syscall failed, bail
        let Some(new_fdi) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no success retval found for fcntl event: {:?}", event);
            return Ok(());
        };

        let cmd: u32 = i8::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?)
            .try_into()?;

        if !(cmd == ppm_consts::PPM_FCNTL_F_DUPFD_CLOEXEC || cmd == ppm_consts::PPM_FCNTL_F_DUPFD) {
            return Ok(());
        }

        // reinsert the fd with the new fd number
        parsers::get_fdi(event).and_then(|enter_fdi| {
            self.with_mut_leader_fdlist_ctx(event, |fdlist| {
                fdlist
                    .get(&enter_fdi)
                    .cloned()
                    .and_then(|fdinfo| fdlist.insert(new_fdi as u64, fdinfo))
            })
        });
        Ok(())
    }

    fn parse_eventfd_eventfd2_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        etype: event_codes,
    ) -> Result<()> {
        // if no thread info, bail
        if self.seen_threadsnaps.get_mut(&event.thread_id).is_none() {
            debug!("no thread found for eventfd event {:?}", event);
            return Ok(());
        }

        // if syscall failed, bail
        let Some(new_fdi) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no success retval found for eventfd event: {:?}", event);
            return Ok(());
        };

        let mut fdinfo = ZoneKernelFdInfo {
            fd: Some(new_fdi as u64),
            fd_type: fd_types::SCAP_FD_EVENT as i32,
            ..Default::default()
        };

        if etype == event_codes::PPME_SYSCALL_EVENTFD2_X {
            fdinfo.open_flags =
                u16::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?).into();
        }

        self.with_mut_leader_fdlist_ctx(event, |fdlist| fdlist.insert(new_fdi as u64, fdinfo));
        Ok(())
    }

    fn parse_chdir_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if we have a thread for this event, update the cwd of that thread, otherwise NBD
        self.with_mut_threadinfo_ctx(event, |tinfo| {
            tinfo.cwd = event.event_params[1].param_pretty.clone(); // pretty version is already a string
            Some(())
        });
        Ok(())
    }

    fn parse_fchdir_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for fchdir event: {:?}", event);
            return Ok(());
        };

        // set the thread cwd to the name of the fd fchdir was invoked aginst
        parsers::get_fdi(event).map(|enter_fdi| {
            self.with_mut_leader_fdlist_ctx(event, |fdlist| {
                fdlist.get(&enter_fdi).map(|fdinfo| {
                    fdinfo
                        .info
                        .as_ref()
                        .and_then(|info| match info.info_type.as_ref() {
                            Some(InfoType::FileName(name)) => Some(name.clone()),
                            Some(InfoType::RegularFile(file)) => Some(file.name.clone()),
                            _ => None,
                        })
                        .unwrap_or("NA".to_string())
                })
            })
            .map(|dirname| {
                self.with_mut_threadinfo_ctx(event, |tinfo| {
                    tinfo.cwd = dirname;
                    Some(())
                })
            })
        });
        Ok(())
    }

    fn parse_getcwd_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for getcwd event: {:?}", event);
            return Ok(());
        };

        // if no thread info, bail
        if self.seen_threadsnaps.get_mut(&event.thread_id).is_none() {
            debug!("no thread found for getcwd event {:?}", event);
            return Ok(());
        }

        // set the thread cwd to the name of the fd getcwd was invoked aginst
        self.with_mut_threadinfo_ctx(event, |tinfo| {
            tinfo.cwd = event.event_params[1].param_pretty.clone();
            Some(())
        });
        Ok(())
    }

    fn parse_dup_exit(&mut self, event: &ZoneKernelSyscallEvent, etype: event_codes) -> Result<()> {
        // if syscall failed, bail
        let Some(retval) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no success retval found for dup event: {:?}", event);
            return Ok(());
        };

        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!("no thread found for dup event {:?}", event);
            return Ok(());
        };

        self.with_mut_threadinfo_ctx(event, |mut_tinfo| {
            // determine if thread is part of shell pipe
            if retval == 0 {
                mut_tinfo.flags |= ppm_consts::PPM_CL_PIPE_DST;
            }
            if retval == 1 {
                mut_tinfo.flags |= ppm_consts::PPM_CL_PIPE_SRC;
            }
            Some(())
        });

        // libsinsp: "If the old FD is in the table, remove it properly.
        // The old FD is:
        // 	- dup(): fd number of a previously closed fd that has not been removed from the fd_table
        //		     and has been reassigned to the newly created fd by dup()(very rare condition);
        //  - dup2(): fd number of an existing fd that we pass to the dup2() as the "newfd". dup2()
        //			  will close the existing one. So we need to clean it up / overwrite;
        //  - dup3(): same as dup2()."

        let oldfdi = retval as u64;
        let enterfdi = parsers::get_fdi(event);
        self.with_mut_leader_ctx(event, |l_tinfo| {
            // for dup3, we only need to set/reset CLOEXEC
            if etype == event_codes::PPME_SYSCALL_DUP3_X {
                let flags = u32::from_ne_bytes(
                    event.event_params[3]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
                // set the thread cwd to the name of the fd fchdir was invoked aginst
                if let Some(enter_fdi) = enterfdi
                    && let Some(fdinfo) = l_tinfo.fdlist.get_mut(&enter_fdi)
                {
                    if flags != 0 {
                        fdinfo.open_flags |= flags;
                    } else {
                        fdinfo.open_flags &= !ppm_consts::PPM_O_CLOEXEC;
                    }

                    let newinfo = fdinfo.clone();

                    l_tinfo.fdlist.insert(oldfdi, newinfo);
                };
            }
            Some(())
        });
        Ok(())
    }

    fn parse_single_param_fd_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        etype: event_codes,
        fdtype: fd_types,
    ) -> Result<()> {
        // if syscall failed, bail
        let Some(retval) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no success retval found for parsefd event: {:?}", event);
            return Ok(());
        };

        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!("no thread found for parsefd event {:?}", event);
            return Ok(());
        };

        let mut newfdinfo = ZoneKernelFdInfo {
            fd: Some(retval as u64),
            fd_type: fdtype as i32,
            ..Default::default()
        };
        if etype == event_codes::PPME_SYSCALL_INOTIFY_INIT1_X
            || etype == event_codes::PPME_SYSCALL_SIGNALFD4_X
        {
            newfdinfo.open_flags =
                u16::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?).into();
        }

        self.with_mut_leader_fdlist_ctx(event, |fdlist| fdlist.insert(retval as u64, newfdinfo));

        Ok(())
    }

    fn parse_getrlimit_setrlimit_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for rlimit event: {:?}", event);
            return Ok(());
        };

        // if no thread info, bail
        let Some(tgid) = self.with_mut_threadsnaps_ctx(|threadsnap| {
            threadsnap.get(&event.thread_id).and_then(|tinfo| tinfo.pid)
        }) else {
            debug!("no thread found for rlimit event {:?}", event);
            return Ok(());
        };

        let resource_id: u32 =
            u8::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?).into();

        if resource_id != ppm_consts::PPM_RLIMIT_NOFILE {
            debug!("unhandled resource type");
            return Ok(());
        }

        // libsinsp does fucky casting here, this is the safe version
        let resource_val =
            i64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);

        if resource_val >= 0 {
            self.with_mut_threadsnaps_ctx(|threadsnap| {
                if let Some(main_thread) = threadsnap.get_mut(&tgid) {
                    main_thread.fdlimit = resource_val;
                }
                Some(())
            });
        }
        Ok(())
    }

    fn parse_prlimit_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for prlimit event: {:?}", event);
            return Ok(());
        };

        // if no thread info, bail
        if self
            .with_mut_threadsnaps_ctx(|threadsnap| {
                threadsnap.get(&event.thread_id).and_then(|tinfo| tinfo.pid)
            })
            .is_none()
        {
            debug!("no thread found for prlimit event {:?}", event);
            return Ok(());
        };

        let resource_id: u32 =
            u8::from_ne_bytes(event.event_params[6].param_data.as_slice().try_into()?).into();

        if resource_id != ppm_consts::PPM_RLIMIT_NOFILE {
            debug!("unhandled resource type");
            return Ok(());
        }

        let resource_val =
            i64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        let mut tid_evt: u64 =
            i64::from_ne_bytes(event.event_params[5].param_data.as_slice().try_into()?)
                .try_into()?;

        if tid_evt == 0 {
            tid_evt = event.thread_id
        }

        let Some(parent_thread_tgid) = self
            .seen_threadsnaps
            .get(&tid_evt)
            .and_then(|parent_thread| parent_thread.pid)
        else {
            warn!("no parent thread tgid found");
            return Ok(());
        };

        if let Some(main_thread) = self.seen_threadsnaps.get_mut(&parent_thread_tgid) {
            main_thread.fdlimit = resource_val;
        }

        Ok(())
    }

    fn parse_socketpair_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!("no thread found for event {:?}", event);
            return Ok(());
        };

        let fd1 = i64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        let fd2 = i64::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);

        if fd1 == fd2 {
            warn!("invalid socket fds for socketpair, ignoring {:?}", event);
            return Ok(());
        }

        let source_addr =
            u64::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?);
        let peer_addr = u64::from_ne_bytes(event.event_params[4].param_data.as_slice().try_into()?);

        let newfdinfo = ZoneKernelFdInfo {
            fd: Some(fd1 as u64),
            fd_type: fd_types::SCAP_FD_UNIX_SOCK as i32,
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::UnixSocket(ZoneKernelUnixSocketInfo {
                    source: source_addr,
                    destination: peer_addr,
                    name: "NA".into(),
                })),
            }),
            ..Default::default()
        };

        let mut newfdinfo2 = newfdinfo.clone();

        self.with_mut_leader_fdlist_ctx(event, |fdlist| {
            fdlist.insert(fd1 as u64, newfdinfo);
            newfdinfo2.fd = Some(fd2 as u64);
            fdlist.insert(fd2 as u64, newfdinfo2)
        });

        Ok(())
    }

    // this is an ENTER event, unlike most of the others.
    fn parse_context_switch(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        let Some(mut_tinfo) = self.seen_threadsnaps.get_mut(&event.thread_id) else {
            debug!("no thread found for event {:?}", event);
            return Ok(());
        };

        mut_tinfo.pfmajor =
            u64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        mut_tinfo.pfminor =
            u64::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);

        let Some(parent_thread_tgid) = self
            .seen_threadsnaps
            .get(&event.thread_id)
            .and_then(|parent_thread| parent_thread.pid)
        else {
            warn!("no parent thread tgid found");
            return Ok(());
        };

        self.with_mut_threadsnaps_ctx(|threadsnap| {
            if let Some(main_thread) = threadsnap.get_mut(&parent_thread_tgid) {
                main_thread.vmsize_kb = u32::from_ne_bytes(
                    event.event_params[3]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
                main_thread.vmrss_kb = u32::from_ne_bytes(
                    event.event_params[4]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
                main_thread.vmswap_kb = u32::from_ne_bytes(
                    event.event_params[5]
                        .param_data
                        .as_slice()
                        .try_into()
                        .ok()?,
                );
            }

            Some(())
        });

        let exectime = self
            .last_proc_switch_times_by_cpuid
            .get(&event.cpuid)
            .cloned()
            .unwrap_or_default();

        self.last_proc_switch_times_by_cpuid.insert(
            event.cpuid,
            ExecTime {
                last_switch_ts: event.timestamp,
                previous_switch_ts: exectime.last_switch_ts,
                cumulative_switch_time: exectime.cumulative_switch_time
                    + (event.timestamp - exectime.last_switch_ts),
            },
        );

        Ok(())
    }

    fn parse_brk_mmap_mmap2_munmap_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        let Some(mut_tinfo) = self.seen_threadsnaps.get_mut(&event.thread_id) else {
            debug!("no thread found for event {:?}", event);
            return Ok(());
        };

        mut_tinfo.vmsize_kb =
            u32::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        mut_tinfo.vmrss_kb =
            u32::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);
        mut_tinfo.vmswap_kb =
            u32::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?);

        Ok(())
    }

    fn parse_uid_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        event_uid_idx: usize,
    ) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        self.set_thread_user(event, event_uid_idx);

        Ok(())
    }

    fn parse_gid_exit(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        event_gid_idx: usize,
    ) -> Result<()> {
        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        self.set_thread_group(event, event_gid_idx);

        Ok(())
    }

    fn parse_chroot_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        let Some(mut_tinfo) = self.seen_threadsnaps.get_mut(&event.thread_id) else {
            debug!("no thread found for event {:?}", event);
            return Ok(());
        };

        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        // TODO(bml) libsinsp tries to hydrate this to an abspath at parse time,
        // since we have `pretty` params, we should be able to do that at capture time in-zone,
        // verify we are.
        mut_tinfo.root = event.event_params[1].param_pretty.clone();

        Ok(())
    }

    fn parse_setsid_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        let Some(mut_tinfo) = self.seen_threadsnaps.get_mut(&event.thread_id) else {
            warn!("no thread found for event {:?}", event);
            return Ok(());
        };

        // if syscall failed, bail
        let Some(retval) = parsers::get_retval(event).filter(|&v| v >= 0) else {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        mut_tinfo.sid = retval as u64;

        Ok(())
    }

    fn parse_getsockopt_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        if !self.seen_threadsnaps.contains_key(&event.thread_id) {
            debug!("no thread found for event {:?}", event);
            return Ok(());
        };

        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        let fd: u64 = i64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?)
            .try_into()?;
        let level: u32 = i8::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?)
            .try_into()?;
        let optname: u32 =
            i8::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?)
                .try_into()?;

        let parinfo = event.event_params[4].param_data.clone();

        // TODO(bml) this can be the event's overriding retcode in the below case
        let err = i64::from_ne_bytes(
            parinfo[1..9] // add 1 byte to skip over PT_DYN param index
                .try_into()?,
        );

        if level == ppm_consts::PPM_SOCKOPT_LEVEL_SOL_SOCKET
            && optname == ppm_consts::PPM_SOCKOPT_SO_ERROR
        {
            self.with_mut_leader_fdlist_ctx(event, |fdlist| {
                fdlist.get_mut(&fd).map(|fdinfo| {
                    if err < 0 {
                        Self::set_socket_failed(fdinfo);
                    } else {
                        Self::set_socket_connected(fdinfo);
                    }
                })
            });
        }

        Ok(())
    }

    fn parse_capset_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        let Some(mut_tinfo) = self.seen_threadsnaps.get_mut(&event.thread_id) else {
            debug!("no thread found for event {:?}", event);
            return Ok(());
        };

        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        mut_tinfo.cap_inheritable =
            u64::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        mut_tinfo.cap_permitted =
            u64::from_ne_bytes(event.event_params[2].param_data.as_slice().try_into()?);
        mut_tinfo.cap_effective =
            u64::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?);

        Ok(())
    }

    fn parse_prctl_exit(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        // if no thread info, bail
        let Some(mut_tinfo) = self.seen_threadsnaps.get_mut(&event.thread_id) else {
            debug!("no thread found for event {:?}", event);
            return Ok(());
        };

        // if syscall failed, bail
        if parsers::get_retval(event).filter(|&v| v >= 0).is_none() {
            warn!("no success retval found for event: {:?}", event);
            return Ok(());
        };

        let option = u32::from_ne_bytes(event.event_params[1].param_data.as_slice().try_into()?);
        let child_subreaper =
            i64::from_ne_bytes(event.event_params[3].param_data.as_slice().try_into()?);

        match option {
            // libsinsp: "Parameter 4: arg2_int (type: PT_INT64) */
            // If the user provided an arg2 != 0, we set the child_subreaper
            // attribute for the calling process. If arg2 is zero, unset the attribute"
            ppm_consts::PPM_PR_SET_CHILD_SUBREAPER | ppm_consts::PPM_PR_GET_CHILD_SUBREAPER => {
                mut_tinfo.reaper = child_subreaper != 0;
            }
            _ => return Ok(()),
        }

        Ok(())
    }

    // TODO(bml) do we need to separately track userlists here?
    fn set_thread_user(&mut self, event: &ZoneKernelSyscallEvent, uid_idx: usize) {
        if event.event_params[uid_idx].param_data.is_empty() {
            debug!("no valid UID found in event params");
            return;
        }

        if let Ok(res) = event.event_params[uid_idx].param_data.as_slice().try_into() {
            let uid = u32::from_ne_bytes(res);
            self.with_mut_threadinfo_ctx(event, |tinfo| {
                tinfo.uid = uid;
                Some(())
            });
        } else {
            debug!("no valid UID found in event params")
        }
    }

    // TODO(bml) do we need to separately track grouplists here?
    fn set_thread_group(&mut self, event: &ZoneKernelSyscallEvent, gid_idx: usize) {
        if event.event_params[gid_idx].param_data.is_empty() {
            debug!("no valid GID found in event params");
            return;
        }

        if let Ok(res) = event.event_params[gid_idx].param_data.as_slice().try_into() {
            let gid = u32::from_ne_bytes(res);
            self.with_mut_threadinfo_ctx(event, |tinfo| {
                tinfo.gid = gid;
                Some(())
            });
        } else {
            debug!("no valid GID found in event params")
        }
    }

    fn set_socket_connected(fdinfo: &mut ZoneKernelFdInfo) {
        fdinfo.state_flags &= !(FLAGS_CONNECTION_PENDING | FLAGS_CONNECTION_FAILED);
        fdinfo.state_flags |= FLAGS_SOCKET_CONNECTED;
    }

    fn set_socket_failed(fdinfo: &mut ZoneKernelFdInfo) {
        fdinfo.state_flags &= !(FLAGS_SOCKET_CONNECTED | FLAGS_CONNECTION_PENDING);
        fdinfo.state_flags |= FLAGS_CONNECTION_FAILED;
    }

    fn mark_thread_dead(tinfo: &mut ZoneKernelThreadInfo) {
        tinfo.flags |= ppm_consts::PPM_CL_CLOSED;
    }

    /// In some corner cases is possible that a main thread previously marked as `dead`
    /// could become alive again. For example, when an execve is performed by a secondary
    /// thread and the main thread is already dead.
    fn mark_thread_undead(tinfo: &mut ZoneKernelThreadInfo) {
        if parsers::thread_dead(tinfo) {
            tinfo.flags &= !ppm_consts::PPM_CL_CLOSED;
        }
    }

    fn parse_dirfd(
        &mut self,
        event: &ZoneKernelSyscallEvent,
        event_name: &str,
        dirfd: i64,
    ) -> Option<PathBuf> {
        let is_abs_path = !event_name.is_empty() && event_name.starts_with("/");
        if is_abs_path {
            return Some(PathBuf::from(".".to_string()));
        }

        let valid_dirfd: u64 = dirfd.try_into().ok()?;

        let result = if dirfd == ppm_consts::PPM_AT_FDCWD as i64 {
            self.with_mut_threadinfo_ctx(event, |tinfo| Some(tinfo.cwd.clone()))
        } else {
            self.with_mut_leader_fdlist_ctx(event, |fdlist| {
                fdlist
                    .get(&valid_dirfd)
                    .and_then(|fdinfo| fdinfo.info.as_ref())
                    .map(|info| {
                        let fname = match info.info_type.as_ref() {
                            Some(InfoType::RegularFile(f_name)) => f_name.name.clone(),
                            Some(InfoType::FileName(f_name)) => f_name.clone(),
                            _ => {
                                warn!("expected a real file");
                                "".into()
                            }
                        };

                        if fname.is_empty() || fname.ends_with("/") {
                            fname
                        } else {
                            fname + "/"
                        }
                    })
            })
        };

        if let Some(path) = result
            && !path.is_empty()
        {
            Some(PathBuf::from(path))
        } else {
            None
        }
    }

    fn mark_fd_dropped(&mut self, tid: &u64, fdi: &u64) {
        self.dropped_fds_by_thread.insert(*tid, *fdi);
    }

    /// This function cleans any threads previously marked as dead from the threadsnapshot.
    /// It will never purge the current event's thread, even if that thread has been marked as dead.
    /// That is because a thread's event needs to stick around for context for
    /// as long as the current event lives (but no longer).
    /// Additionally, when a thread is removed, any threads whose ptid matches the removed thread's tid
    /// are also removed.
    fn purge_other_dead_threads(&mut self, event: &ZoneKernelSyscallEvent) {
        self.with_mut_threadsnaps_ctx(|threadsnaps| {
            let mut removed_tids = Vec::new();

            // First pass: identify and remove dead threads (excluding current event's thread)
            threadsnaps.retain(|_tid, tinfo| {
                // remove if thread is
                // - marked dead
                // - is not the current event's thread
                // - is a child thread (pid != tid) and has no valid parent tid (orphan)
                let should_remove = parsers::thread_dead(tinfo)
                    && tinfo.tid.is_some_and(|t_tid| event.thread_id != t_tid);

                if should_remove && let Some(tid) = tinfo.tid {
                    removed_tids.push(tid);
                }

                !should_remove
            });

            // Second pass: remove any threads whose parents we just purged (again retaining the current thread)
            threadsnaps.retain(|_tid, tinfo| {
                let is_current_thread = tinfo.tid.is_some_and(|t_tid| event.thread_id == t_tid);
                let is_orphaned = tinfo.ptid.is_some_and(|ptid| removed_tids.contains(&ptid));

                !is_orphaned || is_current_thread
            });

            Some(())
        });
    }

    /// the libsinsp code here conditionally ovewrites properties of the socket if the socket type changes.
    /// for instance, listen sockets -> sending sockets, and ipv4 to ipv6.
    ///
    /// The historical info isn't important to keep, however - it's a pure overwrite of the socket with
    /// info from the provided tuple param.
    fn update_sockfd_from_tuple(
        fdinfo: &mut ZoneKernelFdInfo,
        tuple_param: &ZoneKernelEventParam,
    ) -> bool {
        // Check if param_data is empty
        if tuple_param.param_data.is_empty() {
            return false;
        }

        let family = tuple_param.param_data[0] as u32;
        let u32_size = std::mem::size_of::<u32>();
        let u16_size = std::mem::size_of::<u16>();
        let u64_size = std::mem::size_of::<u64>();
        let u128_size = std::mem::size_of::<u128>();

        if family == ppm_consts::PPM_AF_INET {
            let mut prev_l4proto = 0;
            // If this was previously a server socket, save off the L4 protocol
            // and switch this to a regular socket
            if fdinfo.fd_type == fd_types::SCAP_FD_IPV4_SERVSOCK as i32
                && let Some(ZoneKernelFdInfoData {
                    info_type: Some(InfoType::Ipv4ServerSocket(server_info)),
                }) = fdinfo.info.take()
            {
                prev_l4proto = server_info.protocol;
            }

            let required_v4_len = 1 + u32_size + u16_size + u32_size + u16_size;
            if tuple_param.param_data.len() < required_v4_len {
                error!("l4 sockfd tuple was not the expected size");
                return false;
            }

            // parse the source/dest port/ip from the param bytes.
            let mut offset = 1;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u32_size].try_into() else {
                error!("error parsing tsip");
                return false;
            };
            let tsip = u32::from_ne_bytes(bytes);
            offset += u32_size;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u16_size].try_into() else {
                error!("error parsing tsport");
                return false;
            };
            let tsport = u16::from_ne_bytes(bytes);
            offset += u16_size;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u32_size].try_into() else {
                error!("error parsing tdip");
                return false;
            };
            let tdip = u32::from_ne_bytes(bytes);
            offset += u32_size;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u16_size].try_into() else {
                error!("error parsing tdport");
                return false;
            };
            let tdport = u16::from_ne_bytes(bytes);

            let info = fdinfo
                .info
                .get_or_insert(ZoneKernelFdInfoData { info_type: None });

            // if we `take()`'d the info type above, replace it. Otherwise, update in place.
            if let Some(InfoType::Ipv4Socket(socket)) = &mut info.info_type {
                socket.source_ip = Ipv4Addr::from(tsip).to_string();
                socket.dest_ip = Ipv4Addr::from(tdip).to_string();
                socket.source_port = tsport as u32;
                socket.dest_port = tdport as u32;
            } else {
                info.info_type = Some(InfoType::Ipv4Socket(ZoneKernelIpv4SocketInfo {
                    source_ip: Ipv4Addr::from(tsip).to_string(),
                    dest_ip: Ipv4Addr::from(tdip).to_string(),
                    source_port: tsport as u32,
                    dest_port: tdport as u32,
                    protocol: prev_l4proto,
                }));
            }

            fdinfo.fd_type = fd_types::SCAP_FD_IPV4_SOCK as i32;
        } else if family == ppm_consts::PPM_AF_INET6 {
            let required_v6_len = 1 + u128_size + u16_size + u128_size + u16_size;
            if tuple_param.param_data.len() < required_v6_len {
                error!("l4 sockfd tuple was not the expected size");
                return false;
            }

            // parse the source/dest port/ip from the param bytes.
            let mut offset = 1;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u128_size].try_into() else {
                error!("error parsing sip");
                return false;
            };
            let sip = Ipv6Addr::from(u128::from_ne_bytes(bytes));
            offset += u128_size;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u16_size].try_into() else {
                error!("error parsing sport");
                return false;
            };
            let sport = u16::from_ne_bytes(bytes);
            offset += u16_size;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u128_size].try_into() else {
                error!("error parsing dip");
                return false;
            };
            let dip = Ipv6Addr::from(u128::from_ne_bytes(bytes));
            offset += u128_size;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u16_size].try_into() else {
                error!("error parsing dport");
                return false;
            };
            let dport = u16::from_ne_bytes(bytes);

            // unmap 4-in-6 addrs, and forcibly convert the socket type
            let v4_mapped = sip.to_ipv4_mapped().is_some() && dip.to_ipv4_mapped().is_some();

            let info = fdinfo
                .info
                .get_or_insert(ZoneKernelFdInfoData { info_type: None });

            if v4_mapped {
                if let Some(InfoType::Ipv4Socket(socket)) = &mut info.info_type {
                    socket.source_ip = sip
                        .to_ipv4_mapped()
                        .expect("previously asserted to be v4")
                        .to_string();
                    socket.dest_ip = dip
                        .to_ipv4_mapped()
                        .expect("previously asserted to be v4")
                        .to_string();
                    socket.source_port = sport as u32;
                    socket.dest_port = dport as u32;
                } else {
                    info.info_type = Some(InfoType::Ipv4Socket(ZoneKernelIpv4SocketInfo {
                        source_ip: sip
                            .to_ipv4_mapped()
                            .expect("previously asserted to be v4")
                            .to_string(),
                        dest_ip: dip
                            .to_ipv4_mapped()
                            .expect("previously asserted to be v4")
                            .to_string(),
                        source_port: sport as u32,
                        dest_port: dport as u32,
                        protocol: l4_types::SCAP_L4_UNKNOWN as i32,
                    }));
                }

                fdinfo.fd_type = fd_types::SCAP_FD_IPV4_SOCK as i32;
            } else {
                if let Some(InfoType::Ipv6Socket(socket)) = &mut info.info_type {
                    socket.source_ip = sip.to_string();
                    socket.dest_ip = dip.to_string();
                    socket.source_port = sport as u32;
                    socket.dest_port = dport as u32;
                } else {
                    info.info_type = Some(InfoType::Ipv6Socket(ZoneKernelIpv6SocketInfo {
                        source_ip: sip.to_string(),
                        dest_ip: dip.to_string(),
                        source_port: sport as u32,
                        dest_port: dport as u32,
                        protocol: l4_types::SCAP_L4_UNKNOWN as i32,
                    }));
                }

                fdinfo.fd_type = fd_types::SCAP_FD_IPV6_SOCK as i32;
            }
        } else if family == ppm_consts::PPM_AF_UNIX {
            let required_unix_len = 1 + u64_size + u64_size + 1; // at least a null-terminated (empty) name
            if tuple_param.param_data.len() < required_unix_len {
                error!(
                    "unix sockfd tuple size {} was not the expected size {required_unix_len}",
                    tuple_param.param_data.len()
                );
                return false;
            }

            let mut offset = 1;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u64_size].try_into() else {
                error!("error parsing source");
                return false;
            };
            let source = u64::from_ne_bytes(bytes);
            offset += u64_size;
            let Ok(bytes) = tuple_param.param_data[offset..offset + u64_size].try_into() else {
                error!("error parsing dest");
                return false;
            };
            let dest = u64::from_ne_bytes(bytes);
            offset += u64_size;

            let Ok(cstr) = CStr::from_bytes_until_nul(&tuple_param.param_data[offset..]) else {
                error!("error parsing name");
                return false;
            };

            let Ok(name) = cstr.to_str() else {
                return false;
            };

            let info = fdinfo
                .info
                .get_or_insert(ZoneKernelFdInfoData { info_type: None });

            if let Some(InfoType::UnixSocket(socket)) = &mut info.info_type {
                socket.source = source;
                socket.destination = dest;
                socket.name = name.to_string();
            } else {
                info.info_type = Some(InfoType::UnixSocket(ZoneKernelUnixSocketInfo {
                    source,
                    destination: dest,
                    name: name.to_string(),
                }));
            }

            fdinfo.fd_type = fd_types::SCAP_FD_UNIX_SOCK as i32;
            // if unix, that's all we have to do, bail
            return true;
        }

        // If we reach this point and the protocol is not set yet, we assume this
        // connection is UDP, because TCP would fail if the address is changed in
        // the middle of a connection.
        if fdinfo.fd_type == fd_types::SCAP_FD_IPV4_SOCK as i32 {
            if let Some(InfoType::Ipv4Socket(socket)) =
                fdinfo.info.as_mut().and_then(|i| i.info_type.as_mut())
                && socket.protocol == l4_types::SCAP_L4_UNKNOWN as i32
            {
                socket.protocol = l4_types::SCAP_L4_UDP as i32;
            }
        } else if fdinfo.fd_type == fd_types::SCAP_FD_IPV6_SOCK as i32
            && let Some(InfoType::Ipv6Socket(socket)) =
                fdinfo.info.as_mut().and_then(|i| i.info_type.as_mut())
            && socket.protocol == l4_types::SCAP_L4_UNKNOWN as i32
        {
            socket.protocol = l4_types::SCAP_L4_UDP as i32;
        }

        // TODO(bml) here libsinsp tries to infer any still-missing sip/dip addresses
        // by scanning local network interfaces. Skipping that for now.
        true
    }

    fn populate_vmem_params_from_event(
        tinfo: &mut ZoneKernelThreadInfo,
        event: &ZoneKernelSyscallEvent,
    ) -> Result<()> {
        // if one of these params is present, it is guaranteed that the other will be too.
        if event.event_params.len() >= 12 && !event.event_params[8].param_data.is_empty() {
            tinfo.pfmajor =
                u64::from_ne_bytes(event.event_params[8].param_data.as_slice().try_into()?);
            tinfo.pfminor =
                u64::from_ne_bytes(event.event_params[9].param_data.as_slice().try_into()?);
            tinfo.vmsize_kb =
                u32::from_ne_bytes(event.event_params[10].param_data.as_slice().try_into()?);
            tinfo.vmrss_kb =
                u32::from_ne_bytes(event.event_params[11].param_data.as_slice().try_into()?);
            tinfo.vmswap_kb =
                u32::from_ne_bytes(event.event_params[12].param_data.as_slice().try_into()?);
            Ok(())
        } else {
            anyhow::bail!("invalid event!")
        }
    }

    fn populate_command_info_from_event(
        tinfo: &mut ZoneKernelThreadInfo,
        event: &ZoneKernelSyscallEvent,
    ) -> Result<()> {
        // set exe
        tinfo.exe = CStr::from_bytes_until_nul(&event.event_params[1].param_data)?
            .to_str()?
            .to_string();

        // set command line string (if part of the event, otherwise just use executable)
        if event.event_params.len() >= 13 && !event.event_params[13].param_data.is_empty() {
            tinfo.comm = CStr::from_bytes_until_nul(&event.event_params[13].param_data)?
                .to_str()?
                .to_string();
        } else {
            tinfo.comm = tinfo.exe.clone();
        }

        // set args. these are sent as null-term strings, and parsed in the rules engine as such.
        // that means we need to read to the *last* null terminator.
        // this is cheezy but it works, since the threadinfo type,
        // like the C type, deals in a string with embedded null-terminated substrings
        let args: Vec<String> = event.event_params[2]
            .param_data
            .split(|&b| b == 0)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect();
        tinfo.args = args.join("\0");

        Ok(())
    }

    fn set_role_server(fdinfo: &mut ZoneKernelFdInfo) {
        fdinfo.state_flags |= FLAGS_ROLE_SERVER;
    }

    fn set_role_client(fdinfo: &mut ZoneKernelFdInfo) {
        fdinfo.state_flags |= FLAGS_ROLE_CLIENT;
    }

    fn swap_addresses(fdinfo: &mut ZoneKernelFdInfo) {
        if let Some(info) = &mut fdinfo.info {
            match &mut info.info_type {
                Some(InfoType::Ipv4Socket(socket_info)) => {
                    let tip = socket_info.source_ip.clone();
                    let tport = socket_info.source_port;
                    socket_info.source_ip = socket_info.dest_ip.clone();
                    socket_info.source_port = socket_info.dest_port;
                    socket_info.dest_ip = tip;
                    socket_info.dest_port = tport;
                }
                Some(InfoType::Ipv6Socket(socket_info)) => {
                    let tip = socket_info.source_ip.clone();
                    let tport = socket_info.source_port;
                    socket_info.source_ip = socket_info.dest_ip.clone();
                    socket_info.source_port = socket_info.dest_port;
                    socket_info.dest_ip = tip;
                    socket_info.dest_port = tport;
                }
                _ => {}
            }
        }
    }
}

impl ThreadState {
    /// Note that this (intentionally) will clear/drop the entire threadstate for the zone,
    /// and replace it with the new one, starting from scratch,
    /// if a given zone ID already has a thread snapshot.
    ///
    /// This is because a "new snapshot" is always assumed to be a state-initializing
    /// (or re-initializing) event.
    pub fn init_zone_with_snap(&mut self, snap: ZoneKernelThreadSnapshotEvent) {
        self.zone_info.insert(
            snap.zone_id.clone(),
            ZoneInfo {
                seen_threadsnaps: snap.thread_info,
                boot_epoch: snap.zone_boot_epoch,
                ..Default::default()
            },
        );
    }

    /// Removes an entire zone's threadsnapshot from the state
    /// (presumably because the zone terminated)
    ///
    /// Note that we may get an event for a zone that we
    /// have already dropped the entire thread table for.
    /// This should be entirely fine - the parser should just
    /// skip the event if no record of that zone exists.
    pub fn drop_zone_from_snap(&mut self, zone_id: &str) {
        self.zone_info.remove(zone_id);
    }

    /// Returns `None` if no threadinfo exists in the table for the zone+threadid combo
    /// specified by the event. Otherwise, runs a FnOnce over an immutable reference to the threadinfo
    pub fn with_threadinfo<F, R>(&self, zone_id: &str, thread_id: &u64, f: F) -> Option<R>
    where
        F: FnOnce(&ZoneKernelThreadInfo) -> Option<R>,
    {
        self.zone_info
            .get(zone_id)
            .and_then(|info| info.seen_threadsnaps.get(thread_id))
            .and_then(f)
    }

    /// Returns `None` if no zoneinfo exists in the table for the zone specified. Otherwise, runs a FnOnce over an
    /// immutable reference to the zoneinfo
    pub fn with_zoneinfo<F, R>(&self, zone_id: &str, f: F) -> Option<R>
    where
        F: FnOnce(&ZoneInfo) -> Option<R>,
    {
        self.zone_info.get(zone_id).and_then(f)
    }

    /// See `sinsp_parser::process_event` in libsinsp/parsers.cpp
    pub fn process_event(&mut self, event: &ZoneKernelSyscallEvent) -> Result<()> {
        use event_codes::*;
        let etype = event_codes::from_repr(event.event_type)
            .ok_or(anyhow!("could not parse event type"))
            .expect("should parse");

        let Some(zinfo) = self.zone_info.get_mut(&event.zone_id) else {
            debug!("ignoring event for unmonitored zone {:?}", &event.zone_id);
            return Ok(());
        };

        match etype {
            // Modern BPF driver only captures exit events - these enter events never arrive:
            // PPME_SYSCALL_OPEN_E | PPME_SYSCALL_CREAT_E | PPME_SYSCALL_OPENAT_E |
            // PPME_SYSCALL_OPENAT_2_E | PPME_SYSCALL_OPENAT2_E | PPME_SYSCALL_EXECVE_19_E |
            // PPME_SYSCALL_EXECVEAT_E
            PPME_SYSCALL_READ_X
            | PPME_SYSCALL_WRITE_X
            | PPME_SOCKET_RECV_X
            | PPME_SOCKET_SEND_X
            | PPME_SOCKET_RECVFROM_X
            | PPME_SOCKET_RECVMSG_X
            | PPME_SOCKET_RECVMMSG_X
            | PPME_SOCKET_SENDTO_X
            | PPME_SOCKET_SENDMSG_X
            | PPME_SOCKET_SENDMMSG_X
            | PPME_SYSCALL_READV_X
            | PPME_SYSCALL_WRITEV_X
            | PPME_SYSCALL_PREAD_X
            | PPME_SYSCALL_PWRITE_X
            | PPME_SYSCALL_PREADV_X
            | PPME_SYSCALL_PWRITEV_X => zinfo.parse_rw_exit(event, etype),
            PPME_SYSCALL_SENDFILE_X => {
                // I don't think we need this, it's mostly for invoking callbacks for optional observers
                // and doesn't seem to affect thread/fd table state
                debug!("nothing to do for sendfile events: {:?}", etype);
                Ok(())
            }
            PPME_SYSCALL_OPEN_X
            | PPME_SYSCALL_CREAT_X
            | PPME_SYSCALL_OPENAT_2_X
            | PPME_SYSCALL_OPENAT2_X
            | PPME_SYSCALL_OPEN_BY_HANDLE_AT_X => zinfo.parse_create_open_exit(event, etype),
            PPME_SYSCALL_FCHMOD_X | PPME_SYSCALL_FCHOWN_X => {
                zinfo.parse_fchmod_fchown_exit(event, etype)
            }
            PPME_SYSCALL_OPENAT_X => {
                zinfo.parse_fspath_related_exit(event, etype)?;
                zinfo.parse_create_open_exit(event, etype)
            }
            PPME_SYSCALL_UNSHARE_X | PPME_SYSCALL_SETNS_X => {
                zinfo.parse_unshare_setns_exit(event, etype)
            }
            PPME_SYSCALL_MEMFD_CREATE_X => zinfo.parse_memfd_create_exit(event),
            PPME_SYSCALL_CLONE_20_X
            | PPME_SYSCALL_FORK_20_X
            | PPME_SYSCALL_VFORK_20_X
            | PPME_SYSCALL_CLONE3_X => zinfo.parse_clone_fork_exit(event),
            PPME_SYSCALL_PIDFD_OPEN_X => zinfo.parse_pidfd_open_exit(event),
            PPME_SYSCALL_PIDFD_GETFD_X => zinfo.parse_pidfd_getfd_exit(event),
            PPME_SYSCALL_EXECVE_19_X | PPME_SYSCALL_EXECVEAT_X => zinfo.parse_execve_exit(event),
            PPME_PROCEXIT_E | PPME_PROCEXIT_1_E => zinfo.parse_thread_exit(event, etype),
            PPME_SYSCALL_PIPE_X | PPME_SYSCALL_PIPE2_X => zinfo.parse_pipe_exit(event, etype),
            PPME_SOCKET_SOCKET_X => zinfo.parse_socket_exit(event),
            PPME_SOCKET_BIND_X => zinfo.parse_bind_exit(event),
            // PPME_SOCKET_CONNECT_E never arrives from modern_bpf (only captures exit events)
            PPME_SOCKET_CONNECT_X => zinfo.parse_connect_exit(event),
            PPME_SOCKET_ACCEPT_X
            | PPME_SOCKET_ACCEPT_5_X
            | PPME_SOCKET_ACCEPT4_X
            | PPME_SOCKET_ACCEPT4_5_X
            | PPME_SOCKET_ACCEPT4_6_X => zinfo.parse_accept_exit(event),
            PPME_SYSCALL_CLOSE_X => zinfo.parse_close_exit(event),
            PPME_SYSCALL_FCNTL_X => zinfo.parse_fcntl_exit(event),
            PPME_SYSCALL_EVENTFD_X | PPME_SYSCALL_EVENTFD2_X => {
                zinfo.parse_eventfd_eventfd2_exit(event, etype)
            }
            PPME_SYSCALL_CHDIR_X => zinfo.parse_chdir_exit(event),
            PPME_SYSCALL_FCHDIR_X => zinfo.parse_fchdir_exit(event),
            PPME_SYSCALL_GETCWD_X => zinfo.parse_getcwd_exit(event),
            PPME_SOCKET_SHUTDOWN_X => {
                // I don't think we need this, it's mostly for invoking callbacks for optional observers
                // and doesn't seem to affect thread/fd table state
                debug!("nothing to do for socket shutdown events: {:?}", etype);
                Ok(())
            }
            PPME_SYSCALL_DUP_X | PPME_SYSCALL_DUP_1_X | PPME_SYSCALL_DUP2_X
            | PPME_SYSCALL_DUP3_X => zinfo.parse_dup_exit(event, etype),
            PPME_SYSCALL_SIGNALFD_X | PPME_SYSCALL_SIGNALFD4_X => {
                zinfo.parse_single_param_fd_exit(event, etype, fd_types::SCAP_FD_SIGNALFD)
            }
            PPME_SYSCALL_TIMERFD_CREATE_X => {
                zinfo.parse_single_param_fd_exit(event, etype, fd_types::SCAP_FD_TIMERFD)
            }
            PPME_SYSCALL_INOTIFY_INIT_X | PPME_SYSCALL_INOTIFY_INIT1_X => {
                zinfo.parse_single_param_fd_exit(event, etype, fd_types::SCAP_FD_INOTIFY)
            }
            PPME_SYSCALL_BPF_2_X => {
                zinfo.parse_single_param_fd_exit(event, etype, fd_types::SCAP_FD_BPF)
            }
            PPME_SYSCALL_USERFAULTFD_X => {
                zinfo.parse_single_param_fd_exit(event, etype, fd_types::SCAP_FD_USERFAULTFD)
            }
            PPME_SYSCALL_IO_URING_SETUP_X => {
                zinfo.parse_single_param_fd_exit(event, etype, fd_types::SCAP_FD_IOURING)
            }
            PPME_SYSCALL_EPOLL_CREATE_X | PPME_SYSCALL_EPOLL_CREATE1_X => {
                zinfo.parse_single_param_fd_exit(event, etype, fd_types::SCAP_FD_EVENTPOLL)
            }
            PPME_SYSCALL_GETRLIMIT_X | PPME_SYSCALL_SETRLIMIT_X => {
                zinfo.parse_getrlimit_setrlimit_exit(event)
            }
            PPME_SYSCALL_PRLIMIT_X => zinfo.parse_prlimit_exit(event),
            PPME_SOCKET_SOCKETPAIR_X => zinfo.parse_socketpair_exit(event),
            PPME_SCHEDSWITCH_1_E | PPME_SCHEDSWITCH_6_E => zinfo.parse_context_switch(event),
            PPME_SYSCALL_BRK_4_X
            | PPME_SYSCALL_MMAP_X
            | PPME_SYSCALL_MMAP2_X
            | PPME_SYSCALL_MUNMAP_X => zinfo.parse_brk_mmap_mmap2_munmap_exit(event),
            PPME_SYSCALL_SETRESUID_X => zinfo.parse_uid_exit(event, 2),
            PPME_SYSCALL_SETREUID_X => zinfo.parse_uid_exit(event, 2),
            PPME_SYSCALL_SETUID_X => zinfo.parse_uid_exit(event, 1),
            PPME_SYSCALL_SETRESGID_X => zinfo.parse_gid_exit(event, 2),
            PPME_SYSCALL_SETREGID_X => zinfo.parse_gid_exit(event, 2),
            PPME_SYSCALL_SETGID_X => zinfo.parse_gid_exit(event, 1),
            PPME_CPU_HOTPLUG_E => {
                // libsinsp effectively throws an exception and stops event processingg when this is detected.
                // that's because this can invalidate the thread data.
                // in our case we can bubble this up, and re-initialize zone event streaming.
                Err(ProcessingError::ZoneCPUHotplug(
                    "detected CPU hotplug event, event tracking must be reinitialized".to_string(),
                ))?
            }
            PPME_SYSCALL_CHROOT_X => zinfo.parse_chroot_exit(event),
            PPME_SYSCALL_SETSID_X => zinfo.parse_setsid_exit(event),
            PPME_SOCKET_GETSOCKOPT_X => {
                if !event.event_params.is_empty() {
                    zinfo.parse_getsockopt_exit(event)
                } else {
                    Ok(())
                }
            }
            PPME_SYSCALL_CAPSET_X => zinfo.parse_capset_exit(event),
            PPME_USER_ADDED_E | PPME_USER_DELETED_E => {
                Ok(())
                // all this does in libsinsp is update the internal userlist,
                // which ATM we do not track.
            }
            PPME_GROUP_ADDED_E | PPME_GROUP_DELETED_E => {
                Ok(())
                // all this does in libsinsp is update the internal grouplist,
                // which ATM we do not track.
            }
            PPME_SYSCALL_PRCTL_X => zinfo.parse_prctl_exit(event),
            _ => {
                trace!("unhandled syscall: {:?}", event);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::generated::protect::control::v1::{
        ZoneKernelEventParam, ZoneKernelFdInfo, ZoneKernelFdInfoData, ZoneKernelIpv4SocketInfo,
        ZoneKernelSyscallEvent, ZoneKernelThreadInfo, ZoneKernelThreadSnapshotEvent,
    };
    use std::collections::HashMap;

    fn create_test_event(zone_id: &str, thread_id: u64, event_type: u32) -> ZoneKernelSyscallEvent {
        ZoneKernelSyscallEvent {
            zone_id: zone_id.to_string(),
            thread_id,
            event_type,
            timestamp: 1000,
            event_name: "test".to_string(),
            event_params: vec![],
            ..Default::default()
        }
    }

    fn create_param(param_type: u32, data: Vec<u8>, pretty: String) -> ZoneKernelEventParam {
        ZoneKernelEventParam {
            param_type,
            param_data: data,
            param_pretty: pretty,
            ..Default::default()
        }
    }

    fn create_thread_info(tid: u64, pid: u64) -> ZoneKernelThreadInfo {
        ZoneKernelThreadInfo {
            tid: Some(tid),
            pid: Some(pid),
            ptid: Some(1),
            flags: 0,
            fdlist: HashMap::new(),
            cwd: "/".to_string(),
            ..Default::default()
        }
    }

    fn create_fd_info(fdi: u64) -> ZoneKernelFdInfo {
        ZoneKernelFdInfo {
            fd: Some(fdi),
            ..Default::default()
        }
    }

    #[test]
    fn init_zone_with_snap() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        thread_info.insert(100, create_thread_info(100, 100));

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        assert!(state.zone_info.contains_key(zone_id));
        assert_eq!(state.zone_info.get(zone_id).unwrap().boot_epoch, 5000);
        assert_eq!(
            state.zone_info.get(zone_id).unwrap().seen_threadsnaps.len(),
            1
        );
    }

    #[test]
    fn init_zone_replaces_existing() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info1 = HashMap::new();
        thread_info1.insert(100, create_thread_info(100, 100));

        let snap1 = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info: thread_info1,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap1);

        // Second snapshot should replace
        let mut thread_info2 = HashMap::new();
        thread_info2.insert(200, create_thread_info(200, 200));

        let snap2 = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info: thread_info2,
            zone_boot_epoch: 10000,
        };

        state.init_zone_with_snap(snap2);

        let zone = state.zone_info.get(zone_id).unwrap();
        assert_eq!(zone.boot_epoch, 10000);
        assert_eq!(zone.seen_threadsnaps.len(), 1);
        assert!(zone.seen_threadsnaps.contains_key(&200));
        assert!(!zone.seen_threadsnaps.contains_key(&100));
    }

    #[test]
    fn drop_zone_from_snap() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        thread_info.insert(100, create_thread_info(100, 100));

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);
        assert!(state.zone_info.contains_key(zone_id));

        state.drop_zone_from_snap(zone_id);
        assert!(!state.zone_info.contains_key(zone_id));
    }

    #[test]
    fn with_threadinfo_returns_none_when_missing() {
        let state = ThreadState::default();
        let result = state.with_threadinfo("test-zone", &100, |_| Some(42));
        assert_eq!(result, None);
    }

    #[test]
    fn with_threadinfo_executes_closure() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        thread_info.insert(100, create_thread_info(100, 100));

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let result = state.with_threadinfo(zone_id, &100, |tinfo| Some(tinfo.tid.unwrap()));

        assert_eq!(result, Some(100));
    }

    #[test]
    fn maybe_get_event_fdinfo_returns_none_no_thread() {
        let zinfo = ZoneInfo::default();
        let mut event =
            create_test_event("test-zone", 100, event_codes::PPME_SYSCALL_CLOSE_X as u32);
        event.event_params.push(create_param(
            param_type::PT_FD as u32,
            vec![0; 8],
            "".to_string(),
        ));

        let result = zinfo.maybe_get_event_fdinfo(&event);
        assert!(result.is_none());
    }

    #[test]
    fn maybe_get_event_fdinfo_returns_none_no_fd() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        thread_info.insert(100, create_thread_info(100, 100));

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        // Event without FD param
        let event = create_test_event(zone_id, 100, event_codes::PPME_SYSCALL_OPEN_E as u32);

        let result = zinfo.maybe_get_event_fdinfo(&event);
        assert!(result.is_none());
    }

    #[test]
    fn mark_thread_dead() {
        let mut tinfo = create_thread_info(100, 100);
        assert!(!parsers::thread_dead(&tinfo));

        ZoneInfo::mark_thread_dead(&mut tinfo);
        assert!(parsers::thread_dead(&tinfo));
    }

    #[test]
    fn mark_thread_undead() {
        let mut tinfo = create_thread_info(100, 100);
        ZoneInfo::mark_thread_dead(&mut tinfo);
        assert!(parsers::thread_dead(&tinfo));

        ZoneInfo::mark_thread_undead(&mut tinfo);
        assert!(!parsers::thread_dead(&tinfo));
    }

    #[test]
    fn set_role_server() {
        let mut fdinfo = ZoneKernelFdInfo::default();
        assert_eq!(fdinfo.state_flags & FLAGS_ROLE_SERVER, 0);

        ZoneInfo::set_role_server(&mut fdinfo);
        assert_ne!(fdinfo.state_flags & FLAGS_ROLE_SERVER, 0);
    }

    #[test]
    fn set_role_client() {
        let mut fdinfo = ZoneKernelFdInfo::default();
        assert_eq!(fdinfo.state_flags & FLAGS_ROLE_CLIENT, 0);

        ZoneInfo::set_role_client(&mut fdinfo);
        assert_ne!(fdinfo.state_flags & FLAGS_ROLE_CLIENT, 0);
    }

    #[test]
    fn set_socket_connected() {
        let mut fdinfo = ZoneKernelFdInfo::default();
        fdinfo.state_flags = FLAGS_CONNECTION_PENDING | FLAGS_CONNECTION_FAILED;

        ZoneInfo::set_socket_connected(&mut fdinfo);

        assert_eq!(fdinfo.state_flags & FLAGS_CONNECTION_PENDING, 0);
        assert_eq!(fdinfo.state_flags & FLAGS_CONNECTION_FAILED, 0);
        assert_ne!(fdinfo.state_flags & FLAGS_SOCKET_CONNECTED, 0);
    }

    #[test]
    fn set_socket_failed() {
        let mut fdinfo = ZoneKernelFdInfo::default();
        fdinfo.state_flags = FLAGS_SOCKET_CONNECTED | FLAGS_CONNECTION_PENDING;

        ZoneInfo::set_socket_failed(&mut fdinfo);

        assert_eq!(fdinfo.state_flags & FLAGS_SOCKET_CONNECTED, 0);
        assert_eq!(fdinfo.state_flags & FLAGS_CONNECTION_PENDING, 0);
        assert_ne!(fdinfo.state_flags & FLAGS_CONNECTION_FAILED, 0);
    }

    #[test]
    fn swap_addresses_ipv4() {
        let mut fdinfo = ZoneKernelFdInfo {
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::Ipv4Socket(ZoneKernelIpv4SocketInfo {
                    source_ip: "192.168.1.1".to_string(),
                    dest_ip: "10.0.0.1".to_string(),
                    source_port: 8080,
                    dest_port: 80,
                    protocol: 0,
                })),
            }),
            ..Default::default()
        };

        ZoneInfo::swap_addresses(&mut fdinfo);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::Ipv4Socket(socket)),
        }) = &fdinfo.info
        {
            assert_eq!(socket.source_ip, "10.0.0.1");
            assert_eq!(socket.dest_ip, "192.168.1.1");
            assert_eq!(socket.source_port, 80);
            assert_eq!(socket.dest_port, 8080);
        } else {
            panic!("Expected Ipv4Socket");
        }
    }

    #[test]
    fn populate_vmem_params_from_event_success() {
        let mut tinfo = create_thread_info(100, 100);
        let mut event = create_test_event(
            "test-zone",
            100,
            event_codes::PPME_SYSCALL_CLONE_20_X as u32,
        );

        // Add required params at indices 8-12
        for _ in 0..8 {
            event
                .event_params
                .push(create_param(0, vec![0; 8], "".to_string()));
        }

        event.event_params.push(create_param(
            0,
            1000u64.to_ne_bytes().to_vec(),
            "".to_string(),
        )); // pfmajor
        event.event_params.push(create_param(
            0,
            2000u64.to_ne_bytes().to_vec(),
            "".to_string(),
        )); // pfminor
        event.event_params.push(create_param(
            0,
            3000u32.to_ne_bytes().to_vec(),
            "".to_string(),
        )); // vmsize_kb
        event.event_params.push(create_param(
            0,
            4000u32.to_ne_bytes().to_vec(),
            "".to_string(),
        )); // vmrss_kb
        event.event_params.push(create_param(
            0,
            5000u32.to_ne_bytes().to_vec(),
            "".to_string(),
        )); // vmswap_kb

        let result = ZoneInfo::populate_vmem_params_from_event(&mut tinfo, &event);

        assert!(result.is_ok());
        assert_eq!(tinfo.pfmajor, 1000);
        assert_eq!(tinfo.pfminor, 2000);
        assert_eq!(tinfo.vmsize_kb, 3000);
        assert_eq!(tinfo.vmrss_kb, 4000);
        assert_eq!(tinfo.vmswap_kb, 5000);
    }

    #[test]
    fn populate_vmem_params_from_event_insufficient_params() {
        let mut tinfo = create_thread_info(100, 100);
        let event = create_test_event(
            "test-zone",
            100,
            event_codes::PPME_SYSCALL_CLONE_20_X as u32,
        );

        let result = ZoneInfo::populate_vmem_params_from_event(&mut tinfo, &event);
        assert!(result.is_err());
    }

    #[test]
    fn populate_command_info_from_event_success() {
        let mut tinfo = create_thread_info(100, 100);
        let mut event = create_test_event(
            "test-zone",
            100,
            event_codes::PPME_SYSCALL_EXECVE_19_X as u32,
        );

        // Add exe param at index 1
        event
            .event_params
            .push(create_param(0, vec![0; 8], "".to_string())); // placeholder index 0
        event
            .event_params
            .push(create_param(0, b"/bin/bash\0".to_vec(), "".to_string())); // exe

        // Add args param at index 2
        let args = b"bash\0-c\0echo test\0".to_vec();
        event
            .event_params
            .push(create_param(0, args, "".to_string()));

        let result = ZoneInfo::populate_command_info_from_event(&mut tinfo, &event);

        assert!(result.is_ok());
        assert_eq!(tinfo.exe, "/bin/bash");
        assert!(tinfo.args.contains("bash"));
        assert!(tinfo.args.contains("echo test"));
    }

    #[test]
    fn mark_fd_dropped() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        thread_info.insert(100, create_thread_info(100, 100));

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        zinfo.mark_fd_dropped(&100, &5);

        let dropped = state
            .zone_info
            .get(zone_id)
            .unwrap()
            .dropped_fds_by_thread
            .get(&100);
        assert_eq!(dropped, Some(&5));
    }

    #[test]
    fn with_mut_threadsnaps_ctx_executes_closure() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let thread_info = HashMap::new();
        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        let result = zinfo.with_mut_threadsnaps_ctx(|threads| {
            threads.insert(100, create_thread_info(100, 100));
            Some(threads.len())
        });

        assert_eq!(result, Some(1));
    }

    #[test]
    fn with_mut_threadinfo_ctx_none_for_missing_thread() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let thread_info = HashMap::new();
        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        let event = create_test_event(zone_id, 100, 0);
        let result = zinfo.with_mut_threadinfo_ctx(&event, |_| Some(42));
        assert_eq!(result, None);
    }

    #[test]
    fn with_threadinfo_ctx_none_for_missing_thread() {
        let zinfo = ZoneInfo::default();
        let event = create_test_event("test-zone", 100, 0);
        let result = zinfo.with_threadinfo_ctx(&event, |_| Some(42));
        assert_eq!(result, None);
    }

    #[test]
    fn with_mut_fdinfo_ctx_none_for_missing_fd() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        let tinfo = create_thread_info(100, 100);
        thread_info.insert(100, tinfo.clone());

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        let mut event = create_test_event(zone_id, 100, event_codes::PPME_SYSCALL_CLOSE_X as u32);
        event.event_params.push(create_param(
            param_type::PT_FD as u32,
            5i64.to_ne_bytes().to_vec(),
            "".to_string(),
        ));

        let result = zinfo.with_mut_leader_fdinfo_ctx(&tinfo, &0, |_| Some(42));
        assert_eq!(result, None);
    }

    #[test]
    fn with_mut_leader_fdlist_ctx_mutates_leader_fdlist_only() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        let leader = create_thread_info(100, 100);
        let child = create_thread_info(101, 100);
        thread_info.insert(100, leader.clone());
        thread_info.insert(101, child.clone());

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        let mut event = create_test_event(zone_id, 101, event_codes::PPME_SYSCALL_CLOSE_X as u32);
        event.event_params.push(create_param(
            param_type::PT_FD as u32,
            5i64.to_ne_bytes().to_vec(),
            "".to_string(),
        ));

        zinfo.with_mut_leader_fdlist_ctx(&event, |leader_fdlist| {
            let fdi = 22;
            let fdinfo = create_fd_info(fdi);
            leader_fdlist.insert(fdi, fdinfo)
        });

        // when using event from child thread, child threadinfo ctx should have no fds
        zinfo.with_mut_threadinfo_ctx(&event, |tinfo_child| {
            assert_eq!(tinfo_child.fdlist.len(), 0);
            Some(())
        });

        // but parent should have one
        zinfo
            .seen_threadsnaps
            .get(&leader.tid())
            .and_then(|leader_tinfo| {
                assert_eq!(leader_tinfo.fdlist.len(), 1);
                Some(())
            });
    }

    #[test]
    fn add_socket_fd_ipv4() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        thread_info.insert(100, create_thread_info(100, 100));

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");
        let event = create_test_event(zone_id, 100, 0);
        zinfo.add_socket_fd(
            &event,
            Some(5),
            ppm_consts::PPM_AF_INET,
            SOCK_STREAM,
            IPPROTO_TCP,
        );

        let fdinfo = zinfo.with_mut_leader_fdlist_ctx(&event, |fdlist| fdlist.get(&5).cloned());

        assert!(fdinfo.is_some());
        let fdinfo = fdinfo.unwrap();
        assert_eq!(fdinfo.fd, Some(5));
        assert_eq!(fdinfo.fd_type, fd_types::SCAP_FD_IPV4_SOCK as i32);
    }

    #[test]
    fn add_socket_fd_unix() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();
        thread_info.insert(100, create_thread_info(100, 100));

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        let event = create_test_event(zone_id, 100, 0);
        zinfo.add_socket_fd(&event, Some(5), ppm_consts::PPM_AF_UNIX, 0, 0);

        let fdinfo = zinfo.with_threadinfo_ctx(&event, |tinfo| tinfo.fdlist.get(&5).cloned());

        assert!(fdinfo.is_some());
        let fdinfo = fdinfo.unwrap();
        assert_eq!(fdinfo.fd, Some(5));
        assert_eq!(fdinfo.fd_type, fd_types::SCAP_FD_UNIX_SOCK as i32);
    }

    #[test]
    fn purge_other_dead_threads_removes_dead_threads() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();

        let live_thread = create_thread_info(100, 100);
        thread_info.insert(100, live_thread.clone());

        let mut dead_thread = create_thread_info(200, 200);
        ZoneInfo::mark_thread_dead(&mut dead_thread);
        thread_info.insert(200, dead_thread);

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        // Create event for live thread
        let event = create_test_event(zone_id, 100, 0);
        zinfo.purge_other_dead_threads(&event);

        // Live thread should still exist
        assert!(
            state
                .zone_info
                .get(zone_id)
                .unwrap()
                .seen_threadsnaps
                .contains_key(&100)
        );
        // Dead thread should be removed
        assert!(
            !state
                .zone_info
                .get(zone_id)
                .unwrap()
                .seen_threadsnaps
                .contains_key(&200)
        );
    }

    #[test]
    fn purge_other_dead_threads_keeps_current_thread_even_if_dead() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();

        // Add dead thread that is the current event's thread
        let mut dead_thread = create_thread_info(100, 100);
        ZoneInfo::mark_thread_dead(&mut dead_thread);
        thread_info.insert(100, dead_thread);

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        // Create event for the dead thread
        let event = create_test_event(zone_id, 100, 0);
        zinfo.purge_other_dead_threads(&event);

        // Dead thread should still exist because it's the current event's thread
        assert!(
            state
                .zone_info
                .get(zone_id)
                .unwrap()
                .seen_threadsnaps
                .contains_key(&100)
        );
    }

    #[test]
    fn purge_other_dead_threads_removes_orphaned_child_threads() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();

        // Add parent thread (dead)
        let mut parent_thread = create_thread_info(100, 100);
        ZoneInfo::mark_thread_dead(&mut parent_thread);
        thread_info.insert(100, parent_thread);

        // Add child thread with parent tid = 100
        let mut child_thread = create_thread_info(200, 100); // tid=200, pid=100
        child_thread.ptid = Some(100); // parent is 100
        thread_info.insert(200, child_thread);

        // Add unrelated live thread
        let live_thread = create_thread_info(300, 300);
        thread_info.insert(300, live_thread);

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        // Create event for unrelated thread
        let event = create_test_event(zone_id, 300, 0);
        zinfo.purge_other_dead_threads(&event);

        let zone = state.zone_info.get(zone_id).unwrap();

        assert!(!zone.seen_threadsnaps.contains_key(&100));
        assert!(!zone.seen_threadsnaps.contains_key(&200));
        assert!(zone.seen_threadsnaps.contains_key(&300));
    }

    #[test]
    fn purge_other_dead_threads_keeps_current_thread_even_if_orphaned() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();

        // Add parent thread (dead)
        let mut parent_thread = create_thread_info(100, 100);
        ZoneInfo::mark_thread_dead(&mut parent_thread);
        thread_info.insert(100, parent_thread);

        // Add child thread that will be orphaned
        let mut child_thread = create_thread_info(200, 100);
        child_thread.ptid = Some(100);
        thread_info.insert(200, child_thread);

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        // Create event for child thread (which will be orphaned)
        let event = create_test_event(zone_id, 200, 0);
        zinfo.purge_other_dead_threads(&event);

        let zone = state.zone_info.get(zone_id).unwrap();

        assert!(!zone.seen_threadsnaps.contains_key(&100));
        // Child should remain because it's the current event's thread
        assert!(zone.seen_threadsnaps.contains_key(&200));
    }

    #[test]
    fn purge_other_dead_threads_removes_orphaned_non_leader_threads() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();

        // Add thread leader (alive)
        let leader_thread = create_thread_info(100, 100);
        thread_info.insert(100, leader_thread);

        // Add dead child thread (pid != tid) with no parent (orphan)
        let mut orphan_thread = create_thread_info(200, 100); // tid=200, pid=100
        ZoneInfo::mark_thread_dead(&mut orphan_thread);
        orphan_thread.ptid = None; // No parent
        thread_info.insert(200, orphan_thread);

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        // Create event for leader thread
        let event = create_test_event(zone_id, 100, 0);
        zinfo.purge_other_dead_threads(&event);

        let zone = state.zone_info.get(zone_id).unwrap();

        // Leader should remain
        assert!(zone.seen_threadsnaps.contains_key(&100));
        // Orphan should be removed (dead, child thread, no parent)
        assert!(!zone.seen_threadsnaps.contains_key(&200));
    }

    #[test]
    fn purge_other_dead_threads_keeps_live_threads() {
        let mut state = ThreadState::default();
        let zone_id = "test-zone";

        let mut thread_info = HashMap::new();

        // Add multiple live threads
        for i in 100..105 {
            thread_info.insert(i, create_thread_info(i, i));
        }

        let snap = ZoneKernelThreadSnapshotEvent {
            zone_id: zone_id.to_string(),
            thread_info,
            zone_boot_epoch: 5000,
        };

        state.init_zone_with_snap(snap);

        let zinfo = state
            .zone_info
            .get_mut(zone_id)
            .expect("zone must be present");

        let event = create_test_event(zone_id, 100, 0);
        zinfo.purge_other_dead_threads(&event);

        let zone = state.zone_info.get(zone_id).unwrap();

        // All live threads should remain
        for i in 100..105 {
            assert!(zone.seen_threadsnaps.contains_key(&i));
        }
    }

    #[test]
    fn update_sockfd_from_tuple_ipv4() {
        let mut fdinfo = ZoneKernelFdInfo::default();

        // Build IPv4 tuple: family(1) + sip(4) + sport(2) + dip(4) + dport(2)
        let mut tuple_data = vec![ppm_consts::PPM_AF_INET as u8];
        // 127.0.0.1 in native byte order (the code uses from_ne_bytes)
        let ip_127_0_0_1 = u32::from(Ipv4Addr::new(127, 0, 0, 1));
        tuple_data.extend_from_slice(&ip_127_0_0_1.to_ne_bytes());
        tuple_data.extend_from_slice(&8080u16.to_ne_bytes());
        tuple_data.extend_from_slice(&ip_127_0_0_1.to_ne_bytes());
        tuple_data.extend_from_slice(&80u16.to_ne_bytes());

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);
        assert_eq!(fdinfo.fd_type, fd_types::SCAP_FD_IPV4_SOCK as i32);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::Ipv4Socket(socket)),
        }) = &fdinfo.info
        {
            assert_eq!(socket.source_ip, "127.0.0.1");
            assert_eq!(socket.dest_ip, "127.0.0.1");
            assert_eq!(socket.source_port, 8080);
            assert_eq!(socket.dest_port, 80);
        } else {
            panic!("Expected Ipv4Socket");
        }
    }

    #[test]
    fn update_sockfd_from_tuple_ipv4_updates_existing() {
        let mut fdinfo = ZoneKernelFdInfo {
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::Ipv4Socket(ZoneKernelIpv4SocketInfo {
                    source_ip: "192.168.1.1".to_string(),
                    dest_ip: "10.0.0.1".to_string(),
                    source_port: 9000,
                    dest_port: 443,
                    protocol: l4_types::SCAP_L4_TCP as i32,
                })),
            }),
            ..Default::default()
        };

        // Build new IPv4 tuple
        let mut tuple_data = vec![ppm_consts::PPM_AF_INET as u8];
        // 127.0.0.1 in native byte order (the code uses from_ne_bytes)
        let ip_127_0_0_1 = u32::from(Ipv4Addr::new(127, 0, 0, 1));
        tuple_data.extend_from_slice(&ip_127_0_0_1.to_ne_bytes());
        tuple_data.extend_from_slice(&8080u16.to_ne_bytes());
        tuple_data.extend_from_slice(&ip_127_0_0_1.to_ne_bytes());
        tuple_data.extend_from_slice(&80u16.to_ne_bytes());

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::Ipv4Socket(socket)),
        }) = &fdinfo.info
        {
            assert_eq!(socket.source_ip, "127.0.0.1");
            assert_eq!(socket.source_port, 8080);
            // Protocol should be preserved from original
            assert_eq!(socket.protocol, l4_types::SCAP_L4_TCP as i32);
        } else {
            panic!("Expected Ipv4Socket");
        }
    }

    #[test]
    fn update_sockfd_from_tuple_ipv6() {
        let mut fdinfo = ZoneKernelFdInfo::default();

        // Build IPv6 tuple: family(1) + sip(16) + sport(2) + dip(16) + dport(2)
        let mut tuple_data = vec![ppm_consts::PPM_AF_INET6 as u8];

        // ::1 (loopback)
        let ipv6_loopback = 1u128;
        tuple_data.extend_from_slice(&ipv6_loopback.to_ne_bytes());
        tuple_data.extend_from_slice(&8080u16.to_ne_bytes());
        tuple_data.extend_from_slice(&ipv6_loopback.to_ne_bytes());
        tuple_data.extend_from_slice(&80u16.to_ne_bytes());

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);
        assert_eq!(fdinfo.fd_type, fd_types::SCAP_FD_IPV6_SOCK as i32);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::Ipv6Socket(socket)),
        }) = &fdinfo.info
        {
            assert_eq!(socket.source_port, 8080);
            assert_eq!(socket.dest_port, 80);
        } else {
            panic!("Expected Ipv6Socket");
        }
    }

    #[test]
    fn update_sockfd_from_tuple_ipv6_mapped_ipv4() {
        let mut fdinfo = ZoneKernelFdInfo::default();

        // Build IPv6 tuple with IPv4-mapped addresses: ::ffff:127.0.0.1
        let mut tuple_data = vec![ppm_consts::PPM_AF_INET6 as u8];

        // This needs to be in the correct byte order for Ipv6Addr::from
        let ipv6_addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 0x0001);
        tuple_data.extend_from_slice(&u128::from(ipv6_addr).to_ne_bytes());
        tuple_data.extend_from_slice(&8080u16.to_ne_bytes());
        tuple_data.extend_from_slice(&u128::from(ipv6_addr).to_ne_bytes());
        tuple_data.extend_from_slice(&80u16.to_ne_bytes());

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);
        // Should be converted to IPv4
        assert_eq!(fdinfo.fd_type, fd_types::SCAP_FD_IPV4_SOCK as i32);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::Ipv4Socket(socket)),
        }) = &fdinfo.info
        {
            assert_eq!(socket.source_port, 8080);
            assert_eq!(socket.dest_port, 80);
            assert!(socket.source_ip.contains("127.0.0.1"));
        } else {
            panic!("Expected Ipv4Socket after v4-mapped conversion");
        }
    }

    #[test]
    fn update_sockfd_from_tuple_unix_socket() {
        let mut fdinfo = ZoneKernelFdInfo::default();

        // Build Unix socket tuple: family(1) + source(8) + dest(8) + name(null-terminated)
        let mut tuple_data = vec![ppm_consts::PPM_AF_UNIX as u8];
        tuple_data.extend_from_slice(&12345u64.to_ne_bytes());
        tuple_data.extend_from_slice(&67890u64.to_ne_bytes());
        tuple_data.extend_from_slice(b"/tmp/socket\0");

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);
        assert_eq!(fdinfo.fd_type, fd_types::SCAP_FD_UNIX_SOCK as i32);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::UnixSocket(socket)),
        }) = &fdinfo.info
        {
            assert_eq!(socket.source, 12345);
            assert_eq!(socket.destination, 67890);
            assert_eq!(socket.name, "/tmp/socket");
        } else {
            panic!("Expected UnixSocket");
        }
    }

    #[test]
    fn update_sockfd_from_tuple_unix_socket_updates_existing() {
        let mut fdinfo = ZoneKernelFdInfo {
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::UnixSocket(ZoneKernelUnixSocketInfo {
                    source: 111,
                    destination: 222,
                    name: "/old/path".to_string(),
                })),
            }),
            ..Default::default()
        };

        // Build new Unix socket tuple
        let mut tuple_data = vec![ppm_consts::PPM_AF_UNIX as u8];
        tuple_data.extend_from_slice(&12345u64.to_ne_bytes());
        tuple_data.extend_from_slice(&67890u64.to_ne_bytes());
        tuple_data.extend_from_slice(b"/new/socket\0");

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::UnixSocket(socket)),
        }) = &fdinfo.info
        {
            assert_eq!(socket.source, 12345);
            assert_eq!(socket.destination, 67890);
            assert_eq!(socket.name, "/new/socket");
        } else {
            panic!("Expected UnixSocket");
        }
    }

    #[test]
    fn update_sockfd_from_tuple_insufficient_data_ipv4() {
        let mut fdinfo = ZoneKernelFdInfo::default();

        // Insufficient data for IPv4 (need at least 13 bytes)
        let tuple_data = vec![ppm_consts::PPM_AF_INET as u8, 0, 1, 2, 3]; // Too short
        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(!result); // Should fail
    }

    #[test]
    fn update_sockfd_from_tuple_insufficient_data_ipv6() {
        let mut fdinfo = ZoneKernelFdInfo::default();

        // Insufficient data for IPv6 (need at least 37 bytes)
        let tuple_data = vec![ppm_consts::PPM_AF_INET6 as u8, 0, 1, 2]; // Too short
        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(!result); // Should fail
    }

    #[test]
    fn update_sockfd_from_tuple_insufficient_data_unix() {
        let mut fdinfo = ZoneKernelFdInfo::default();

        // Insufficient data for Unix socket (need at least 18 bytes)
        let tuple_data = vec![ppm_consts::PPM_AF_UNIX as u8, 0, 1]; // Too short
        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(!result); // Should fail
    }

    #[test]
    fn update_sockfd_from_tuple_ipv4_sets_udp_for_unknown_protocol() {
        let mut fdinfo = ZoneKernelFdInfo {
            fd_type: fd_types::SCAP_FD_IPV4_SOCK as i32,
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::Ipv4Socket(ZoneKernelIpv4SocketInfo {
                    source_ip: "0.0.0.0".to_string(),
                    dest_ip: "0.0.0.0".to_string(),
                    source_port: 0,
                    dest_port: 0,
                    protocol: l4_types::SCAP_L4_UNKNOWN as i32,
                })),
            }),
            ..Default::default()
        };

        // Build IPv4 tuple
        let mut tuple_data = vec![ppm_consts::PPM_AF_INET as u8];
        tuple_data.extend_from_slice(&0x0100007Fu32.to_ne_bytes());
        tuple_data.extend_from_slice(&8080u16.to_ne_bytes());
        tuple_data.extend_from_slice(&0x0100007Fu32.to_ne_bytes());
        tuple_data.extend_from_slice(&80u16.to_ne_bytes());

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);

        if let Some(ZoneKernelFdInfoData {
            info_type: Some(InfoType::Ipv4Socket(socket)),
        }) = &fdinfo.info
        {
            // Unknown protocol should be set to UDP
            assert_eq!(socket.protocol, l4_types::SCAP_L4_UDP as i32);
        } else {
            panic!("Expected Ipv4Socket");
        }
    }

    #[test]
    fn update_sockfd_from_tuple_converts_server_socket_to_regular() {
        let mut fdinfo = ZoneKernelFdInfo {
            fd_type: fd_types::SCAP_FD_IPV4_SERVSOCK as i32,
            info: Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::Ipv4ServerSocket(
                    crate::proto::generated::protect::control::v1::ZoneKernelIpv4ServerSocketInfo {
                        local_ip: "0.0.0.0".to_string(),
                        local_port: 8080,
                        protocol: l4_types::SCAP_L4_TCP as i32,
                    },
                )),
            }),
            ..Default::default()
        };

        // Build IPv4 tuple
        let mut tuple_data = vec![ppm_consts::PPM_AF_INET as u8];
        tuple_data.extend_from_slice(&0x0100007Fu32.to_ne_bytes());
        tuple_data.extend_from_slice(&8080u16.to_ne_bytes());
        tuple_data.extend_from_slice(&0x0100007Fu32.to_ne_bytes());
        tuple_data.extend_from_slice(&80u16.to_ne_bytes());

        let param = create_param(0, tuple_data, "".to_string());

        let result = ZoneInfo::update_sockfd_from_tuple(&mut fdinfo, &param);

        assert!(result);
        // Should be converted from server socket to regular socket
        assert_eq!(fdinfo.fd_type, fd_types::SCAP_FD_IPV4_SOCK as i32);

        // Should have regular socket info now
        assert!(matches!(
            fdinfo.info,
            Some(ZoneKernelFdInfoData {
                info_type: Some(InfoType::Ipv4Socket(_)),
            })
        ));
    }
}
