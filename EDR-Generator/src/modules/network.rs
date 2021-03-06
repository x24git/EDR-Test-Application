use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use crate::modules::common::{GenerationError, get_time};
use std::thread;
use crate::modules::logger::Log;

/// Opens a socket connection to the target at a specified port. Will send provided message
/// and then close the connection. Connection will not be maintained
///
/// # Parameters
///
/// - `ip`: A string containing the IP address of the target
/// - `port`: An integer containing the port number of the target
/// - `message`: A u8 vector containing the message contents to send to the target
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: Log data confirming the message was successfully sent to the target.
/// - `Err`: There was an issue sending the message. (Network issue or bad message)
///
///
pub fn send_message(ip: &String, port: u16, message: &Vec<u8>,) -> Result<Log, GenerationError>{
    if port == 0 {
        return Err(GenerationError::new("network".to_string(), "Invalid Port Number".to_string()))
    }
    let net_address = format!("{}:{}", ip, &(port.to_string()));
    match TcpStream::connect(net_address) {
        Ok(mut stream) => {
            match stream.write(&*message){
                Ok(bytes_sent) => {
                    let mut local_addr = "unknown".to_string();
                    let mut local_port: u16 = 0;
                    match stream.local_addr() {
                        Ok(addr) => {
                            local_addr = addr.ip().to_string();
                            local_port = addr.port()
                        }
                        Err(_) => {}
                    }
                    return Ok(adapt_log_network("Network Connection".to_string(),
                                                local_addr, local_port,
                                                String::from(ip), port,
                                                bytes_sent, "TCP/IP".to_string()))
                },
                Err(_) => return Err(GenerationError::new("network".to_string(), "Unable to open stream for writing".to_string()))
            }
        },
        Err(_) => {
            return Err(GenerationError::new("network".to_string(), "Unable to Connect".to_string()))
        }
    }
}

/// Opens a socket to the localhost loopback address. Will send provided message
/// and then close the connection. Connection will not be maintained. Random OS assigned port.
/// Server instances is spun as a new thread to prevent blocking.
///
/// # Parameters
///
/// - `message`: A u8 vector containing the message contents to send to the target
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: Log data confirming the  message was successfully sent to the localhost.
/// - `Err`: There was an issue sending the message. (Can not open local port or bad message)
pub fn send_loopback_message(message: &Vec<u8>) -> Result<Log, GenerationError> {
    let listener = match spawn_server(&String::from("127.0.0.1"), 0){
        Ok(inner) => inner,
        Err(_) => return Err(GenerationError::new("network".to_string(), "Unable to Start Server".to_string()))
    };
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move|| {
        server_listen(listener)
    });
    send_message(&String::from("127.0.0.1"), port, message)
}

/// Spawns a TCPListener at the provided interface and port.
/// Use 0.0.0.0 to listen on all interfaces.
///
/// # Parameters
///
/// - `ip`: A string containing the local network interface to listen on
/// - `port`: An integer containing the port number of the target
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: A TCPListener was successfully created with the requested parameters
/// - `Err`: There was an issue creating the listener. (No permissions or other issue)
fn spawn_server(ip: &String, port: u16) -> Result<TcpListener, GenerationError> {
    let net_address = format!("{}:{}", ip, &(port.to_string()));
    let listener = TcpListener::bind(net_address)?;
    Ok(listener)
}

/// Accepts a TcpListener and then waits until a client connection is established.
/// Data received is returned as a result to the caller.
///
/// # Parameters
///
/// - `listener`: A TCPListener instance to start listening for connections on.
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: A u8 Vector containing the contents of the data received by the TcpListener
/// - `Err`: There was an issue sending the message. (Network issue or bad message)
fn server_listen(listener: TcpListener) -> Result<Vec<u8>, GenerationError>  {
    let mut recv_data = None;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut data = Vec::new();
                match stream.read_to_end(&mut data) {
                    Ok(_) => recv_data = Some(data),
                    Err(_) => recv_data = None
                };
                break;
            },
            Err(_) => {
                recv_data = None;
            }
        }
    }
    drop(listener);
    Ok(recv_data.unwrap_or(vec![]))
}

/// Adapts a network event into a log struct used for logging
///
/// # Parameters
///
/// - `activity`: A string containing the type of activity that has occurred
/// - `source_addr`: String containing the IPv4 IP address of the source
/// - `source_port`: Port number of the source connection
/// - `dest_addr`: String containing the IPv4 IP address of the destination
/// - `dest_port`: Port number of the destination connection
/// - `bytes_sent`: Number of bytes that were sent from source to destination
/// - `protocol`: String containing the network protocol used for communication
///
/// # Returns
///
/// A `Result` which is:
///
/// - A Log struct customized for network connection events
pub fn adapt_log_network(activity: String, source_addr: String, source_port: u16, dest_addr: String, dest_port: u16, bytes_sent: usize, protocol: String) -> Log {
    Log{
        t: String::from("Information"),
        timestamp: get_time(),
        username: String::from(""),
        proc_name: String::from(""),
        proc_cmd: String::from(""),
        proc_id: String::from(""),
        activity,
        file_path: String::from(""),
        source_addr,
        source_port: source_port.to_string(),
        dest_addr,
        dest_port: dest_port.to_string(),
        bytes_sent: bytes_sent.to_string(),
        protocol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_valid() {
        let message = Vec::from(String::from("hello world").as_bytes());
        let listener = spawn_server(&String::from("127.0.0.1"), 0);
        assert!(listener.is_ok());
        let server = listener.unwrap();
        let port = server.local_addr().unwrap().port();
        let child =thread::spawn(move|| {
            server_listen(server)
        });
        let result = send_message(&String::from("127.0.0.1"), port,&message);
        assert!(result.is_ok());
        let child_result = child.join();
        assert_eq!(child_result.unwrap().unwrap(), message);
    }

}
