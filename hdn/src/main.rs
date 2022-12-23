pub mod client_communication;
pub mod map;
pub mod senders;
pub mod server;
pub mod structures;
pub mod test;
use mt_logger::*;
fn main() {
    let info = server::run();
    mt_log!(Level::Info, "{}", info);
}
