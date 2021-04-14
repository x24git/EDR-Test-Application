use std::process::Command;
use crate::modules::common::GenerationError;
use std::thread;
use std::time::Duration;
use shlex::Shlex;
use sysinfo::{SystemExt, ProcessExt};


/// Structure defining the a process
///
/// # Parameters
///
/// - `id`: Process ID
/// - `name`: Process Name
/// - `cmd`: Process Command Line
/// - `stime`: Start Time
pub struct Process {
    pub id: usize,
    pub name: String,
    pub cmd: String,
    pub stime: u64,
}

/// Structure defining the Process Manager Class
///
/// # Parameters
///
/// - `processes`: Process Vector of all running processes
pub struct ProcessManager {
    processes: Vec<Process>,
    system: sysinfo::System,
}

/// Structure defining the process status
///
/// # Parameters
///
/// - `killed`: Process IDs that were killed
/// - `premature`: Process IDs that were terminated autonomously
/// - `failures`: Process IDs that failed to terminate
#[derive(Debug)]
pub struct KillCount {
    pub killed: Vec<usize>,
    pub premature: Vec<usize>,
    pub failures: Vec<usize>,
}

impl Drop for ProcessManager {
    fn drop(&mut self) {
        match self.stop_all() {
            Ok(_) => return,
            Err(e) => eprintln!("{}", e) //todo: hook up to logger
        }
    }
}

impl ProcessManager {
    /// Instantiates the Process Manager with a default shell path
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: The ProcessManager Instance
    /// - `Err`: Unable to get system information
    pub fn new() -> Result<ProcessManager, GenerationError> {
        Ok(ProcessManager {
            processes: Vec::new(),
            system: sysinfo::System::new()
        })

    }
    /// Spawns a new process from the shell
    /// # Parameters
    ///
    /// - `path`: Path to the executable to execute
    /// - `arguments`: additional arguments to pass to the process
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: An integer representing the Process ID
    /// - `Err`: Error when executing command
    pub fn new_process(&mut self, path: String, arguments: Option<String>) -> Result<usize, GenerationError>{
        let args = String::from(arguments.unwrap_or(String::from(" ")));

        match Command::new(&path).args(Shlex::new(&args)).spawn() {
            Ok(child) =>{
                self.system.refresh_processes();
                let process = match self.system.get_process(child.id() as usize){
                   Some(inner) => inner,
                   None => return Err(GenerationError::new("processs".to_string(), "Process Died Unexpectedly".to_string())),
                };

                self.processes.push(Process{
                    id: child.id() as usize,
                    name: String::from(process.name()),
                    cmd: path,
                    stime: process.start_time(),
                });
                Ok(child.id() as usize)
            },
            Err(err) => return Err(GenerationError::from(err))
        }
    }

    /// Stops a process with a given Process ID
    /// # Parameters
    ///
    /// - `pid`: Process ID to stop
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: The Process was stopped successfully
    /// - `Err`: The process could not be stopped (may not exist, or no permissions)
    fn stop_process(&self, pid: usize) -> Result<&sysinfo::Process, GenerationError>{
        let process = match self.system.get_process(pid){
            Some(inner) => inner,
            None => return Err(GenerationError::new("processs".to_string(), "Process Not Found".to_string())),
        };
        process.kill(sysinfo::Signal::Kill);
        Ok((process).clone())
    }

    /// Stops all child processes spawned by the Process Manager instance
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: Array of Process IDs that were stopped
    /// - `Err`: Unable to stop processes (various errors)
    pub fn stop_all(&mut self) -> Result<KillCount, GenerationError> {
        let mut result = KillCount {
            killed: vec![],
            premature: vec![],
            failures: vec![]
        };
        self.system.refresh_processes();
        for process in &self.processes{


            match self.stop_process(process.id) {
                Ok(_) => {
                    thread::sleep(Duration::from_millis(100));
                    self.system.refresh_processes();
                    match self.stop_process(process.id){
                        Ok(_) => {result.failures.push(process.id)},
                        Err(_) => result.killed.push(process.id)
                    };
                },
                Err(_) => result.premature.push(process.id)
            }

        };
        if result.killed.len() == 0 && result.premature.len() == 0 && result.failures.len() > 0 {
            return Err(GenerationError::new("processs".to_string(), "All Child Processes Failed to Terminate".to_string()))
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_os_shell() -> String {
        if cfg!(windows) {
            return String::from("cmd")
        } else if cfg!(unix) {
            return String::from("sh")
        } else {
            assert!(false, "OS not supported");
            return String::from("")
        }
    }

    #[test]
    fn valid_process_creation() {
        let mut manager = ProcessManager::new().unwrap();
        assert!(manager.new_process(get_os_shell(), None).is_ok())
    }

    #[test]
    fn invalid_process_creation(){
        let mut manager = ProcessManager::new().unwrap();
        assert!(manager.new_process(String::from("garbasgwe"), None).is_err())
    }

    #[test]
    fn all_processes_killed(){
        let mut pids:Vec<usize> =  vec![];
        let mut manager = ProcessManager::new().unwrap();
        pids.push(manager.new_process(get_os_shell(), None).unwrap());
        pids.push(manager.new_process(get_os_shell(), None).unwrap());
        pids.push(manager.new_process(get_os_shell(), None).unwrap());
        let result = manager.stop_all().unwrap();
        assert_eq!(result.killed, pids)
    }
}
