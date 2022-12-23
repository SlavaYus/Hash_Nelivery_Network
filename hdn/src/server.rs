use crate::client_communication::handle_client;
use crate::client_communication::ErrorType;
use mt_logger::*;

use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
};

/// Run the server for connecting with clients
/// work with errors
/// Asking in main

pub fn run() -> String {
    mt_new!(None, Level::Info, OutputStream::Both);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let client_data_storage: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        eprintln!("Stable connection");

        let client_data_storage: Arc<Mutex<HashMap<String, String>>> = client_data_storage.clone();

        let _thread = std::thread::Builder::new().spawn(move || {
            let communication_result = handle_client(client_data_storage, stream);
            match communication_result {
                Ok(_) => String::from("Success"),
                Err(e) => match e {
                    ErrorType::IoError(_) => String::from("Problem with connection!"),
                    ErrorType::SerdeJson(_) => String::from("Wrong format!"),
                    ErrorType::StringError(_) => String::from("Ups!"),
                },
            }
        });
    }
    return String::from("End");
}
