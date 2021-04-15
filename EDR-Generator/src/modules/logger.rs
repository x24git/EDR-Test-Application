use csv::{WriterBuilder, Writer};
use serde::Serialize;
use crate::modules::common::{GenerationError, get_time};
use std::fs::File;
use std::process;
use sysinfo::{SystemExt, ProcessExt};

/// Structure containing all information and  format for a standard log message
///
/// # Parameters
///
/// - `t`: Type of log
/// - `timestamp`: Time of log event
/// - `username`: username that generated the event
/// - `proc_name`: name of process that generated event (or is the event)
/// - `proc_cmd`: command line arguments of process that generated event (or is the event)
/// - `proc_id`: process id of process that generated event (or is the event)
/// - `activity`: short text describing the type of event
/// - `file_path`: full path to a file involved in the event
/// - `source_addr`: IPv4 address of the source of a network event
/// - `source_port`: port number of the source of a network event
/// - `dest_addr`: IPv4 address of the destination of a network event
/// - `dest_port`: port number of the destination of a network event
/// - `bytes_sent`: number of bytes sent during a network event
/// - `protocol`: network protocol of the network event
#[derive(Serialize)]
pub struct Log {
    pub t: String,
    pub timestamp: String,
    pub username: String,
    pub proc_name: String,
    pub proc_cmd: String,
    pub proc_id: String,
    pub activity: String,
    pub file_path: String,
    pub source_addr: String,
    pub source_port: String,
    pub dest_addr: String,
    pub dest_port: String,
    pub bytes_sent: String,
    pub protocol: String,
}

/// Structure containing all information and  format for an error log message
///
/// # Parameters
///
/// - `t`: Type of log
/// - `timestamp`: Time of log event
/// - `message`: content of the error
#[derive(Serialize)]
pub struct LogError {
    t: String,
    timestamp: String,
    message: String,
}


/// Structure defining the Logger Class
///
/// # Parameters
///
/// - `writer`: Result of CSV Writer used for writing output in csv format
/// - `username`: global username for the current application
/// - `proc_name`: global process name for the current application
/// - `proc_cmd`: global process command line arguments for the current application
/// - `proc_id`: global process id for the current application
pub struct Logger{
    writer: csv::Result<Writer<File>>,
    username: String,
    proc_name: String,
    proc_cmd: String,
    proc_id: String,
}

impl Logger {
    /// Instantiates the Logger with an output to the specified file. Gathers current process information
    ///
    /// # Parameters
    ///
    /// - `path`: path for the output file where the csv data will be stored
    ///
    /// # Returns
    ///
    /// A Logger Class Instance
    pub fn new(path: &String) -> Logger {
        // Retrieve information about the current process
        let mut system = sysinfo::System::new();
        let mut proc_name = "".to_string();
        let mut proc_cmd = "".to_string();
        let proc_id = process::id() as usize;
        // Update processes
        system.refresh_processes();
        match system.get_process(proc_id) {
            None => {}
            Some(process) => {
                proc_name = process.name().to_string();
                proc_cmd = process.cmd().join(" "); //command arguments should be joined as a string
            }
        }
        Logger {
            writer: WriterBuilder::new().flexible(true).from_path(path),
            username: whoami::username(),
            proc_name: proc_name,
            proc_cmd: proc_cmd,
            proc_id: proc_id.to_string()
        }
    }

    /// Logs an event to the CSV output writer
    /// # Parameters
    ///
    /// - `data`: A Log structure containing all data needed to be logged
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    /// # Panics
    ///
    /// Does not panic, but rather if errors occur, they are passed to the error logger.
    pub fn log_event(&mut self, mut data: Log) {
        data.username = self.username.clone();
        //check if the event already has process information, otherwise use the parent process info
        if data.proc_name == "" { data.proc_name = self.proc_name.clone();}
        if data.proc_id == "" { data.proc_id = self.proc_id.clone();}
        if data.proc_cmd == "" { data.proc_cmd = self.proc_cmd.clone();}
       match self.writer.as_mut() {
           Ok(inner) => {
               match inner.serialize(data) {
                   Ok(_) => {}
                   Err(_) => self.log_error(GenerationError::new("logging".to_string(), "Unable to Serialize Log Message".to_string()))
                }
           },
           Err(_) => self.log_error(GenerationError::new("logging".to_string(), "Unable to Generate Log".to_string()))
       };
    }
    /// Logs an GenerationError class error to the CSV output writer
    /// # Parameters
    ///
    /// - `data`: A Generation Error that will be logged
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    /// # Panics
    ///
    /// If error occurs in this process, we are unable to log the error, so it is necessary to panic
    /// and bubble the error up to the parent caller.
    pub fn log_error(&mut self, data: GenerationError) {
        let error_log = LogError{
            t: "Error".to_string(),
            timestamp: get_time(),
            message: format!("{}: {}", data.kind, data.message)
        };
        eprintln!("{}", data);
        match self.writer.as_mut() {
            Ok(inner) => { match inner.serialize(error_log) {
                    Ok(_) => {},
                    Err(e) => match e.kind() {
                        csv::ErrorKind::UnequalLengths{ .. } => {},
                        _ => panic!( "{}", GenerationError::new("logging".to_string(), e.to_string()))
                    }
                }
            },
            Err(_) =>  panic!( "{}", GenerationError::new("logging".to_string(), "Unable to Generate Log Data".to_string()))
        }
    }
}

