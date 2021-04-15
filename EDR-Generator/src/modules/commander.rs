use crate::modules::process::ProcessManager;
use crate::modules::file_system;
use crate::modules::network;
use std::time::Duration;
use crate::modules::logger::{Logger};
use csv::{ReaderBuilder, Reader, StringRecord};
use std::fs::File;
use crate::modules::common::GenerationError;
use std::thread;

/// Structure defining the Logger Class
///
/// # Parameters
///
/// - `reader`: CSV Reader used for reading input commands in csv format
/// - `process_manager`: process_manager instance to handle process event commands
/// - `logger`: Logger instance to handle logging of events
/// - `errors_encountered`: number of errors encountered during commanding
pub struct TaskCommander {
    reader: Reader<File>,
    process_manager: Option<ProcessManager>,
    logger: Logger,
    errors_encountered: usize,
}

impl TaskCommander {
    /// Instantiates the Commander with specific information about the input CSV
    ///
    /// # Parameters
    ///
    /// - `path`: path for the input file where the csv data will be retrieved
    /// - `deliminator`: deliminator that will be used when reading the csv file
    /// - `logger`: logger instance to use for logging
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: TaskCommander Instance
    /// - `Err`: Error in reading the input file
    pub fn new(path: &String, deliminator: u8, logger: Logger) -> Result<TaskCommander, GenerationError> {
        Ok(TaskCommander {
            reader: match ReaderBuilder::new().delimiter(deliminator).has_headers(false).flexible(true).from_path(path) {
                Ok(inner) => inner,
                Err(e) => return Err(GenerationError::new("io".to_string(), format!("The following error was encountered when attempting to open {} for processing: {}", path, e.to_string())))
            },
            process_manager: match ProcessManager::new() {
                Ok(inner) => Some(inner),
                Err(_) => None
            },
            logger,
            errors_encountered: 0,
        })
    }

    /// Retrieves the number of errors TaskCommander has encountered
    ///
    /// # Returns
    ///
    /// The number of errors encountered
    pub fn get_num_errors(self) -> usize {
        self.errors_encountered
    }


    /// Reads the next entry in the command list and processes the instructions
    ///
    /// # Returns
    ///
    /// A boolean representing if there is an entry to be processed. False returned when EOF.
    pub fn read_next(&mut self) -> bool {
        if let Some(result) = self.reader.records().next() {
            let new_record = result.unwrap();
            match &new_record[0] {
                "process" => self.run_process(new_record),
                "pause" => self.pause(new_record),
                "new_file" | "mod_file" | "delete_file" => self.file_system(new_record),
                "connect" | "connect_self" => self.network(new_record),
                _ => self.error_print(GenerationError::new("input_format".to_string(), format!("{} is not a valid instruction)", &new_record[0])))
            }
            true
        } else {
            false
        }
    }

    /// Runs a process by verifying the providing instructions, formatting data, and logging
    ///
    /// # Parameters
    ///
    /// - `params`: a StringRecord representing the row within the CSV document containing
    /// instructions on how to create the process
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    /// # Panics
    ///
    /// Should not panic as all errors are sent to the error logger.
    fn run_process(&mut self, params: StringRecord) {
        // check if process_manager is available
        if self.process_manager.is_none() {
            self.error_print(GenerationError::new("user_permissions".to_string(), "Child processes are not allowed to be spawned".to_string()));
            return;
        }
        //ensure correct number of parameters have been provided
        if params.len() < 2 {
            self.error_print(GenerationError::new("input_format".to_string(), format!("Record {:?} is not formatted correctly for a process (process,<path>,[arguments...])", params)));
            return;
        }
        let mut arguments = None;
        if params.len() > 2 {
            //concatenate additional parameter into a single space separated string to be used as process arguments
            let mut arguments_str = "".to_string();
            for index in 2..params.len() {
                arguments_str.push_str(&format!("{} ", &params[index]));
            }
            arguments = Some(arguments_str);
        }

        match self.process_manager.as_mut().unwrap().new_process(String::from(&params[1]), arguments) {
            Ok(result_log) => self.logger.log_event(result_log),
            Err(e) => {
                self.error_print(GenerationError::new(e.kind, format!("Record {:?} encountered an error {})", params, e.message)))
            }
        }
    }

