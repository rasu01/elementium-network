mod network;

use network::*;

fn main() {

    let port = 25501;

    if let Ok(mut server) = Server::new(port, 10) {
        println!("Started server with port {}", port);

        server.setup_channel(0, ChannelType::Reliable);
        server.setup_channel(1, ChannelType::Nonreliable);
        server.setup_channel(2, ChannelType::Sequenced);
        server.setup_channel(3, ChannelType::NonreliableDropable);

        loop {
            server.update(1.0/20.0);

            while let Some(event) = server.get_event() {

                match event {

                    EventType::Connect(address) => {
                        println!("A new client has connected {}", address);
                        let mut pack_test = Packet::new();

                        pack_test.push::<String>(&String::from("日本語はめっちゃ難しいけど、勉強するのが好き！！"));
                        server.send_to_peer(&address, 0, &pack_test);
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
