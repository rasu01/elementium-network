use super::*;

impl Client {
    fn new(address: String) -> std::result::Result<Client, std::io::Error> {

        match std::net::UdpSocket::bind("0.0.0.0:0") {
			Ok(sock) => {

				match sock.set_nonblocking(true) {

					Ok(_) => {
						return Ok(Client { 
                            socket: sock,
                            receive_buffer: [0; 60000], 
                            address: address, 
                            sequence: 0, 
                            reliable: 0, 
                            events: std::collections::VecDeque::new()
                        });
					}

					Err(error) => {
						return Err(error);
					}
				}
			}

			Err(error) => {
				return Err(error);
			}
		}

    }
}