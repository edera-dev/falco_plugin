use crate::proto::generated::protect::control::v1::ZoneKernelFdInfo;
use crate::proto::generated::protect::control::v1::{
    ZoneKernelSyscallEvent, ZoneKernelThreadInfo, zone_kernel_fd_info_data::InfoType,
};
use anyhow::{Result, anyhow};
use libscap_bindings::consts as ppm_consts;
use libscap_bindings::types::{
    ppm_event_code as event_codes, ppm_event_flags as event_flags, scap_l4_proto as l4_types,
};
use log::error;
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

#[derive(PartialEq)]
pub enum OpenType {
    Read,
    Write,
    Exec,
    Create,
}

pub fn get_openstate(evt: &ZoneKernelSyscallEvent) -> Result<OpenType> {
    // strum from discriminant to make this simpler
    let etype =
        event_codes::from_repr(evt.event_type).ok_or(anyhow!("could not parse event type"))?;

    if is_open_file(etype) {
        let is_new_version = etype == event_codes::PPME_SYSCALL_OPENAT_2_X
            || etype == event_codes::PPME_SYSCALL_OPENAT2_X;
        // new versions have open flags at arg 3 insted of arg 2
        let flags = if is_new_version {
            u32::from_ne_bytes(evt.event_params[3].param_data.as_slice().try_into()?)
        } else {
            u32::from_ne_bytes(evt.event_params[2].param_data.as_slice().try_into()?)
        };

        if (flags & ppm_consts::PPM_O_RDONLY) != 0 {
            return Ok(OpenType::Read);
        } else if (flags & ppm_consts::PPM_O_WRONLY) != 0 {
            return Ok(OpenType::Write);
        } else if (flags & ppm_consts::PPM_O_F_CREATED) != 0 {
            return Ok(OpenType::Create);
        } else if (flags & (ppm_consts::PPM_O_TMPFILE | ppm_consts::PPM_O_CREAT)) != 0
            && etype != event_codes::PPME_SYSCALL_OPEN_BY_HANDLE_AT_X
        {
            let mode_bits = if is_new_version {
                u32::from_ne_bytes(evt.event_params[4].param_data.as_slice().try_into()?)
            } else {
                u32::from_ne_bytes(evt.event_params[3].param_data.as_slice().try_into()?)
            };
            if (mode_bits
                & (ppm_consts::PPM_S_IXUSR | ppm_consts::PPM_S_IXGRP | ppm_consts::PPM_S_IXOTH))
                != 0
            {
                return Ok(OpenType::Exec);
            }
        }
    }

    Err(anyhow!("not an open event"))
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
    (evt.event_flags & (event_flags::EF_USES_FD as u32)) != 0
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
    // * it is a syscall event.
    // * it is an exit event.
    // * it has at least one parameter. Some exit events are not instrumented, see
    // `PPME_SOCKET_GETSOCKNAME_X`

    if evt.event_category.contains("SYSCALL") && !is_enter(evt) && !evt.event_params.is_empty() {
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

/// If this event is the kind of syscall event (enter types, some others) that encodes an FD,
/// return the FD number (for table lookup, etc).
/// Note that unlike libsinsp's equivalents, this can be passed an enter or exit event,
/// and will suck out the FDI (if present) in either case.
pub fn get_fdi(event: &ZoneKernelSyscallEvent) -> Option<u64> {
    use event_codes::*;
    let etype = event_codes::from_repr(event.event_type)
        .ok_or(anyhow!("could not parse event type"))
        .expect("should parse");

    // most FDs come in the enter event
    let maybe_fd_loc: Option<usize> = if is_enter(event) {
        if has_fd(event) {
            match etype {
                PPME_SYSCALL_MMAP_E | PPME_SYSCALL_MMAP2_E => Some(4),

                PPME_SYSCALL_SPLICE_E => Some(1),
                _ => {
                    Some(0) // *most* of the time, the 0th param is the fd
                }
            }
        } else {
            None
        }
    // sendmmsg and recvmmsg send all data in the EXIT event, fd included.
    } else if (etype == PPME_SOCKET_SENDMMSG_X || etype == PPME_SOCKET_RECVMMSG_X)
        && !event.event_params.is_empty()
    {
        Some(1)
    } else {
        None
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

pub fn get_enter_event_fd_loc(event: &ZoneKernelSyscallEvent, etype: event_codes) -> Option<u64> {
    use event_codes::*;
    if !is_enter(event) || !has_fd(event) {
        error!("not an enter event or has no FD! {:?}", event);
        return None;
    }

    match etype {
        PPME_SYSCALL_MMAP_E | PPME_SYSCALL_MMAP2_E => Some(4),
        PPME_SYSCALL_SPLICE_E => Some(1),
        _ => {
            // For almost all parameters the default position is `0`
            Some(0)
        }
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