    /// Runs file operations by verifying the providing instructions, formatting data, and logging
    ///
    /// # Parameters
    ///
    /// - `params`: a StringRecord representing the row within the CSV document containing
    /// instructions on how to work the file system
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    /// # Panics
    ///
    /// Should not panic as all errors are sent to the error logger.
    fn file_system(&mut self, params: StringRecord) {
        //ensure correct number of parameters have been provided
        if params.len() < 2 {
            self.error_print(GenerationError::new("input_format".to_string(), format!("Record {:?} is not formatted correctly for a process (<file_op>,<path>)", params)));
            return;
        }
        //determine which file operation to perform
        let result = match &params[0] {
            "new_file" => file_system::new_file(&String::from(&params[1])),
            "mod_file" => file_system::mod_file(&String::from(&params[1])),
            "delete_file" => file_system::delete_file(&String::from(&params[1])),
            _ => return self.error_print(GenerationError::new("input_format".to_string(), format!("{} is not a valid File Operation Command", &params[1])))
        };
        match result {
            Ok(result_log) => self.logger.log_event(result_log),
            Err(e) => {
                self.error_print(GenerationError::new(e.kind, format!("Record {:?} encountered an error {})", params, e.message)))
            }
        }
    }

    /// Runs network operations by verifying the providing instructions, formatting data, and logging
    ///
    /// # Parameters
    ///
    /// - `params`: a StringRecord representing the row within the CSV document containing
    /// instructions on how to set up a network connection
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    /// # Panics
    ///
    /// Should not panic as all errors are sent to the error logger.
    fn network(&mut self, params: StringRecord) {
        //ensure correct number of parameters have been provided for the correct command
        if (params.len() < 2 && &params[0] == "connect_self") || (params.len() < 4 && &params[0] == "connect") {
            self.error_print(GenerationError::new("input_format".to_string(), format!("Record {:?} is not formatted correctly for a process (<connect>,[destination_host],[destination_port],<message>)", params)));
            return;
        }
        //determine which network operation to perform
        let result = match &params[0] {
            "connect" => {
                //ensure that port number can be parsed into a u16 correctly
                let port = match params[2].parse::<u16>() {
                    Ok(inner) => inner,
                    _ => {
                        self.error_print(GenerationError::new("input_format".to_string(), format!("Record {:?} is not formatted correctly for a process (<connect>,[destination_host],[destination_port],<message>)", params)));
                        return;
                    }
                };
                network::send_message(&String::from(&params[1]), port, &Vec::from(params[3].to_string().as_bytes()))
            }
            "connect_self" => network::send_loopback_message(&Vec::from(params[1].to_string().as_bytes())),
            _ => return self.error_print(GenerationError::new("input_format".to_string(), format!("{} is not a valid Network Operation Command", &params[1])))
        };
        match result {
            Ok(result_log) => self.logger.log_event(result_log),
            Err(e) => {
                self.error_print(GenerationError::new(e.kind, format!("Record {:?} encountered an error {})", params, e.message)))
            }
        }
    }

    /// Pauses execution by verifying the providing instructions
    ///
    /// # Parameters
    ///
    /// - `params`: a StringRecord representing the row within the CSV document containing
    /// instructions on how long to pause
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    /// # Panics
    ///
    /// Should not panic as all errors are sent to the error logger.
    fn pause(&mut self, params: StringRecord) {
        //ensure correct number of parameters have been provided
        if params.len() < 2  {
            self.error_print(GenerationError::new("input_format".to_string(), format!("Record {:?} is not formatted correctly for a connect ([connect],[msec])", params)));
        }
        //ensure that delay time can be parsed into a u64 correctly
        let delay = match params[1].parse::<u64>() {
            Ok(inner) => inner,
            _ => {
                self.error_print(GenerationError::new("input_format".to_string(), format!("Record {:?} is not formatted correctly for a connect ([connect],[msec]", params)));
                return;
            }
        };
        thread::sleep(Duration::from_millis(delay))
    }

    /// Helper function for handling errors. Logs the error to the logger, displays error to console
    /// and increments number of errors that were encountered.
    ///
    /// # Parameters
    ///
    /// - `error`: Generation Error that will be logged and displayed.
    ///
    /// # Returns
    ///
    /// Nothing
    fn error_print(&mut self, error: GenerationError) {
        eprintln!("{}", error);
        self.logger.log_error(error);
        self.errors_encountered += 1;
    }
}


