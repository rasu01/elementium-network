mod network;

use network::*;

fn main() {

    let port = 25501;

    if let Ok(mut server) = Server::new(port, 10) {
        println!("Started server with port {}", port);

        loop {
            server.update(1.0/20.0);

            while let Some(event) = server.get_event() {

                match event {

                    EventType::Connect(address) => {
                        println!("A new client has connected {}", address);
                    }

                    EventType::Timeout(address) => {
                        println!("A client has timed out {}", address);
                    }

                    _ => {}

                }

            }
        }

    } else {
        println!("Could not open server on port {}. Is it already taken?", port);
    }
}
