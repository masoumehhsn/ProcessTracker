use std::fs;
use std::{thread, time::Duration};
use std::collections::HashMap;
use std::time::{Instant};


mod process;
mod process_monitor;
mod resources;
mod cpu_usage;
use crate::process_monitor::get_processes;

fn main() {
    let mut previous_cpu_times: HashMap<u32, u64> = HashMap::new();
    let ticks_per_second = unsafe {
        libc::sysconf(libc::_SC_CLK_TCK)
    } as f64;
    if ticks_per_second <= 0.0 {
        eprintln!("Failed to get clock ticks per second");
        return;
    }
    let mut pevious_sample_time : Option<Instant> = None; 
    loop {
        let now = Instant::now();
        let elapsed_time = pevious_sample_time.map(|previous| now.duration_since(previous).as_secs_f64());
        pevious_sample_time = Some(now);
        let processes = get_processes();
        print!("\x1B[2J\x1B[H");

        println!(
            "{:<8} {:<25} {:<8} {:<15} {:<12}",
            "PID",
            "NAME",
            "STATE",
            "MEMORY (MiB)",
            "CPU (%)"
        );
        for process in &processes {
                let cpu_time = match cpu_usage::read_process_cpu_time(process.pid) {
                Ok(cpu_time) => cpu_time,
                Err(error) => {
                    eprintln!("Error: {}", error);
                    continue;
                }
            };
            let cpu_precent = match (
                previous_cpu_times.get(&process.pid), elapsed_time) {
                    (Some(&previous_cpu_time), Some(elapsed))
                    if elapsed > 0.0 => {
                        let cpu_difference = cpu_time.strict_sub(previous_cpu_time);
                        (cpu_difference as f64
                            / ticks_per_second
                            /elapsed) * 100.0

                    }
                    _ => 0.0,
                
            };
            previous_cpu_times.insert(process.pid,cpu_time );
                        println!(
                "{:<8} {:<25} {:<8} {:<15.2} {:<12.2}",
                process.pid,
                process.name,
                process.state,
                process.memory_usage as f64 / 1024.0,
                cpu_precent
            );

        }
        thread::sleep(Duration::from_secs(2));
    }

}