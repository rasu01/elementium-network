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
							events: std::collections::VecDeque::new(),
							internal_packet_count: 0
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

					let client_address = client.to_string();

					let is_connected = self.connections.contains_key(&client_address);

					println!("Packet Type: {}", packet_type);

					match PacketType::from_u8(packet_type) {

						Some(PacketType::Connect) => {
							if !is_connected {

								if self.connections.len()  < self.max_connections {

									self.connections.insert(client.to_string(), PeerData::new());

									let event = EventType::Connect(client.to_string());
									self.events.push_back(event);

									self.send_connection_status(&client_address, true);

								} else {
									//cannot connect, server is full!
									let event = EventType::ServerFull;
									self.events.push_back(event);

									self.send_connection_status(&client_address, false);
								}
							}
						}

						Some(PacketType::Disconnect) => {
							
						}

						Some(PacketType::Data) => {
							
						}

						Some(PacketType::Ping) => {
							println!("PING");
						}

						Some(PacketType::Receipt) => {
							
						}

						None => {},
					}
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

	fn send_connection_status(&mut self, peer: &String, accepted: bool) {
		let mut packet = Packet::new();

		packet.write::<u8>(&1); //packet type
		packet.write::<u8>(&INTERNAL_CHANNEL); //channel id
		packet.write::<u128>(&self.internal_packet_count);

		packet.write::<u8>(&(accepted as u8));
		packet.write::<u32>(&0x1); //reliable data
		packet.write::<u32>(&0x1); //sequence data

		//store packet
		self.internal_send(peer, &packet);
		self.internal_packet_count += 1;
	}

	fn internal_send(&mut self, peer: &String, packet: &Packet) {
		match self.socket.send_to(packet.slice(), peer) {
			Ok(_) => {},
			Err(e ) => {
				println!("Unable to send. Error: {}", e);
			}
		}
	}
}