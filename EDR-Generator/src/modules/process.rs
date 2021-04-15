use std::process::Command;
use crate::modules::common::GenerationError;
use std::thread;
use std::time::Duration;
use shlex::Shlex;
use sysinfo::{SystemExt, ProcessExt};
use crate::modules::logger::Log;


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
/// - `system`: System instance that tracks system processes
pub struct ProcessManager{
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
pub struct KillCount {
    pub killed: Vec<Log>,
    pub premature: Vec<Log>,
    pub failures: Vec<Log>,
}

impl Drop for ProcessManager {
    fn drop(&mut self) {
        match self.stop_all() {
            Ok(_) => return,
            Err(e) => eprintln!("{}", e) //todo: hook up to logger
        }
    }
}

impl ProcessManager{
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
            system: sysinfo::System::new(),
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
    /// - `Ok`: Log data confirming the process was created
    /// - `Err`: Error when executing command
    pub fn new_process(&mut self, path: String, arguments: Option<String>) -> Result<Log, GenerationError>{
        let args = String::from(arguments.unwrap_or(String::from(" ")));
        match Command::new(&path).args(Shlex::new(&args)).spawn() {
            Ok(child) =>{
                self.system.refresh_processes();
                let process = match self.system.get_process(child.id() as usize){
                   Some(inner) => inner,
                   None => return Err(GenerationError::new("processes".to_string(), "Process Died Unexpectedly".to_string())),
                };
                let full_cmd = format!("{} {}", path, args);

                self.processes.push(Process{
                    id: child.id() as usize,
                    name: String::from(process.name()),
                    cmd: String::from(full_cmd.clone()),
                    stime: process.start_time(),
                });

                Ok(adapt_log_process("New Process".to_string(),
                                     process.start_time(),
                                     String::from(process.name()),
                                     String::from(full_cmd),
                                     process.pid().to_string()))
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
    /// - `Ok`: KillCount data structure with Log data from the stopped processes
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
                        Ok(_) => {result.failures.push(adapt_log_process("Process Failed to Stop".to_string(),
                                                                         process.stime.clone(),
                                                                         process.name.clone(),
                                                                         process.cmd.clone(),
                                                                         process.id.to_string()))},
                        Err(_) => result.killed.push(adapt_log_process("Process Stopped".to_string(),
                                                                       process.stime.clone(),
                                                                       process.name.clone(),
                                                                       process.cmd.clone(),
                                                                       process.id.to_string()))
                    };
                },
                Err(_) => result.premature.push(adapt_log_process("Process had prematurely terminated".to_string(),
                                                                  process.stime.clone(),
                                                                  process.name.clone(),
                                                                  process.cmd.clone(),
                                                                  process.id.to_string()))
            }

        };
        if result.killed.len() == 0 && result.premature.len() == 0 && result.failures.len() > 0 {
            return Err(GenerationError::new("process".to_string(), "All Child Processes Failed to Terminate".to_string()))
        }
        Ok(result)
    }
}

/// Adapts a process event into a log struct used for logging
///
/// # Parameters
///
/// - `activity`: A string containing the type of activity that has occurred
/// - `timestamp`: Unix Epoc time when the process was created
/// - `proc_name`: Name of the process that was created
/// - `proc_cmd`: Command Line arguments that the process was started with
/// - `proc_id`: String containing the process ID
///
/// # Returns
///
/// A `Result` which is:
///
/// - A Log struct customized for process creation events
fn adapt_log_process(activity: String, timestamp: u64, proc_name: String, proc_cmd: String, proc_id: String) -> Log {

    Log{
        t: String::from("Information"),
        timestamp: timestamp.to_string(),
        username: String::from(""),
        proc_name,
        proc_cmd,
        proc_id,
        activity,
        file_path: String::from(""),
        source_addr: String::from(""),
        source_port: String::from(""),
        dest_addr: String::from(""),
        dest_port: String::from(""),
        bytes_sent: String::from(""),
        protocol: String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_os_shell() -> String {
        if cfg!(windows) {
            String::from("cmd")
        } else if cfg!(unix) {
            String::from("sh")
        } else {
            assert!(false, "OS not supported");
            String::from("")
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
        let mut pids:Vec<Log> =  vec![];
        let mut manager = ProcessManager::new().unwrap();
        pids.push(manager.new_process(get_os_shell(), None).unwrap());
        pids.push(manager.new_process(get_os_shell(), None).unwrap());
        pids.push(manager.new_process(get_os_shell(), None).unwrap());
        let result = manager.stop_all().unwrap();
        assert_eq!(result.killed.len(), pids.len())
    }
}
