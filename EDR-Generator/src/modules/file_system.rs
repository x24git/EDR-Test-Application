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
pub fn new_file(path: String,) -> Result<(), Error>{
    todo!()
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
pub fn mod_file(path: String,) -> Result<(), Error>{
    todo!()
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
pub fn delete_file(path: String,) -> Result<(), Error>{
    todo!()
}