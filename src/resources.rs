use std::fs;

pub struct CpuStats {
    last_idle: u64,
    last_total: u64,
}

impl CpuStats {
    pub fn new() -> Self {
        Self {
            last_idle: 0,
            last_total: 0,
        }
    }

    pub fn get_cpu_usage(&mut self) -> Result<f64, String> {
        let contents = fs::read_to_string("/proc/stat")
            .map_err(|e| format!("Failed to read /proc/stat: {e}"))?;

        let cpu_line = contents
            .lines()
            .find(|line| {
                line.starts_with("cpu ")
            })
            .ok_or("CPU information not found")?;

        let values: Vec<u64> = cpu_line
            .split_whitespace()
            .skip(1)
            .map(|value| {
                value.parse::<u64>()
                    .map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to parse CPU stats: {e}"))?;

        if values.len() < 4 {
            return Err("Not enough CPU statistics".to_string());
        }

        let user = values[0];
        let nice = values[1];
        let system = values[2];
        let idle = values[3];

        let total = user + nice + system + idle;

        let total_diff = total.saturating_sub(self.last_total);
        let idle_diff = idle.saturating_sub(self.last_idle);

        self.last_total = total;
        self.last_idle = idle;

        if total_diff == 0 {
            return Ok(0.0);
        }

        let usage =
            100.0 * (1.0 - idle_diff as f64 / total_diff as f64);

        Ok(usage)
    }
}