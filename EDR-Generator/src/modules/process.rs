use std::process;
use std::io::Error;

/// Structure defining the a process
///
/// # Parameters
///
/// - `id`: Process ID
/// - `name`: Process Name
/// - `cmd`: Process Command Line
/// - `stime`: Start Time
pub struct Process {
    pub id: u32,
    pub name: String,
    pub cmd: String,
    pub stime: String,
}

/// Structure defining the Process Manager Class
///
/// # Parameters
///
/// - `shell`: Shell path to spawn new processes from
/// - `processes`: Process Vector of all running processes
pub struct ProcessManager {
    shell: String,
    processes: Vec<Process>,
}

impl ProcessManager {
    /// Instantiates the Process Manager with a default shell path
    /// # Parameters
    ///
    /// - `shell_path`: Optional shell path to spawn new processes from. If not provided,
    ///                 default OS shell will be used. "cmd" for Windows, "sh" for Unix/Darwin
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: The ProcessManager Instance
    /// - `Err`: Shell path does not exist (or no permissions)
    pub fn new(shell_path: Option<String>) -> Result<ProcessManager, Error> {
        todo!()
    }
    /// Spawns a new process from the shell
    /// # Parameters
    ///
    /// - `name`: Name of the process to spawn
    /// - `command`: Shell command to execute when spawning the process
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: An integer representing the Process ID
    /// - `Err`: Error when executing command
    pub fn new_process(&mut self, name: String, command: String) -> Result<(u64), Error>{
        todo!()
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
    fn stop_process(&mut self, pid: u64) -> Result<(), Error>{
        todo!()
    }

    /// Stops all child processes spawned by the Process Manager instance
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: Array of Process IDs that were stopped
    /// - `Err`: Unable to stop processes (various errros)
    pub fn stop_all(&mut self) -> Result<([u64]), Error>{
        todo!()


}