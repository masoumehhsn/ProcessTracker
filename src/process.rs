#[derive(Debug)]
pub struct Process{
    pub pid: u32,
    pub name: String,
    pub state: String,
    pub memory_usage: u64,
    pub cpu_usage: f64,

}

impl Process {
    pub fn new(pid: u32,name: String, state: String, memory_usage: u64, cpu_usage : f64) -> Process{
        Process { pid, name, state, memory_usage, cpu_usage }
    }
}
