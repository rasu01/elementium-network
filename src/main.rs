mod network;

use network::*;

fn main() {

    let port: u16 = 25501;

    if let Ok(server) = Server::new(port, 10) {
        println!("Started server with port {}", port);
    } else {
        println!("Could not open server on port {}. Is it already taken?", port);
    }
}
