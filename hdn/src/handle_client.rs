use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::checkers;
use crate::map;
use crate::structures::{CommandLoad, CommandUniversal, LoadOutput, Name, NameOutput, StoreOutput};
use crate::transformers;

/// Enum of Errors
pub enum ErrorType {
    IoError(std::io::Error),
    SerdeJson(serde_json::Error),
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
/// Handling of clients. In has taking name of client and then is everithing ok make store or load.
/// It is asking in thread like this: handle_client(map_mutex, stream);
/// check the name of client first. If it is in right format, check the command store or load.
/// If everything in right format sent the answer to client in json format and print log.
/// Return Err if we have problems with connection or with wrong format of json
pub fn handle_client(
    mutex_map: Arc<Mutex<HashMap<String, String>>>,
    mut stream: TcpStream,
) -> Result<String, ErrorType> {
    let mut name_buf: Vec<u8> = Vec::new();

    let mut buf = BufReader::new(stream.try_clone().unwrap());

    buf.read_until(b'}', &mut name_buf)?;

    let name_string = transformers::vec_to_string(&name_buf.clone());

    let name: Name = serde_json::from_str(&name_string)?;
    let name_output = NameOutput {
        student_name: name.name.clone(),
    };

    let pre_sended_name: String = serde_json::to_string(&name_output)?;
    let sended_name = transformers::string_to_vec(&pre_sended_name);
    let probable_connection_problem = stream.write_all(&sended_name);
    if probable_connection_problem.is_err() {
        eprintln!("Problem with network!");
        return Ok(String::from("Problem with network!"));
    }
    eprintln!(
        "127.0.0.1 [ {} ], Connection established.",
        transformers::time_to_string()
    );
    //end of reding name

    let cloned_map = mutex_map.clone();
    //start catching messages
    loop {
        let mut request: Vec<u8> = Vec::new();
        let result_of_reading = buf.read_until(b'}', &mut request);
        if checkers::checker(result_of_reading, &request) {
            break;
        }
        //find json task. making string from vector
        let request_string = transformers::vec_to_string(&request.clone());
        //eprintln!("{}", request_string );
        let mut task = CommandUniversal::new();
        let res: Result<CommandUniversal, serde_json::Error> =
            serde_json::from_str(&request_string);
        if res.is_err() {
            let task_load: CommandLoad = serde_json::from_str(&request_string)?;
            transformers::load_to_command(&task_load, &mut task);
        } else {
            task = res.unwrap();
        }

        let mut load_answer = LoadOutput::new();

        let mut store_answer = StoreOutput::new();

        let mut guard = cloned_map.lock().unwrap();
        //work with two options store or load
        //Store: puts data into map, write message to client, print log
        if task.request_type == "store" {
            map::store(&task, &mut store_answer, &mut guard);

            let pre_sended: String = serde_json::to_string(&store_answer)?;
            let sended = transformers::string_to_vec(&pre_sended);
            stream.write_all(&sended)?;

            //printing log
            eprintln!(
                "127.0.0.1 [{}] Received request to write new value  {}  by key {}.
                 Storage size: {}.",
                transformers::time_to_string(),
                task.hash.clone(),
                task.key.clone(),
                guard.clone().len()
            );
        } else if task.request_type == "load" {
            //Load: sending message to client, printing log
            map::load(&task, &mut load_answer, &mut guard);

            let pre_sended: String = serde_json::to_string(&load_answer)?;
            let sended = transformers::string_to_vec(&pre_sended);
            stream.write_all(&sended)?;
            eprintln!(
                "127.0.0.1 [{}] Received request to get value by key {}.
                 Storage size: {}.",
                transformers::time_to_string(),
                task.key,
                guard.clone().len()
            );
        } else {
            eprintln!("8Wrong Format!");
            break;
        }
    }
    return Ok(String::from("Success"));
}
