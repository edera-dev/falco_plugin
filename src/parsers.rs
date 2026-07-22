use crate::proto::generated::protect::control::v1::ZoneKernelFdInfo;
use crate::proto::generated::protect::control::v1::{
    ZoneKernelSyscallEvent, ZoneKernelThreadInfo, zone_kernel_fd_info_data::InfoType,
};
use anyhow::Result;
use libscap_bindings::consts as ppm_consts;
use libscap_bindings::types::{
    ppm_event_code as event_codes, ppm_event_flags as event_flags, ppm_param_type as param_type,
    scap_l4_proto as l4_types,
};
use std::ffi::CStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

// a handful of libsinsp/libscap consts
// duplicated here
const DIRECTION_MASK: u32 = 1;
const FLAGS_ROLE_CLIENT: u32 = 1 << 2;
const FLAGS_ROLE_SERVER: u32 = 1 << 3;
const FLAGS_SOCKET_CONNECTED: u32 = 1 << 13;
const FLAGS_OVERLAY_UPPER: u32 = 1 << 17;
const FLAGS_OVERLAY_LOWER: u32 = 1 << 18;

// Open access/create/exec classification mirrors libsinsp
// `evt.is_open_read/write/exec/create`.
const OPEN_EXEC_MODE_MASK: u32 =
    ppm_consts::PPM_S_IXUSR | ppm_consts::PPM_S_IXGRP | ppm_consts::PPM_S_IXOTH;

fn read_u32_param(evt: &ZoneKernelSyscallEvent, idx: usize) -> Option<u32> {
    let param = evt.event_params.get(idx)?;
    Some(u32::from_ne_bytes(
        param.param_data.as_slice().try_into().ok()?,
    ))
}

/// Normalized open flags for an open-family exit event (the `flags` param), or
/// `None` if this isn't such an event or the param is absent/malformed. The
/// modern openat variants carry flags at arg 3, the legacy forms at arg 2.
fn open_flags(evt: &ZoneKernelSyscallEvent, etype: event_codes) -> Option<u32> {
    let idx = match etype {
        event_codes::PPME_SYSCALL_OPENAT_2_X | event_codes::PPME_SYSCALL_OPENAT2_X => 3,
        event_codes::PPME_SYSCALL_OPEN_X | event_codes::PPME_SYSCALL_OPEN_BY_HANDLE_AT_X => 2,
        _ => return None,
    };
    read_u32_param(evt, idx)
}

pub fn is_open_read(evt: &ZoneKernelSyscallEvent) -> bool {
    let Some(etype) = event_codes::from_repr(evt.event_type) else {
        return false;
    };
    open_flags(evt, etype).is_some_and(|flags| (flags & ppm_consts::PPM_O_RDONLY) != 0)
}

pub fn is_open_write(evt: &ZoneKernelSyscallEvent) -> bool {
    let Some(etype) = event_codes::from_repr(evt.event_type) else {
        return false;
    };
    open_flags(evt, etype).is_some_and(|flags| (flags & ppm_consts::PPM_O_WRONLY) != 0)
}

pub fn is_open_create(evt: &ZoneKernelSyscallEvent) -> bool {
    let Some(etype) = event_codes::from_repr(evt.event_type) else {
        return false;
    };
    let Some(flags) = open_flags(evt, etype) else {
        return false;
    };
    // O_F_CREATED means the file was created; O_TMPFILE creates one only on success.
    (flags & ppm_consts::PPM_O_F_CREATED) != 0
        || ((flags & ppm_consts::PPM_O_TMPFILE) != 0 && get_retval(evt).is_some_and(|r| r >= 0))
}

pub fn is_open_exec(evt: &ZoneKernelSyscallEvent) -> bool {
    let Some(etype) = event_codes::from_repr(evt.event_type) else {
        return false;
    };
    // `creat` carries mode at arg 2; open-family carries it at arg 3 (legacy) or
    // arg 4 (modern), and only counts as exec when the open can create the file.
    // open_by_handle_at has no mode param and is excluded, as in libsinsp.
    let mode_idx = match etype {
        event_codes::PPME_SYSCALL_CREAT_X => 2,
        event_codes::PPME_SYSCALL_OPEN_X
        | event_codes::PPME_SYSCALL_OPENAT_2_X
        | event_codes::PPME_SYSCALL_OPENAT2_X => {
            let Some(flags) = open_flags(evt, etype) else {
                return false;
            };
            if (flags & (ppm_consts::PPM_O_TMPFILE | ppm_consts::PPM_O_CREAT)) == 0 {
                return false;
            }
            if etype == event_codes::PPME_SYSCALL_OPEN_X {
                3
            } else {
                4
            }
        }
        _ => return false,
    };
    read_u32_param(evt, mode_idx).is_some_and(|mode| (mode & OPEN_EXEC_MODE_MASK) != 0)
}

