use std::fs::{OpenOptions, remove_file, canonicalize};
use crate::modules::common::{GenerationError, get_time};
use std::io::Write;
use crate::modules::logger::Log;
use std::path::PathBuf;

/// Create a new file at a given path
///
/// # Parameters
///
/// - `path`: A string containing the system file path (including name)
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: Log data confirming the file was created.
/// - `Err`: There was an issue creating the file. (Qualified Path does not exist or no permissions)
pub fn new_file(path: &String) -> Result<Log, GenerationError> {
    OpenOptions::new().write(true).create_new(true).open(path)?;
    Ok(adapt_log_file("New File".to_string(), canonicalize(path).unwrap().into_os_string().into_string()?))
}

/// Modify a new file at a given path. Modification will just append a single null byte to the end
/// of the file and close the file.
///
/// # Parameters
///
/// - `path`: A string containing the system file path (including name)
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: Log data confirming the file was modified.
/// - `Err`: There was an issue modifying the file. (File does not exist or no permissions)
pub fn mod_file(path: &String, ) -> Result<Log, GenerationError> {
    let mut file = OpenOptions::new().write(true).append(true).open(path)?;
    file.write(b"\0")?;
    Ok(adapt_log_file("Modify File".to_string(), canonicalize(path).unwrap().into_os_string().into_string()?))

}

/// Delete a new file at a given path.
///
/// # Parameters
///
/// - `path`: A string containing the system file path (including name)
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: Log data confirming the file was deleted.
/// - `Err`: There was an issue deleting the file. (File does not exist or no permissions)
pub fn delete_file(path: &String, ) -> Result< Log, GenerationError> {
    let orig_path = canonicalize(path).unwrap_or(PathBuf::new()).into_os_string().into_string()?;
    remove_file(path)?;
    Ok(adapt_log_file("Delete File".to_string(), orig_path))
}

/// Adapts a file event into a log struct used for logging
///
/// # Parameters
///
/// - `activity`: A string containing the type of activity that has occurred
/// - `file_path`: A string containing the file path that was modified, created, or deleted
///
/// # Returns
///
/// A `Result` which is:
///
/// - A Log struct customized for file events
fn adapt_log_file(activity: String, file_path: String) -> Log {
    Log{
        t: String::from("Information"),
        timestamp: get_time(),
        username: String::from(""),
        proc_name: String::from(""),
        proc_cmd: String::from(""),
        proc_id: String::from(""),
        activity,
        file_path,
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
    use rand::Rng;
    use super::*;
    use std::io::Read;

    fn rng_filename() -> String{
        let mut rng = rand::thread_rng();
        let rng_test = rng.gen_range(1000..9999);
        let mut path = "test".to_owned();
        path.push_str(&String::from(rng_test.to_string()));
        path.push_str(".txt");
        return path
    }

    #[test]
    fn valid_file_creation()-> Result<(), GenerationError> {
        let path = rng_filename();
        assert!(new_file(&String::from(&path)).is_ok());
        assert!(OpenOptions::new().read(true).open(String::from(&path)).is_ok());
        remove_file(&path)?;
        Ok(())
    }

    #[test]
    fn duplicate_file_creation()-> Result<(), GenerationError> {
        let path = rng_filename();
        assert!(new_file(&String::from(&path)).is_ok());
        assert!(new_file(&String::from(&path)).is_err());
        assert!(OpenOptions::new().read(true).open(String::from(&path)).is_ok());
        remove_file(&path)?;
        Ok(())
    }

    #[test]
    fn valid_file_modification()-> Result<(), GenerationError> {
        let path = rng_filename();
        OpenOptions::new().write(true).create(true).open(&path).unwrap();
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        assert_eq!(buffer, "");
        assert!(mod_file(&String::from(&path)).is_ok());
        file = OpenOptions::new().read(true).open(&path).unwrap();
        buffer = String::new();
        file.read_to_string(&mut buffer)?;
        assert_eq!(buffer, "\0");
        remove_file(&path)?;
        Ok(())
    }

    #[test]
    fn bad_file_modification()-> Result<(), GenerationError> {
        let path = rng_filename();
        assert!(mod_file(&String::from(&path)).is_err());
        Ok(())
    }

    #[test]
    fn valid_file_deletion()-> Result<(), GenerationError> {
        let path = rng_filename();
        OpenOptions::new().write(true).create(true).open(&path).unwrap();
        assert!(delete_file(&String::from(&path)).is_ok());
        assert!(OpenOptions::new().read(true).open(String::from(&path)).is_err());
        Ok(())
    }

    #[test]
    fn bad_file_deletion()-> Result<(), GenerationError> {
        let path = rng_filename();
        assert!(delete_file(&String::from(&path)).is_err());
        Ok(())
    }
}