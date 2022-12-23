use crate::map::{load, store};
use crate::{structures::ClientRequest, structures::RequestResponse};

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::senders::{log_request, send_response, send_student_name};

/// Enum of Errors
pub enum ErrorType {
    IoError(std::io::Error),
    SerdeJson(serde_json::Error),
    StringError(std::string::FromUtf8Error),
}
impl From<std::io::Error> for ErrorType {
    fn from(err: std::io::Error) -> Self {
        ErrorType::IoError(err)
    }
}
impl From<serde_json::Error> for ErrorType {
    fn from(err: serde_json::Error) -> Self {
        ErrorType::SerdeJson(err)
    }
}
impl From<std::string::FromUtf8Error> for ErrorType {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ErrorType::StringError(err)
    }
}

/// reading stream
fn read_from_stream(
    tcp_stream_buf_reader: &mut BufReader<TcpStream>,
) -> Result<ClientRequest, ErrorType> {
    let mut buffer = Vec::new();
    tcp_stream_buf_reader.read_until(b'}', &mut buffer)?;
    let buffer = String::from_utf8(buffer).unwrap();
    Ok(serde_json::from_str(&buffer)?)
}
/// Handling of clients. In has taking name of client and then is everithing ok make store or load.
/// It is asking in thread like this: handle_client(map_mutex, stream);
/// check the name of client first. If it is in right format, check the command store or load.
/// If everything in right format sent the answer to client in json format and print log.
/// Return Err if we have problems with connection or with wrong format of json
pub fn handle_client(
    client_data_storage: Arc<Mutex<HashMap<String, String>>>,
    mut stream: TcpStream,
) -> Result<String, ErrorType> {
    let mut stream_reader = BufReader::new(stream.try_clone().unwrap());
    // writing name
    send_student_name(&mut stream)?;

    loop {
        let request: ClientRequest = read_from_stream(&mut stream_reader)?;
        let response: RequestResponse;
        let mut client_data_storage: MutexGuard<HashMap<String, String>> =
            client_data_storage.lock().unwrap();
        log_request(&client_data_storage, &request);
        match request {
            // if request is load
            ClientRequest::Load { key } => {
                response = load(&mut client_data_storage, &key);
            }
            // if request is store
            ClientRequest::Store { key, hash } => {
                response = store(&mut client_data_storage, &key, &hash);
            }
        }
        // sending response
        send_response(response, &mut stream)?;
    }
}
