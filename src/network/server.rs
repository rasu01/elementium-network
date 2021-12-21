use super::*;

impl Server {

	pub fn new(port: u16, max_connections: usize) -> std::result::Result<Server, std::io::Error> {

		match std::net::UdpSocket::bind(format!("0.0.0.0:{}", port)) {
			Ok(sock) => {

				match sock.set_nonblocking(true) {

					Ok(_) => {
						return Ok(Server {
							socket: sock,
							max_connections: max_connections,
							connections: std::collections::HashMap::new(),
							receive_buffer: [0; 60000],
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

	pub fn update(&mut self, sleep_time: f64) {
		match self.socket.recv_from(&mut self.receive_buffer) {
			Ok((packet_size, client)) => {
				//println!("{}", packet_size);

				if true { //size of packet header.. !packet_size < 18
					let mut packet = Packet::new();
					packet.write_bytes(&self.receive_buffer[0..packet_size]);

					let packet_type = packet.read_u8();
					let channel_id = packet.read_u8();

					let is_connected = self.connections.contains_key(&client.to_string());

					println!("{}", packet_type);

					match PacketType::from_u8(packet_type) {

						Some(PacketType::Connect) => {
							if !is_connected {

								if self.connections.len()  < self.max_connections {

									self.connections.insert(client.to_string(), PeerData::new());

									let event = EventType::Connect(client.to_string());
									self.events.push_back(event);

								} else {
									//cannot connect, server is full!
									let event = EventType::ServerFull;
									self.events.push_back(event);
								}
							}
						}

						Some(PacketType::Disconnect) => {
							
						}

						Some(PacketType::Data) => {
							
						}

						Some(PacketType::Ping) => {
							
						}

						Some(PacketType::Receipt) => {
							
						}

						None => {},
					}

					println!("{}", packet.len());
				}
			},

			Err(error) => {
				if error.kind() != std::io::ErrorKind::WouldBlock {
					println!("Error receiving packet: {}", error);
				}
			}
		}
		std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
	}

	pub fn events_available(&self) -> bool {
		return self.events.len() > 0;
	}

	pub fn get_event(&mut self) -> Option<EventType> {
		match self.events.pop_front() {Some(event) => return Some(event),None => return None}
	}
}