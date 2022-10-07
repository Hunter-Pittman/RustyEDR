// Continious process monitoring using background_jobs
use sysinfo::*;

pub struct Process {
    pid: u32,
    parent_process: u32,
    name: String,
    command: Vec<String>,
    rundir: String,
    user: Option<Uid>
}

pub fn process_info() -> std::vec::Vec<Process> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let processes = sys.processes();

    let mut process_dump = vec![];

    for (pid, process_data) in processes {

        let value = Process {
            pid: pid.as_u32(),
            parent_process: match process_data.parent() {
                None => 0,
                Some(ppid) => ppid.as_u32()
            },
            name: process_data.name().to_string(),
            command: process_data.cmd().to_vec(),
            rundir: process_data.cwd().display().to_string(),
            user: process_data.user_id().map(|s| s.to_owned())
        };
        process_dump.push(value);
    }

    return process_dump
}