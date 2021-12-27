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
							internal_packet_count: 0,
							stored_packets: std::collections::HashMap::new()
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

				if packet_size >= 20 { //We are not accepting packets less than this..

					let mut packet = Packet::new();
					packet.push_bytes(&self.receive_buffer[0..packet_size]);

					let packet_header = packet.read::<PacketHeader>();

					let client_address = client.to_string();
					let is_connected = self.connections.contains_key(&client_address);

					match packet_header.packet_type {

						PacketType::Connect => {
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
							self.send_receipt(&client_address, &packet_header);
						}

						PacketType::Disconnect => {
							
						}

						PacketType::Data => {
							
						}

						PacketType::Ping => {
							if is_connected {
								if let Some(connection) = self.connections.get_mut(&client_address) {
									connection.update_timeout();
									self.send_ping(&client_address);
								}
							}
						}

						PacketType::Receipt => {
							
						}

        				PacketType::Undefined => {
							println!("Packet Header was wrongly acquired.");
						}
					}
				}
			},

			Err(error) => {
				if error.kind() != std::io::ErrorKind::WouldBlock {
					println!("Error receiving packet: {}", error);
				}
			}
		}
		self.internal_update();
		std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
	}

	pub fn internal_update(&mut self) {

		let mut peers_to_remove: Vec<String> = Vec::new();

		for (peer, data) in &self.connections {

			//remove timed out peers
			if data.has_timed_out() {
				peers_to_remove.push(peer.to_string());
				continue;
			}

			//TODO: check already received packets timeouts.
		}

		for peer in peers_to_remove {
			//finally remove the peer and send store event
			let event = EventType::Timeout(peer.to_string());
			self.events.push_back(event);
			self.connections.remove(&peer);
		}

		//check the stored packets timers
		let mut timers_to_update: Vec<StoredPacketIdentifier> = Vec::new();
		for (spi, sp) in &self.stored_packets {
			if sp.has_timed_out() {
				self.internal_send(&spi.peer.to_string(), &sp.packet);
				timers_to_update.push(spi.clone());
			}
		}
		for spi in timers_to_update { //this could probably be done better..
			if let Some(sp) = self.stored_packets.get_mut(&spi) {
				sp.update_timeout();
			}
		}
	}

	pub fn get_event(&mut self) -> Option<EventType> {
		match self.events.pop_front() {Some(event) => return Some(event),None => return None}
	}

	fn send_connection_status(&mut self, peer: &String, accepted: bool) {
		let mut packet = Packet::new();
		let packet_header = PacketHeader::new(PacketType::Connect, INTERNAL_CHANNEL, self.internal_packet_count);

		packet.push::<PacketHeader>(&packet_header);
		packet.push::<bool>(&accepted);
		packet.push::<u32>(&0x1); //reliable data
		packet.push::<u32>(&0x1); //sequence data

		//packet.push::<String>(&String::from("日本語を試してくれてありがとう!"));

		//store packet
		self.internal_send(peer, &packet);
		self.internal_packet_count += 1;
	}

	fn send_ping(&self, peer: &String) {
		let mut packet = Packet::new();
		let packet_header = PacketHeader::new(PacketType::Ping, INTERNAL_CHANNEL, 0);
		packet.push::<PacketHeader>(&packet_header);
		self.internal_send(peer, &packet);
	}

	fn send_receipt(&mut self, peer: &String, packet_header: &PacketHeader) {
		let mut packet = Packet::new();
		let receipt_packet_header = PacketHeader::new(PacketType::Receipt, packet_header.channel_id, packet_header.packet_id);
		packet.push::<PacketHeader>(&receipt_packet_header);
		self.internal_send(peer, &packet);
	}

	fn internal_send(&self, peer: &String, packet: &Packet) {
		match self.socket.send_to(packet.slice(), peer) {
			Ok(_) => {},
			Err(e ) => {
				println!("Unable to send. Error: {}", e);
			}
		}
	}
}