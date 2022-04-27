pub mod network;
pub use network::*;

/* 
fn main() {

    let port = 25501;

    let server = false;

    if server {
        if let Ok(mut server) = Server::new(port, 1) {
            println!("Started server with port {}", port);
    
            server.setup_channel(0, ChannelType::Reliable);
            server.setup_channel(1, ChannelType::Nonreliable);
            server.setup_channel(2, ChannelType::Sequenced);
            server.setup_channel(3, ChannelType::NonreliableDropable);
    
            loop {
                server.update(1.0 / 60.0);
    
                while let Some(event) = server.get_event() {
    
                    match event {
    
                        ServerEvent::Connect(address) => {
                            println!("A new client has connected {}", address);
    
                            let mut packet = Packet::new();
                            packet.push::<String>(&String::from("Rustからstringです!"));
    
                            server.send_to_peer(&address, 0, packet);
                        }
    
                        ServerEvent::Timeout(address) => {
                            println!("A client has timed out {}", address);
                        }
    
                        ServerEvent::ServerFull(address) => {
                            println!("Client {} tried to connect, but the server is full.", address);
                        }
    
                        _ => {}
    
                    }
                }
            }
    
        } else {
            println!("Could not open server on port {}. Is it already taken?", port);
        }
    } else {

        if let Ok(mut client) = Client::new(String::from("127.0.0.1:25501")) {

            loop {
                client.update(1.0/60.0);

                while let Some(event) = client.get_event() {

                    match event {

                        ClientEvent::Connect => {
                            println!("Connected to server!");
                        }

                        ClientEvent::Timeout => {
                            println!("Timed out.");
                        }

                        _ => {
                        }

                    }

                }
            }
        }
    }
}*/