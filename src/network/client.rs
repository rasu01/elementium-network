use super::*;

impl Client {
    pub fn new(address: String) -> std::result::Result<Client, std::io::Error> {

        match std::net::UdpSocket::bind("0.0.0.0:0") {
			Ok(sock) => {

				match sock.set_nonblocking(true) {

					Ok(_) => {

						let mut client = Client { 
                            socket: sock,
                            receive_buffer: [0; 60000], 
                            address: address, 
                            sequence: 0, 
                            reliable: 0, 
							is_connected: false,
                            events: std::collections::VecDeque::new(),
							stored_packets: std::collections::HashMap::new(),
							connection_timeout: std::time::Instant::now(),
							internal_packet_count: 0,
                        };

						client.connect();

						return Ok(client);
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

	fn connect(&mut self) {
		if !self.is_connected {
			let packet_header = PacketHeader::new(PacketType::Connect, INTERNAL_CHANNEL, self.internal_packet_count);
			let mut packet = Packet::new();
			packet.push::<PacketHeader>(&packet_header);
			self.internal_send(&packet);
			self.store_packet(INTERNAL_CHANNEL, self.internal_packet_count, &packet);
			self.internal_packet_count+=1;
		}
	}

	pub fn update(&mut self, sleep_time: f64) {
		loop {
			match self.socket.recv(&mut self.receive_buffer) {

				Ok(packet_size) => {

					if packet_size >= PACKET_HEADER_SIZE { //We are not accepting packets less than this..

						let mut packet = Packet::new(); //TODO: move into struct
						packet.push_bytes(&self.receive_buffer[0..packet_size]);
		
						packet.set_read_position(packet_size - PACKET_HEADER_SIZE); //pack header is at the end.
						let packet_header = packet.read::<PacketHeader>();
						packet.reset_read_position(); //now reset it to the start again

						match packet_header.packet_type {
							PacketType::Connect => {
								if !self.is_connected {

									let accepted = packet.read::<bool>();
									if accepted {
										self.is_connected = true;
										self.reliable = packet.read::<u32>();
										self.sequence = packet.read::<u32>();

										let event = ClientEvent::Connect;
										self.events.push_back(event);

									} else {
										self.is_connected = false;

										let event = ClientEvent::ConnectionDenied;
										self.events.push_back(event);
									}

								}

								self.send_receipt(&packet_header);
							}
							PacketType::Disconnect => {
								if self.is_connected {

									self.is_connected = false;

									let event = ClientEvent::Disconnect;
									self.events.push_back(event);

									//send receipt not needed here since the server sent this shut down message.
								}
							}

							PacketType::Data => {

							}

							PacketType::Ping => {
								if self.is_connected {
									self.connection_timeout = std::time::Instant::now();

									let event = ClientEvent::Ping;
									self.events.push_back(event);
								}
							}

							PacketType::Receipt => {
								let spi = StoredPacketIdentifier::new(self.address.clone(), packet_header.channel_id, packet_header.packet_id);
								self.stored_packets.remove(&spi);
							}

							PacketType::Undefined => {
								println!("Packet Header type was undefined.");
							}
						}
					}
				}

				Err(error) => {
					if error.kind() != std::io::ErrorKind::WouldBlock {
						println!("Error receiving packet: {}", error);
						break;
					}
				}
			}

			self.internal_update();
			std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
		}
	}

	pub fn get_event(&mut self) -> Option<ClientEvent> {
		match self.events.pop_front() {Some(event) => return Some(event), None => return None}
	}

	fn internal_update(&mut self) {

		//check ping timer.. todo

		//check if connection timed out.
		if self.connection_timeout.elapsed().as_millis() >= 5000 {
			self.is_connected = false;
			self.connection_timeout = std::time::Instant::now();

			let event = ClientEvent::Timeout;
			self.events.push_back(event);
		}

	}

	fn send_receipt(&mut self, packet_header: &PacketHeader) {
		let mut packet = Packet::new();
		let receipt_packet_header = PacketHeader::new(PacketType::Receipt, packet_header.channel_id, packet_header.packet_id);
		packet.push::<PacketHeader>(&receipt_packet_header);
		self.internal_send(&packet);
	}

	fn internal_send(&mut self, packet: &Packet) {
		match self.socket.send_to(packet.slice(), &self.address) {
			Ok(_) => {},
			Err(e ) => {
				println!("Unable to send. Error: {}", e);
			}
		}
	}

	fn store_packet(&mut self, channel: u8, packet_id: u128, packet: &Packet) {

	}
}