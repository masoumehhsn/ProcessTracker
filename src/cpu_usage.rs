struct ProcessCpuSample {
    pid: u32,
    cpu_time: u64,
}

use std::{fmt::format, fs};

/// Returns the total CPU time consumed by a process, in clock ticks.
///
/// Reads utime (user-mode CPU time) and stime (kernel-mode CPU time)
/// from /proc/<pid>/stat and adds them together.
pub fn read_process_cpu_time(pid: u32) -> Result<u64, String> {
    
    let path = format!("/proc/{}/stat", pid);
    let content = fs::read_to_string(&path).
    map_err(|error| format!("failed to read {} : {}", path, error))?;
    let closing_pattern = content.rfind(')').
    ok_or_else(|| format!("Format error in {}", path))?;
    let remaining = content[closing_pattern+1 ..].trim();
    let fields : Vec<&str> = remaining.split_whitespace().collect();
    if fields.len() < 13 {
        return Err(format!("Not enough fields in {}", path));
    }
    let utime = fields[11].
    parse::<u64>().
    map_err(|error| format!("error in getting utime: {}", error))?;

    let stime = fields[12].
    parse::<u64>().
    map_err(|error| format!("error in getting utime: {}", error))?;

    Ok(utime + stime)

}