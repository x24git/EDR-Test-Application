/// Opens a socket connection to the target at a specified port. Will send provided message
/// and then close the connection. Connection will not be maintained
///
/// # Parameters
///
/// - `ip`: A string containing the IP address of the target
/// - `port`: An integer containing the port number of the target
/// - `message`: A char array pointer to the message contents to send to the target
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: The message was successfully sent to the target.
/// - `Err`: There was an issue sending the message. (Network issue or bad message)
pub fn send_message(ip: String, port: i32, message: *char,) -> Result<(), Error>{
    todo!()
}