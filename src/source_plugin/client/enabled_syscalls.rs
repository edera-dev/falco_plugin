use anyhow::{Context, Result, anyhow};
use aya::maps::{Array, Map, MapData};
use libscap_bindings::types::ppm_sc_code as syscall_codes;
use log::{debug, info};
use std::collections::HashSet;
use std::fs;

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

    for syscall_id in 0u32..syscall_codes::PPM_SC_MAX as u32 {
        if let Ok(value) = array.get(&syscall_id, 0)
            && value != 0
        {
            enabled.insert(syscall_id);
        }
    }

    info!("read {} enabled syscalls", enabled.len());

    Ok(enabled)
}
