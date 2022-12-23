#![allow(unused)]

use crate::test::IpVersion::V4;
/// Tests
use rand::Rng;
use std::io::{self, prelude::*};
use std::mem::take;
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::process::{Child, Command};
use std::str::{self, FromStr};
use std::thread;
use std::time::{self, Duration};
const BINARY_PATH: &str = env!("CARGO_BIN_EXE_hdn");

enum IpVersion {
    V4,
    V6,
}

struct ServerWrapper {
    proc: Option<Child>,
    addr: SocketAddr,
}

impl ServerWrapper {
    fn start(ip_version: IpVersion) -> Self {
        let mut rng = rand::thread_rng();
        let port = rng.gen_range(40000..49151);
        let ip = match ip_version {
            IpVersion::V4 => IpAddr::from_str("127.0.0.1").unwrap(),
            IpVersion::V6 => IpAddr::from_str("::1").unwrap(),
        };

        let proc = Command::new(BINARY_PATH)
            .arg("--ip")
            .arg(ip.to_string())
            .arg("--port")
            .arg(port.to_string())
            .spawn()
            .unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        Self {
            proc: Some(proc),
            addr: SocketAddr::new(ip, port),
        }
    }

    fn is_alive(&mut self) -> bool {
        self.proc
            .as_mut()
            .map_or(false, |proc| proc.try_wait().unwrap().is_none())
    }

    fn expected_to_be_dead(&mut self) {
        let _ = self.stop();
    }

    fn stop(&mut self) -> std::io::Result<()> {
        self.proc.take().map_or(Ok(()), |mut proc| proc.kill())
    }
}

impl Drop for ServerWrapper {
    fn drop(&mut self) {
        let _ = self.stop().unwrap();
    }
}

#[derive(Debug)]
struct Player {
    conn: TcpStream,
}

impl Player {
    fn start(server_addr: SocketAddr) -> std::io::Result<Self> {
        let conn = TcpStream::connect(server_addr)?;
        Ok(Self { conn })
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.conn.write_all(data)
    }

    fn read(&mut self, bytes: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0; bytes];
        self.conn.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn shutdown(&mut self, how: Shutdown) {
        let _ = self.conn.shutdown(how);
    }
}

#[test]
fn global_test() {
    let server = ServerWrapper::start(V4);
    let answer1: String = String::from(
        r#"{
        "student_name": "Yusupov"
    }"#,
    );

    let mut client = Player::start(server.addr).unwrap();
    assert_eq!(client.read(answer1.len()).unwrap(), answer1.as_bytes());
    let request1: String = String::from(
        r#"{"request_type": "store","key": "some_key","hash": "0b672dd94fd3da6a8d404b66ee3f0c83"}"#,
    );
    let answer2: String = String::from(
        r#"{
        "response_status": "success"
      }"#,
    );
    client.write(request1.as_bytes()).unwrap();
    assert_eq!(client.read(answer2.len()).unwrap(), answer2.as_bytes());
    let request2: String = String::from(
        r#"{
        "request_type": "load",
        "key": "some_key"
    } "#,
    );
    client.write(request2.as_bytes()).unwrap();
    let answer3: String = String::from(
        r#"{
        "response_status": "success",
        "requested_key": "some_key",
        "requested_hash": "0b672dd94fd3da6a8d404b66ee3f0c83",
      }"#,
    );
    assert_eq!(client.read(answer3.len()).unwrap(), answer3.as_bytes());
}

#[cfg(test)]
mod test {
    use crate::test::ServerWrapper;

    use crate::client_communication::ErrorType;
    use crate::map::{load, store};
    use crate::senders::send_student_name;
    use crate::structures::type_of_request;
    use crate::structures::RequestResponse;
    use crate::test::Player;
    use std::sync::MutexGuard;
    use std::{
        collections::HashMap,
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        sync::{Arc, Mutex},
    };

    #[test]
    fn type_of_request_test() {
        assert_eq!(type_of_request("store"), true);
        assert_eq!(type_of_request("load"), false);
    }

    #[test]

    fn send_student_name_test() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let a = send_student_name(&mut stream);
            match a {
                Ok(v) => assert_eq!(v, String::from("End")),
                Err(_) => println!("Ups!"),
            };
            break;
        }
    }

    #[test]

    fn total_work() {
        let client_data_storage: Arc<Mutex<HashMap<String, String>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let _thread = std::thread::Builder::new().spawn(move || {
            let mut response: RequestResponse;
            let mut client_data_storage: MutexGuard<HashMap<String, String>> =
                client_data_storage.lock().unwrap();
            response = store(&mut client_data_storage, "0", "hi");
            response = load(&mut client_data_storage, "0");
            let expected = RequestResponse::Success {
                requested_key: String::from("0"),
                requested_hash: String::from("hi"),
            };
            assert_eq!(response, expected);
        });
    }
}
