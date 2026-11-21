use anyhow::{Context, Result, anyhow};
use aya::maps::{Array, Map, MapData};
use libscap_bindings::types::ppm_sc_code as syscall_codes;
use log::{debug, info};
use std::collections::{HashMap, HashSet};
use std::fs;
use strum::IntoEnumIterator;
use syscalls::Sysno;

/// Falco (running on the host) does not pin the bpf map it creates on launch containing
/// the syscalls its filtering engine cares about.
/// So, we have to find the falco `pid` and scan `/proc/` on the host,
/// to figure out the correct bpf map id, and then filter by the name.
fn find_interesting_syscalls_map() -> Result<Map> {
    let falco_pid = find_process_by_name("falco")?;
    debug!("found Falco PID: {}", falco_pid);

    let maps = get_map_ids_for_pid(falco_pid)?;

    for map_id in maps {
        if let Ok(map_data) = MapData::from_id(map_id)
            && let Ok(info) = map_data.info()
        {
            let name = info
                .name_as_str()
                .ok_or(anyhow!("could not parse mapname"))?;

            debug!("found falco-owned map ID {}: {}", map_id, name);

            if name.contains("interesting_sys") {
                debug!("found falco enabled syscall map on host: {}", map_id);
                return Ok(Map::Array(map_data));
            }
        }
    }

    anyhow::bail!("map not found")
}

fn get_map_ids_for_pid(pid: u32) -> Result<Vec<u32>> {
    let fdinfo_dir = format!("/proc/{}/fdinfo", pid);
    let mut map_ids = Vec::new();

    for entry in
        fs::read_dir(&fdinfo_dir).context(format!("couldn't read procdir for pid{}", pid))?
    {
        let entry = entry?;

        if let Ok(fdinfo_content) = fs::read_to_string(entry.path()) {
            if !fdinfo_content.contains("map_id:") {
                continue;
            }

            for line in fdinfo_content.lines() {
                if let Some(val) = line.strip_prefix("map_id:")
                    && let Ok(id) = val.trim().parse::<u32>()
                {
                    map_ids.push(id);
                    break;
                }
            }
        }
    }

    Ok(map_ids)
}

fn find_process_by_name(name: &str) -> Result<u32> {
    for entry in fs::read_dir("/proc").context("couldn't read /proc")? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name()
            && let Some(pid_str) = file_name.to_str()
            && let Ok(pid) = pid_str.parse::<u32>()
        {
            let comm_path = path.join("comm");
            if let Ok(comm) = fs::read_to_string(comm_path)
                && comm.trim() == name
            {
                return Ok(pid);
            }
        }
    }

    anyhow::bail!(format!("Process '{}' not found", name))
}

pub fn read_enabled_syscalls() -> Result<HashSet<u32>> {
    let mut enabled = HashSet::new();
    let map = find_interesting_syscalls_map()?;

    // Convert Map enum to typed Array
    let array: Array<_, u8> =
        Array::try_from(map).context("couldn't read interesting syscalls map")?;

    // The map is indexed by the kernel's native syscall number, not by `ppm_sc` code.
    for native_id in 0u32..array.len() {
        if let Ok(value) = array.get(&native_id, 0)
            && value != 0
        {
            enabled.insert(native_id);
        }
    }

    let ppm_sc = native_to_ppm_sc(&enabled);
    info!(
        "read {} enabled native syscalls, {} as ppm_sc codes",
        enabled.len(),
        ppm_sc.len()
    );

    Ok(ppm_sc)
}

/// Converts native syscall numbers for this host's architecture to libscap `ppm_sc` codes.
/// libscap names each code after the kernel syscall (`PPM_SC_<NAME>`), so match by name.
fn native_to_ppm_sc(native: &HashSet<u32>) -> HashSet<u32> {
    let by_name: HashMap<String, u32> = syscall_codes::iter()
        .map(|code| (code.to_string(), code as u32))
        .collect();

    native
        .iter()
        .filter_map(|&native_id| {
            let sysno = Sysno::new(native_id as usize)?;
            let code = by_name
                .get(&format!("PPM_SC_{}", sysno.name().to_uppercase()))
                .copied();
            if code.is_none() {
                debug!(
                    "no ppm_sc code for native syscall {native_id} ({})",
                    sysno.name()
                );
            }
            code
        })
        .collect()
}

