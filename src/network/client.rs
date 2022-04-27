use super::*;

#[macro_use]
mod client_macros {
	macro_rules! internal_send {
		($self:expr, $packet:expr) => {
			match $self.socket.send_to($packet.slice(), &$self.address) {
				Ok(_) => {},
				Err(e ) => {
					println!("Unable to send. Error: {}", e);
				}
			}
		};
	}

	pub(super) use internal_send;
}

#[allow(dead_code)]
impl Client {
    pub fn new(address: String) -> std::result::Result<Client, std::io::Error> {

        match std::net::UdpSocket::bind("0.0.0.0:0") {
			Ok(sock) => {

				match sock.set_nonblocking(true) {

					Ok(_) => {

						let mut client = Client { 
                            socket: sock,
                            receive_buffer: [0; 60000], 
                            address, 
                            sequence: 0, 
                            reliable: 0, 
							is_connected: false,
                            events: std::collections::VecDeque::new(),
							stored_packets: std::collections::HashMap::new(),
							connection_timeout: std::time::Instant::now(),
							internal_packet_count: 0,
							receive_packet_count: [0; 32],
							send_packet_count: [0; 32],
							stored_sequenced_packets: Default::default(),
							packets_already_received: Default::default(),
							ping_timer: std::time::Instant::now(),
        					stored_packets_to_remove: VecDeque::new(),
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

	//sends a packet to the server if the client is connected
	pub fn send(&mut self, channel: u8, mut packet: Packet) {
		if self.is_connected {
			//add packet header to the end
			packet.push::<PacketHeader>(&PacketHeader::new(PacketType::Data, channel, self.send_packet_count[channel as usize]));

			let packet_id = self.send_packet_count[channel as usize];
			self.send_packet_count[channel as usize] += 1;

			let channel_type = self.get_channel_type(channel);
			if channel_type == ChannelType::Reliable || channel_type == ChannelType::Sequenced {
				self.store_packet(channel, packet_id, &packet);
			}
			self.internal_send(&packet);
		}
	}

	fn get_channel_type(&self, channel_id: u8) -> ChannelType {	
		let is_sequenced = (self.sequence >> channel_id) & 0x1 == 1;
		let is_reliable = (self.reliable >> channel_id) & 0x1 == 1;

		if is_sequenced && is_reliable {
			return ChannelType::Sequenced;
		} else if !is_sequenced && is_reliable {
			return ChannelType::Reliable;
		} else if is_sequenced && !is_reliable {
			return ChannelType::NonreliableDropable; 
		} else{
			return ChannelType::Nonreliable;
		}
	}

	//should be used if we lost connection
	//NOTE, this function creates a new socket, which means the port will change!
	fn reconnect(&mut self) -> std::result::Result<(), std::io::Error> {
		match std::net::UdpSocket::bind("0.0.0.0:0") {
			Ok(sock) => {

				match sock.set_nonblocking(true) {

					Ok(_) => {

						//reset some of the values that should be "Defaulted" when reconnecting. 
						//the same fields that should be deafult when actually connecting for the first time
						self.socket = sock;
						self.sequence = 0;
						self.reliable = 0;
						self.is_connected = false;
						self.events = std::collections::VecDeque::new();
						self.stored_packets = std::collections::HashMap::new();
						self.connection_timeout = std::time::Instant::now();
						self.internal_packet_count = 0;
						self.receive_packet_count= [0; 32];
						self.send_packet_count= [0; 32];
						self.stored_sequenced_packets= Default::default();
						self.packets_already_received= Default::default();
						self.ping_timer = std::time::Instant::now();

						//send connect packet again
						self.connect();

						//send an event to signal we are trying to reconnect
						self.events.push_back(ClientEvent::Reconnecting);

						return Ok(());
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
										self.events.push_back(ClientEvent::Connect);

									} else {
										self.is_connected = false;
										self.events.push_back(ClientEvent::ConnectionDenied);
									}

								}

								self.send_receipt(&packet_header);
							}
							PacketType::Disconnect => {
								if self.is_connected {

									self.is_connected = false;
									self.events.push_back(ClientEvent::Disconnect);

									//try to reconnect now that we lost the connection
									//even if it was the server who disconnected, we should do it anyways
									//since there could be a chance it could get back up again
									if let Err(_) = self.reconnect() {

										//exit if we cannot create a new socket(which the reconnect function does)
										println!("Unable to reconnect after an attempt was made!");
										std::process::exit(1);
									}

									//send receipt not needed here since the server sent this shut down message.
								}
							}

							PacketType::Data => {
								match self.get_channel_type(packet_header.channel_id) {

									ChannelType::Reliable => {
										self.send_receipt(&packet_header);
										if !self.packets_already_received[packet_header.channel_id as usize].contains_key(&packet_header.packet_id) {
											self.packets_already_received[packet_header.channel_id as usize].insert(packet_header.packet_id, std::time::Instant::now());
											self.events.push_back(ClientEvent::Data(packet));
										}
									}

									ChannelType::Sequenced => {
										self.send_receipt(&packet_header);

										//check if the packet isn't an old one
										if !self.packets_already_received[packet_header.channel_id as usize].contains_key(&packet_header.packet_id) {

											//queue the data if it's the packet after the recently receieved one. (aka, is it in order?)
											if self.receive_packet_count[packet_header.channel_id as usize] == packet_header.packet_id {
												self.receive_packet_count[packet_header.channel_id as usize] += 1;
												self.events.push_back(ClientEvent::Data(packet));
											} else { //else queue it to wait for the packets in between this and the packet in order to arrive.
												self.stored_sequenced_packets[packet_header.channel_id as usize].insert(packet_header.packet_id, packet);
											}

											self.stored_packets_to_remove.clear();
											//after this we should check if we can queue some of the stored packets(if there are any :P)
											while let Some((id, stored_packet)) = self.stored_sequenced_packets[packet_header.channel_id as usize].get_key_value(&self.receive_packet_count[packet_header.channel_id as usize]) {
												self.events.push_back(ClientEvent::Data(stored_packet.clone()));
												self.receive_packet_count[packet_header.channel_id as usize] += 1;
												self.stored_packets_to_remove.push_back(*id);
											}
											//lastly remove the stored packets.
											for id in &self.stored_packets_to_remove {
												self.stored_sequenced_packets[packet_header.channel_id as usize].remove(&id);
											}
											self.packets_already_received[packet_header.channel_id as usize].insert(packet_header.packet_id, std::time::Instant::now());
										}
									}

									ChannelType::Nonreliable => {
										self.events.push_back(ClientEvent::Data(packet));
									}

									ChannelType::NonreliableDropable => {
										if self.receive_packet_count[packet_header.channel_id as usize] < packet_header.packet_id || packet_header.packet_id == 0 {
											self.events.push_back(ClientEvent::Data(packet));
											self.receive_packet_count[packet_header.channel_id as usize] = packet_header.packet_id;
										}
										//else if the packet is old, do nothing and just let rust remove the packet when the scope ends
									}
								}
							}

							PacketType::Ping => {
								if self.is_connected {
									self.connection_timeout = std::time::Instant::now();
									self.events.push_back(ClientEvent::Ping);
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
					} else {
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

	fn send_ping(&mut self) {
		let mut packet = Packet::new();
		let packet_header = PacketHeader::new(PacketType::Ping, INTERNAL_CHANNEL, 0);
		packet.push::<PacketHeader>(&packet_header);
		self.internal_send(&packet);
	}

	fn internal_update(&mut self) {

		//check ping timer..
		if self.is_connected {
			if self.ping_timer.elapsed().as_millis() >= 1000 {
				self.send_ping();
				self.ping_timer = std::time::Instant::now();
			}
		}

		//check if connection timed out.
		if self.connection_timeout.elapsed().as_millis() >= 5000 {
			self.is_connected = false;
			self.connection_timeout = std::time::Instant::now();

			let event = ClientEvent::Timeout;
			self.events.push_back(event);

			//try to reconnect now that we lost the connection
			if let Err(_) = self.reconnect() {

				//exit if we cannot create a new socket(which the reconnect function does)
				println!("Unable to reconnect after an attempt was made!");
				std::process::exit(1);
			}
		}

		//check stored packets timer
		self.stored_packets.iter_mut().for_each(|(_, sp)| {
			if sp.has_timed_out() {
				client_macros::internal_send!(self, sp.packet);
				sp.update_timeout();
			}
		});

		//check already received packets timeouts (this is probably slow since it has to iterate the whole array)
		for i in 0..32 {
			self.packets_already_received[i].retain(|_,  &mut timer| !(timer.elapsed().as_millis() >= 5000));
		}
	}

	fn send_receipt(&mut self, packet_header: &PacketHeader) {
		let mut packet = Packet::new();
		let receipt_packet_header = PacketHeader::new(PacketType::Receipt, packet_header.channel_id, packet_header.packet_id);
		packet.push::<PacketHeader>(&receipt_packet_header);
		self.internal_send(&packet);
	}

	fn internal_send(&mut self, packet: &Packet) {
		client_macros::internal_send!(self, packet);
	}

	fn store_packet(&mut self, channel: u8, packet_id: u128, packet: &Packet) {
		//since we only receive packets from the server, we can use the server's address as "peer"
		let spi = StoredPacketIdentifier::new(self.address.clone(), channel, packet_id);
		let sp = StoredPacket::new(&packet);
		self.stored_packets.insert(spi, sp);
	}
}