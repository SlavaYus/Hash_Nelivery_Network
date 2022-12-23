use std::io::Error;

/// Checks are any problems in request from client
pub fn checker(x: Result<usize, Error>, vec: &[u8]) -> bool {
    if x.is_err() {
        eprintln!("1Unknown Problem!");
        return true;
    }
    if vec.last().unwrap() != &b'}' {
        eprintln!("2Wrong Format!");
        return true;
    }
    false
}
