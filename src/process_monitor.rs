use std::fs;

use crate::process::Process;

pub fn get_processes() -> Vec<Process> {
        let entries = fs::read_dir("/proc").expect("Failed to read /proc");
    let mut process_vec : Vec<Process> = Vec::new();
    for entry in entries{
        let entry = match entry{
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let file_name = entry.file_name();
        let pid = match file_name.to_string_lossy().parse::<u32>(){
            Ok(pid) => pid,
            Err(_) => continue,
        };
        let status_path = format!("/proc/{}/status", pid);
        let status = match fs::read_to_string(&status_path){
            Ok(status) => status,
            Err(_) => continue,
        };
        let mut name = String::new();
        let mut memory_usage = 0;
        let mut state = String::new();

        for line in status.lines(){
            if let Some(value) = line.strip_prefix("Name:"){
                name = value.trim().to_string();
            }
            else if let Some(value) = line.strip_prefix("VmRSS:") {
                memory_usage = value
                    .split_whitespace()
                    .next()
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(0);
                
            }
            else if let Some(value) = line.strip_prefix("State:"){
                state = value
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
            }

        }

        let process = Process::new(pid, name, state, memory_usage, 0.0);
        process_vec.push(process);


    }
    process_vec
}