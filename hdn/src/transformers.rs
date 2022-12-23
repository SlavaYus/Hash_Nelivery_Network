use crate::structures::{CommandLoad, CommandUniversal};

/// transform vector of bytes into String
/// ascing in handle_client like vec_to_string(something_from_bufreader);
pub fn vec_to_string(vec: &[u8]) -> String {
    match String::from_utf8(vec.to_owned()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

/// transform String into vector of bytes
/// /// ascing in handle_client like string_to_vec(string);
pub fn string_to_vec(s: &str) -> Vec<u8> {
    let u = s.to_owned();
    u.into_bytes()
}

/// transform Command of loading to universal command
/// /// ascing in handle_client load_to_command(load, command);
pub fn load_to_command(load: &CommandLoad, command: &mut CommandUniversal) {
    command.request_type = load.request_type.clone();
    command.key = load.key.clone();
    command.hash = String::new();
}

/// transform time to string
pub fn time_to_string() -> String {
    chrono::offset::Local::now().to_string()
}