pub fn is_enter(evt: &ZoneKernelSyscallEvent) -> bool {
    (evt.event_type & DIRECTION_MASK) == 0
}

pub fn is_io(evt: &ZoneKernelSyscallEvent) -> bool {
    (evt.event_flags & (event_flags::EF_READS_FROM_FD as u32 | event_flags::EF_WRITES_TO_FD as u32))
        != 0
}

pub fn is_io_read(evt: &ZoneKernelSyscallEvent) -> bool {
    (evt.event_flags & (event_flags::EF_READS_FROM_FD as u32)) != 0
}

pub fn is_io_write(evt: &ZoneKernelSyscallEvent) -> bool {
    (evt.event_flags & (event_flags::EF_WRITES_TO_FD as u32)) != 0
}

pub fn is_wait(evt: &ZoneKernelSyscallEvent) -> bool {
    (evt.event_flags & (event_flags::EF_WAITS as u32)) != 0
}

pub fn has_fd(evt: &ZoneKernelSyscallEvent) -> bool {
    // Check event_flags first
    if (evt.event_flags & (event_flags::EF_USES_FD as u32)) != 0 {
        return true;
    }

    // Fallback: check if any parameter is of type PT_FD
    // This handles edge cases where event_flags might still not be populated
    evt.event_params
        .iter()
        .any(|param| param.param_type == param_type::PT_FD as u32)
}

pub fn reads_fd(evt: &ZoneKernelSyscallEvent) -> bool {
    (evt.event_flags & (event_flags::EF_READS_FROM_FD as u32)) != 0
}

pub fn is_open_file(etype: event_codes) -> bool {
    etype == event_codes::PPME_SYSCALL_OPEN_X
        || etype == event_codes::PPME_SYSCALL_OPENAT_X
        || etype == event_codes::PPME_SYSCALL_OPENAT_2_X
        || etype == event_codes::PPME_SYSCALL_OPENAT2_X
        || etype == event_codes::PPME_SYSCALL_OPEN_BY_HANDLE_AT_X
}

pub fn is_open_net(etype: event_codes) -> bool {
    etype == event_codes::PPME_SOCKET_ACCEPT_X
        || etype == event_codes::PPME_SOCKET_ACCEPT_5_X
        || etype == event_codes::PPME_SOCKET_ACCEPT4_X
        || etype == event_codes::PPME_SOCKET_ACCEPT4_5_X
        || etype == event_codes::PPME_SOCKET_ACCEPT4_6_X
        || etype == event_codes::PPME_SOCKET_CONNECT_X
}

pub fn is_tcp_socket(fdinfo: &ZoneKernelFdInfo) -> bool {
    matches!(
        fdinfo.info.as_ref().and_then(|i| i.info_type.as_ref()),
        Some(InfoType::Ipv4Socket(socket)) if socket.protocol == l4_types::SCAP_L4_TCP as i32
    )
}

pub fn is_role_none(fdinfo: &ZoneKernelFdInfo) -> bool {
    (fdinfo.state_flags & (FLAGS_ROLE_CLIENT | FLAGS_ROLE_SERVER)) == 0
}

pub fn is_socket_connected(fdinfo: &ZoneKernelFdInfo) -> bool {
    fdinfo.state_flags & FLAGS_SOCKET_CONNECTED != 0
}

pub fn is_upper_layer(fdinfo: &ZoneKernelFdInfo) -> bool {
    fdinfo.state_flags & FLAGS_OVERLAY_UPPER != 0
}

pub fn is_lower_layer(fdinfo: &ZoneKernelFdInfo) -> bool {
    fdinfo.state_flags & FLAGS_OVERLAY_LOWER != 0
}