/// The syscall set we ask zones for when not mirroring the host's set: the zone's own
/// default set (protect `crates/zone/src/kernel_events.rs`, unchanged from v1.13.1 to
/// v1.13.4) plus process exit, which the default set lacks. Without process exit, the
/// thread table never drops a thread, so it grows for as long as Falco runs and stale
/// entries mislabel reused PIDs.
pub fn default_zone_syscalls() -> Vec<u32> {
    [
        syscall_codes::PPM_SC_ACCEPT,
        syscall_codes::PPM_SC_ACCEPT4,
        syscall_codes::PPM_SC_BIND,
        syscall_codes::PPM_SC_BPF,
        syscall_codes::PPM_SC_CAPSET,
        syscall_codes::PPM_SC_CHDIR,
        syscall_codes::PPM_SC_CHMOD,
        syscall_codes::PPM_SC_CHROOT,
        syscall_codes::PPM_SC_CLONE,
        syscall_codes::PPM_SC_CLONE3,
        syscall_codes::PPM_SC_CLOSE,
        syscall_codes::PPM_SC_CONNECT,
        syscall_codes::PPM_SC_COPY_FILE_RANGE,
        syscall_codes::PPM_SC_CREAT,
        syscall_codes::PPM_SC_DUP,
        syscall_codes::PPM_SC_DUP2,
        syscall_codes::PPM_SC_DUP3,
        syscall_codes::PPM_SC_EVENTFD,
        syscall_codes::PPM_SC_EVENTFD2,
        syscall_codes::PPM_SC_EXECVE,
        syscall_codes::PPM_SC_EXECVEAT,
        syscall_codes::PPM_SC_FCHDIR,
        syscall_codes::PPM_SC_FCHMOD,
        syscall_codes::PPM_SC_FCHMODAT,
        syscall_codes::PPM_SC_FCNTL,
        syscall_codes::PPM_SC_FLOCK,
        syscall_codes::PPM_SC_FORK,
        syscall_codes::PPM_SC_INOTIFY_INIT,
        syscall_codes::PPM_SC_INOTIFY_INIT1,
        syscall_codes::PPM_SC_IOCTL,
        syscall_codes::PPM_SC_KILL,
        syscall_codes::PPM_SC_LINK,
        syscall_codes::PPM_SC_LINKAT,
        syscall_codes::PPM_SC_LISTEN,
        syscall_codes::PPM_SC_MKDIR,
        syscall_codes::PPM_SC_MKDIRAT,
        syscall_codes::PPM_SC_MOUNT,
        syscall_codes::PPM_SC_OPEN,
        syscall_codes::PPM_SC_OPEN_BY_HANDLE_AT,
        syscall_codes::PPM_SC_OPENAT,
        syscall_codes::PPM_SC_OPENAT2,
        syscall_codes::PPM_SC_PIPE,
        syscall_codes::PPM_SC_PIPE2,
        syscall_codes::PPM_SC_PRLIMIT64,
        syscall_codes::PPM_SC_PTRACE,
        syscall_codes::PPM_SC_QUOTACTL,
        syscall_codes::PPM_SC_RECVFROM,
        syscall_codes::PPM_SC_RECVMSG,
        syscall_codes::PPM_SC_RENAME,
        syscall_codes::PPM_SC_RENAMEAT,
        syscall_codes::PPM_SC_RENAMEAT2,
        syscall_codes::PPM_SC_RMDIR,
        syscall_codes::PPM_SC_SECCOMP,
        syscall_codes::PPM_SC_SENDMMSG,
        syscall_codes::PPM_SC_SENDTO,
        syscall_codes::PPM_SC_SETGID,
        syscall_codes::PPM_SC_SETNS,
        syscall_codes::PPM_SC_SETPGID,
        syscall_codes::PPM_SC_SETRESGID,
        syscall_codes::PPM_SC_SETRESUID,
        syscall_codes::PPM_SC_SETRLIMIT,
        syscall_codes::PPM_SC_SETSID,
        syscall_codes::PPM_SC_SETSOCKOPT,
        syscall_codes::PPM_SC_SETUID,
        syscall_codes::PPM_SC_SHUTDOWN,
        syscall_codes::PPM_SC_SIGNALFD,
        syscall_codes::PPM_SC_SIGNALFD4,
        syscall_codes::PPM_SC_SOCKET,
        syscall_codes::PPM_SC_SOCKETPAIR,
        syscall_codes::PPM_SC_SYMLINK,
        syscall_codes::PPM_SC_SYMLINKAT,
        syscall_codes::PPM_SC_TGKILL,
        syscall_codes::PPM_SC_TIMERFD_CREATE,
        syscall_codes::PPM_SC_TKILL,
        syscall_codes::PPM_SC_UMOUNT2,
        syscall_codes::PPM_SC_UNLINK,
        syscall_codes::PPM_SC_UNLINKAT,
        syscall_codes::PPM_SC_UNSHARE,
        syscall_codes::PPM_SC_USERFAULTFD,
        syscall_codes::PPM_SC_VFORK,
        syscall_codes::PPM_SC_SCHED_PROCESS_EXIT,
    ]
    .into_iter()
    .map(|code| code as u32)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_syscall_numbers_map_to_ppm_sc_codes() {
        let native: HashSet<u32> = [libc::SYS_openat, libc::SYS_clone3, libc::SYS_execve]
            .into_iter()
            .map(|nr| nr as u32)
            .collect();
        let expected: HashSet<u32> = [
            syscall_codes::PPM_SC_OPENAT,
            syscall_codes::PPM_SC_CLONE3,
            syscall_codes::PPM_SC_EXECVE,
        ]
        .into_iter()
        .map(|code| code as u32)
        .collect();
        assert_eq!(native_to_ppm_sc(&native), expected);
    }

    // Every zone-default syscall this architecture has must survive the name match, so
    // mirror mode never silently loses one of them.
    #[test]
    fn zone_default_syscalls_round_trip_through_native_numbers() {
        let by_code: HashMap<u32, String> = syscall_codes::iter()
            .map(|code| (code as u32, code.to_string()))
            .collect();
        let mut checked = 0;
        for code in default_zone_syscalls() {
            let name = by_code[&code].trim_start_matches("PPM_SC_").to_lowercase();
            let Ok(sysno) = name.parse::<Sysno>() else {
                continue; // not a syscall on this architecture (or a tracepoint)
            };
            let native = HashSet::from([sysno.id() as u32]);
            assert_eq!(native_to_ppm_sc(&native), HashSet::from([code]), "{name}");
            checked += 1;
        }
        // Guards against the name match quietly checking nothing.
        assert!(checked >= 60, "only {checked} syscalls checked");
    }

    #[test]
    fn default_zone_syscalls_track_process_lifecycle() {
        let set = default_zone_syscalls();
        for code in [
            syscall_codes::PPM_SC_SCHED_PROCESS_EXIT,
            syscall_codes::PPM_SC_CLONE,
            syscall_codes::PPM_SC_CLONE3,
            syscall_codes::PPM_SC_FORK,
            syscall_codes::PPM_SC_VFORK,
            syscall_codes::PPM_SC_EXECVE,
            syscall_codes::PPM_SC_EXECVEAT,
        ] {
            assert!(set.contains(&(code as u32)), "{code:?} missing");
        }
        assert_eq!(set.len(), 81);
    }
}
