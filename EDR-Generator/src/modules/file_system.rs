use std::fs::{OpenOptions, remove_file};
use crate::modules::common::GenerationError;
use std::io::Write;


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
/// - `Ok`: The file was created.
/// - `Err`: There was an issue creating the file. (Qualified Path does not exist or no permissions)
pub fn new_file(path: &String) -> Result<(), GenerationError>{
    OpenOptions::new().write(true).create_new(true).open(path)?;
    Ok(())
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
/// - `Ok`: The file was modified.
/// - `Err`: There was an issue modifying the file. (File does not exist or no permissions)
pub fn mod_file(path: &String,) -> Result<(), GenerationError>{
    let mut file = OpenOptions::new().write(true).append(true).open(path)?;
    file.write(b"\0")?;
    Ok(())
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
/// - `Ok`: The file was deleted.
/// - `Err`: There was an issue deleting the file. (File does not exist or no permissions)
pub fn delete_file(path: &String,) -> Result<(), GenerationError>{
    remove_file(path)?;
    Ok(())
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