/// See falcosecurity/libs/userspace/libsinsp/event.h
pub fn has_retval(evt: &ZoneKernelSyscallEvent) -> bool {
    if evt.event_type == event_codes::PPME_GENERIC_X as u32 {
        return false;
    }
    // The event has a return value if:
    // * it is a syscall event (or a syscall-like metaevent).
    // * it is an exit event.
    // * it has at least one parameter. Some exit events are not instrumented, see
    // `PPME_SOCKET_GETSOCKNAME_X`
    if (evt.event_category.contains("SYSCALL") || evt.event_category.contains("EC_METAEVENT"))
        && !is_enter(evt)
        && !evt.event_params.is_empty()
    {
        return true;
    }

    false
}

pub fn get_retval(evt: &ZoneKernelSyscallEvent) -> Option<i64> {
    if has_retval(evt) {
        Some(i64::from_ne_bytes(
            evt.event_params[0].param_data.as_slice().try_into().ok()?,
        ))
    } else {
        None
    }
}

/// libsinsp `evt.failed` / error-count semantics (see libsinsp
/// `sinsp_filter_check_event::extract_error_count`): a syscall failed iff it has
/// a return value and that value is negative. A non-negative value (including an
/// fd or a byte count) is a success. `None` means the event carries no return
/// value, which extracts as `<NA>`.
pub fn syscall_failed(evt: &ZoneKernelSyscallEvent) -> Option<bool> {
    get_retval(evt).map(|retval| retval < 0)
}

/// Extract FD number from an event.
///
/// Modern BPF driver only captures exit events, so this extracts FD from exit event parameters.
/// It searches for a parameter with type PT_FD and returns its value.
pub fn get_fdi(event: &ZoneKernelSyscallEvent) -> Option<u64> {
    use event_codes::*;

    if !has_fd(event) {
        return None;
    }

    let etype = event_codes::from_repr(event.event_type)?;

    // For exit events (modern_bpf only captures these), search for the FD parameter
    // Special case: sendmmsg and recvmmsg have FD at position 1
    let maybe_fd_loc: Option<usize> =
        if etype == PPME_SOCKET_SENDMMSG_X || etype == PPME_SOCKET_RECVMMSG_X {
            Some(1)
        } else {
            // For other exit events, search for the PT_FD parameter
            event
                .event_params
                .iter()
                .position(|param| param.param_type == param_type::PT_FD as u32)
        };

    if let Some(loc) = maybe_fd_loc {
        let res = i64::from_ne_bytes(
            event.event_params[loc]
                .param_data
                .as_slice()
                .try_into()
                .ok()?,
        );
        Some(res.try_into().ok()?)
    } else {
        None
    }
}

pub fn vec_contains_hex_bytes(byte_buf: &[u8], hex_matcher: &CStr) -> bool {
    let hex_bytes = hex_matcher.to_bytes();

    // bruh seems bad
    if !hex_bytes.len().is_multiple_of(2) || hex_bytes.is_empty() {
        return false;
    }

    let needle_len = hex_bytes.len() / 2;

    byte_buf.windows(needle_len).any(|window| {
        window
            .iter()
            .zip(hex_bytes.chunks_exact(2))
            .all(|(&byte, hex_pair)| {
                matches!(
                    (hex_char_to_nibble(hex_pair[0]), hex_char_to_nibble(hex_pair[1])),
                    (Some(h), Some(l)) if byte == (h << 4) | l
                )
            })
    })
}

#[inline]
pub fn hex_char_to_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

// libscap stores the "list of caps" as a bitmask in the driver,
// and libsinsp munges the bitmask on syscall events.
// This naturally requires that the discriminants/ordering of Linux caps are identical
// on both ends. Thankfully, the `caps` crate uses the exact same discriminants/ordering,
// (e.g. CAP_MAC_OVERRIDE is `32` in both worlds,
// see https://github.com/lucab/caps-rs/blob/master/src/nr.rs
// and falcosecurity/libs/driver/ppm_events_public.h)
// so we can just use that.
pub fn get_all_caps_bitmask() -> u64 {
    let mut bitmask = 0u64;

    // Get all capabilities from the caps crate
    for cap in caps::all() {
        let bit_pos = cap as u8;
        if bit_pos < 64 {
            // Ensure we don't overflow u64
            bitmask |= 1u64 << bit_pos;
        }
    }

    bitmask
}
pub fn thread_dead(tinfo: &ZoneKernelThreadInfo) -> bool {
    tinfo.flags & ppm_consts::PPM_CL_CLOSED != 0
}

