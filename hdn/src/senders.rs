use crate::client_communication::ErrorType;
use crate::structures::ClientRequest;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::MutexGuard;

use crate::structures::NameOutput;

use crate::structures::RequestResponse;
use mt_logger::*;
use std::io::Write;

/// sending the name of creater and writing log
pub fn send_student_name(stream: &mut TcpStream) -> Result<String, ErrorType> {
    let name_output = NameOutput {
        student_name: String::from("Yusupov"),
    };
    let sended_name = serde_json::to_string(&name_output)?;
    let sended_name = sended_name.as_bytes();
    stream.write_all(&sended_name)?;
    mt_log!(
        Level::Info,
        "127.0.0.1 [ {} ], Connection established.",
        chrono::offset::Local::now().to_string()
    );

    Ok(String::from("END"))
}

/// sending json about result of loading/storing
///
/// ```
/// pub fn send_response(response:RequestResponse, stream: &mut TcpStream) ->  Result<String, ErrorType>{
///     let request_response = serde_json::to_string(&response)?;
///     let request_response = request_response.as_bytes();
///     stream.write_all(&request_response)?;
///     Ok(String::from("END"))
/// }
///
/// ```
pub fn send_response(
    response: RequestResponse,
    stream: &mut TcpStream,
) -> Result<String, ErrorType> {
    let request_response = serde_json::to_string(&response)?;
    let request_response = request_response.as_bytes();
    stream.write_all(&request_response)?;
    Ok(String::from("END"))
}

/// sending logs
/// matches enum request
pub fn log_request(
    client_data_storage: &MutexGuard<HashMap<String, String>>,
    request: &ClientRequest,
) {
    match request {
        // if request is load
        ClientRequest::Load { key } => {
            mt_log!(
                Level::Info,
                "127.0.0.1 [{}] Received request to get value by key {}.
            Storage size: {}.",
                chrono::offset::Local::now().to_string(),
                key,
                &client_data_storage.len()
            );
        }
        // if request is store
        ClientRequest::Store { key, hash } => {
            mt_log!(
                Level::Info,
                "127.0.0.1 [{}] Received request to write new value  {}  by key {}.
            Storage size: {}.",
                chrono::offset::Local::now().to_string(),
                hash,
                key,
                &client_data_storage.len()
            );
        }
    }
}