pub fn thread_invalid(tinfo: &ZoneKernelThreadInfo) -> bool {
    tinfo.tid.is_none() || tinfo.pid.is_none() || tinfo.ptid.is_none()
}

pub fn thread_main(tinfo: &ZoneKernelThreadInfo) -> bool {
    (tinfo.tid.is_some() && tinfo.pid.is_some() && tinfo.tid == tinfo.pid)
        || (tinfo.flags & ppm_consts::PPM_CL_IS_MAIN_THREAD != 0)
}

// pub fn thread_in_pidns(tinfo: &ZoneKernelThreadInfo) -> bool {
//     (tinfo.flags & ppm_consts::PPM_CL_CHILD_IN_PIDNS != 0) || (tinfo.vtid.is_some() && tinfo.tid != tinfo.vtid)
// }

pub fn is_server_port(
    threadinfo: &ZoneKernelThreadInfo,
    sport: u32,
    dport: u32,
    incoming: bool,
) -> bool {
    if is_bound_to_port(threadinfo, dport) {
        false // client port
    } else if uses_client_port(threadinfo, sport) {
        true //server port
    } else {
        // libsinsp: We just assume that a server usually starts with a read and a client with a write.
        if incoming {
            true //server port
        } else {
            false //client port
        }
    }
}

pub fn is_bound_to_port(threadinfo: &ZoneKernelThreadInfo, portno: u32) -> bool {
    threadinfo.fdlist.iter().any(|(_, fdinfo)| {
        if let Some(info) = &fdinfo.info {
            match &info.info_type {
                Some(InfoType::Ipv4Socket(socket_info)) => socket_info.dest_port == portno,
                Some(InfoType::Ipv4ServerSocket(server_info)) => server_info.local_port == portno,
                _ => false,
            }
        } else {
            false
        }
    })
}

pub fn uses_client_port(threadinfo: &ZoneKernelThreadInfo, portno: u32) -> bool {
    threadinfo.fdlist.iter().any(|(_, fdinfo)| {
        if let Some(info) = &fdinfo.info {
            match &info.info_type {
                Some(InfoType::Ipv4Socket(socket_info)) => socket_info.source_port == portno,
                _ => false,
            }
        } else {
            false
        }
    })
}

/// Hits /etc/services to resolve "friendly names" for common proto/port combos.
/// Note that /etc/services can differ between machines, and we're checking ZONE events
/// against the HOST copy, so it's possible there may be a mismatch
/// (`/etc/services` can differ between machines/distros),
/// but that's (very) unlikely and harmless if it happens.
///
/// Returns the service name if found, otherwise returns "port/proto" (e.g., "80/tcp").
pub fn lookup_service(port: u32, proto: &str) -> String {
    let file = File::open("/etc/services").ok();
    if let Some(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let port_proto = parts[1].split('/').collect::<Vec<_>>();
                if port_proto.len() == 2
                    && port_proto[0] == port.to_string()
                    && port_proto[1] == proto
                {
                    return parts[0].to_string();
                }
            }
        }
    }

    format!("{}/{}", port, proto)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::generated::protect::control::v1::ZoneKernelEventParam;

    // Builds an exit event carrying `retval` as its first param, the way the
    // driver reports a syscall return value: a signed 64-bit two's-complement
    // value (negative on error). `event_type` must be an exit code.
    fn exit_event_with_retval(event_type: event_codes, retval: i64) -> ZoneKernelSyscallEvent {
        ZoneKernelSyscallEvent {
            event_type: event_type as u32,
            event_category: "EC_FILE | EC_SYSCALL".to_string(),
            event_params: vec![ZoneKernelEventParam {
                name: "fd".to_string(),
                param_data: retval.to_ne_bytes().to_vec(),
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[test]
    fn successful_open_is_not_failed() {
        // A successful open returns a positive fd, which must read as success -
        // this is the regression the old `retval != 0` check got wrong.
        let evt = exit_event_with_retval(event_codes::PPME_SYSCALL_OPENAT_2_X, 3);
        assert!(has_retval(&evt));
        assert_eq!(get_retval(&evt), Some(3));
        assert_eq!(syscall_failed(&evt), Some(false));
    }

    #[test]
    fn failed_open_enoent_is_failed() {
        let evt = exit_event_with_retval(event_codes::PPME_SYSCALL_OPENAT_2_X, -2);
        assert_eq!(get_retval(&evt), Some(-2));
        assert_eq!(syscall_failed(&evt), Some(true));
    }

    #[test]
    fn eagain_return_is_failed() {
        // EAGAIN arrives as a negative return (-11), so it reads as failed, in
        // line with libsinsp `evt.failed` (no errno is special-cased there).
        let evt = exit_event_with_retval(event_codes::PPME_SOCKET_CONNECT_X, -11);
        assert_eq!(get_retval(&evt), Some(-11));
        assert_eq!(syscall_failed(&evt), Some(true));
    }

    #[test]
    fn enter_event_has_no_retval() {
        // Enter events carry no return value; failure extracts as <NA> (None).
        let evt = exit_event_with_retval(event_codes::PPME_SYSCALL_OPENAT_2_E, 0);
        assert!(is_enter(&evt));
        assert!(!has_retval(&evt));
        assert_eq!(get_retval(&evt), None);
        assert_eq!(syscall_failed(&evt), None);
    }

    // constructs a modern openat exit event: params are [fd, dirfd, name, flags, mode].
    // fd=3 (success) so the O_TMPFILE create path is satisfied when exercised.
    fn openat2x_event(flags: u32, mode: u32) -> ZoneKernelSyscallEvent {
        let u32_param = |v: u32| ZoneKernelEventParam {
            param_data: v.to_ne_bytes().to_vec(),
            ..Default::default()
        };
        ZoneKernelSyscallEvent {
            event_type: event_codes::PPME_SYSCALL_OPENAT_2_X as u32,
            event_category: "EC_FILE | EC_SYSCALL".to_string(),
            event_params: vec![
                u32_param(3),
                u32_param(0),
                u32_param(0),
                u32_param(flags),
                u32_param(mode),
            ],
            ..Default::default()
        }
    }

    #[test]
    fn rdonly_open_is_read_only() {
        let evt = openat2x_event(ppm_consts::PPM_O_RDONLY, 0);
        assert!(is_open_read(&evt));
        assert!(!is_open_write(&evt));
    }

    #[test]
    fn wronly_open_is_write_only() {
        let evt = openat2x_event(ppm_consts::PPM_O_WRONLY, 0);
        assert!(!is_open_read(&evt));
        assert!(is_open_write(&evt));
    }

    #[test]
    fn rdwr_open_is_both_read_and_write() {
        let evt = openat2x_event(ppm_consts::PPM_O_RDWR, 0);
        assert!(is_open_read(&evt));
        assert!(is_open_write(&evt));
    }

    #[test]
    fn created_flag_is_create_independent_of_access_mode() {
        let evt = openat2x_event(ppm_consts::PPM_O_WRONLY | ppm_consts::PPM_O_F_CREATED, 0);
        assert!(is_open_create(&evt));
        assert!(is_open_write(&evt));
    }

    #[test]
    fn creating_open_with_exec_mode_is_exec() {
        let evt = openat2x_event(
            ppm_consts::PPM_O_WRONLY | ppm_consts::PPM_O_CREAT,
            ppm_consts::PPM_S_IXUSR,
        );
        assert!(is_open_exec(&evt));
    }

    #[test]
    fn plain_read_open_is_neither_create_nor_exec() {
        let evt = openat2x_event(ppm_consts::PPM_O_RDONLY, 0);
        assert!(!is_open_create(&evt));
        assert!(!is_open_exec(&evt));
    }

    #[test]
    fn get_fdi_unknown_event_type_does_not_panic() {
        // A malicious zone can send an arbitrary event_type, so classification must
        // return None rather than panic.
        let evt = ZoneKernelSyscallEvent {
            event_type: 0xDEAD_BEEF,
            event_flags: event_flags::EF_USES_FD as u32,
            ..Default::default()
        };
        assert!(has_fd(&evt));
        assert_eq!(get_fdi(&evt), None);
    }
}
