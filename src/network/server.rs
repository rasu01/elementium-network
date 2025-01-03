use std::collections::VecDeque;
use super::*;

#[macro_use]
mod server_macros {
	macro_rules! internal_send {
		($self:expr, $packet:expr, $peer:expr) => {
			match $self.socket.send_to($packet.slice(), $peer.to_string()) {
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
							stored_packets: std::collections::HashMap::new(),
							sequence: 0,
							reliable: 0,
							stored_packets_to_remove: VecDeque::new()
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

	pub fn update(&mut self) {

		loop {
			match self.socket.recv_from(&mut self.receive_buffer) {
				Ok((packet_size, client)) => {
					if packet_size >= PACKET_HEADER_SIZE { //We are not accepting packets less than this..

						let mut packet = Packet::new(); //TODO: move into struct
						packet.push_bytes(&self.receive_buffer[0..packet_size]);
		
						packet.set_read_position(packet_size - PACKET_HEADER_SIZE); //pack header is at the end.
						let packet_header = packet.read::<PacketHeader>();
						packet.reset_read_position(); //now reset it to the start again

						let client_address = client.to_string();
						let is_connected = self.connections.contains_key(&client_address);
		
						match packet_header.packet_type {
		
							PacketType::Connect => {
								if !is_connected {
		
									if self.connections.len()  < self.max_connections {
		
										self.connections.insert(client.to_string(), PeerData::new());
		
										let event = ServerEvent::Connect(client.to_string());
										self.events.push_back(event);
		
										self.send_connection_status(&client_address, true);
		
									} else {
										//cannot connect, server is full!
										let event = ServerEvent::ServerFull(client_address.clone());
										self.events.push_back(event);
										self.send_connection_status(&client_address, false);
									}
									
								}
								self.send_receipt(&client_address, &packet_header);
							}
		
							PacketType::Disconnect => {

								self.send_receipt(&client_address, &packet_header);
								if is_connected {
									self.connections.remove(&client_address);
									self.events.push_back(ServerEvent::Disconnect(client_address));
								}
							}
		
							PacketType::Data => {
								if is_connected {

									match self.get_channel_type(packet_header.channel_id) {

										ChannelType::Reliable => {
											self.send_receipt(&client_address, &packet_header);
											if let Some(peer_data) = self.connections.get_mut(&client_address) {
												if !peer_data.packets_already_received[packet_header.channel_id as usize].contains_key(&packet_header.packet_id) {
													peer_data.packets_already_received[packet_header.channel_id as usize].insert(packet_header.packet_id, std::time::Instant::now());
													self.events.push_back(ServerEvent::Data(packet, client_address));
												}
											}
										}

										ChannelType::Sequenced => {
											self.send_receipt(&client_address, &packet_header);
											if let Some(peer_data) = self.connections.get_mut(&client_address) {

												//check if the packet isn't an old one
												if !peer_data.packets_already_received[packet_header.channel_id as usize].contains_key(&packet_header.packet_id) {

													//queue the data if it's the packet after the recently receieved one. (aka, is it in order?)
													if peer_data.receive_packet_count[packet_header.channel_id as usize] == packet_header.packet_id {
														peer_data.receive_packet_count[packet_header.channel_id as usize] += 1;
														self.events.push_back(ServerEvent::Data(packet, client_address.clone()));
													} else { //else queue it to wait for the packets in between this and the packet in order to arrive.
														peer_data.stored_sequenced_packets[packet_header.channel_id as usize].insert(packet_header.packet_id, packet);
													}

													self.stored_packets_to_remove.clear();
													//after this we should check if we can queue some of the stored packets(if there are any :P)
													while let Some((id, stored_packet)) = peer_data.stored_sequenced_packets[packet_header.channel_id as usize].get_key_value(&peer_data.receive_packet_count[packet_header.channel_id as usize]) {
														self.events.push_back(ServerEvent::Data(stored_packet.clone(), client_address.clone()));
														peer_data.receive_packet_count[packet_header.channel_id as usize] += 1;
														self.stored_packets_to_remove.push_back(*id);
													}
													//lastly remove the stored packets.
													for id in &self.stored_packets_to_remove {
														peer_data.stored_sequenced_packets[packet_header.channel_id as usize].remove(&id);
													}
													peer_data.packets_already_received[packet_header.channel_id as usize].insert(packet_header.packet_id, std::time::Instant::now());
												}
											}
										}

										ChannelType::Nonreliable => {
											self.events.push_back(ServerEvent::Data(packet, client_address));
										}

										ChannelType::NonreliableDropable => {
											if let Some(peer_data) = self.connections.get_mut(&client_address) {
												if peer_data.receive_packet_count[packet_header.channel_id as usize] < packet_header.packet_id || packet_header.packet_id == 0 {
													self.events.push_back(ServerEvent::Data(packet, client_address));
													peer_data.receive_packet_count[packet_header.channel_id as usize] = packet_header.packet_id;
												}
											}
										}
									}
								}
							}
		
							PacketType::Ping => {
								if is_connected {
									if let Some(connection) = self.connections.get_mut(&client_address) {
										connection.update_timeout();
										self.send_ping(&client_address);
										self.events.push_back(ServerEvent::Ping(client_address));
									}
								}
							}
		
							PacketType::Receipt => {
								let spi = StoredPacketIdentifier::new(client_address, packet_header.channel_id, packet_header.packet_id);
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
		}

		self.internal_update();
	}

	pub fn internal_update(&mut self) {

		self.connections.retain(|peer, data | {
			if data.has_timed_out() {
				self.stored_packets.retain(|spi, _| spi.peer != peer.to_string()); //remove stored packets

				//send event
				self.events.push_back(ServerEvent::Timeout(peer.to_string()));
				return false;
			} else {

				//check already received packets timeouts.
				for i in 0..32 {
					data.packets_already_received[i].retain(|_,  &mut timer| !(timer.elapsed().as_millis() >= 5000));
				}
				return true;
			}
		});

		//check the stored packets timers
		self.stored_packets.iter_mut().for_each(|(spi, sp)| {
			if sp.has_timed_out() {
				server_macros::internal_send!(self, sp.packet, spi.peer);
				sp.update_timeout();
			}
		});
	}

	pub fn get_event(&mut self) -> Option<ServerEvent> {
		match self.events.pop_front() {Some(event) => return Some(event),None => return None}
	}

	fn send_connection_status(&mut self, peer: &String, accepted: bool) {
		let mut packet = Packet::new();
		let packet_header = PacketHeader::new(PacketType::Connect, INTERNAL_CHANNEL, self.internal_packet_count);

		packet.push::<bool>(&accepted);
		packet.push::<u32>(&self.reliable); //reliable data
		packet.push::<u32>(&self.sequence); //sequence data
		packet.push::<PacketHeader>(&packet_header); //packet header at the end!

		self.internal_send(peer, &packet);
		self.store_packet(&peer, INTERNAL_CHANNEL, self.internal_packet_count, &packet);
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
		server_macros::internal_send!(self, packet, peer);
	}

	fn store_packet(&mut self, peer: &String, channel: u8, packet_id: u128, packet: &Packet) {
		let spi = StoredPacketIdentifier::new(peer.clone(), channel, packet_id);
		let sp = StoredPacket::new(&packet);
		self.stored_packets.insert(spi, sp);
	}

	#[allow(dead_code)]
	pub fn send_to_peer(&mut self, peer: &String, channel: u8, mut packet: Packet) {

		if let Some(peer_data) = self.connections.get_mut(peer) { //can this be done faster? Add packet header at the end of the packet instead..

			//add packet header to the end
			packet.push::<PacketHeader>(&PacketHeader::new(PacketType::Data, channel, peer_data.send_packet_count[channel as usize]));

			let packet_id = peer_data.send_packet_count[channel as usize];
			peer_data.send_packet_count[channel as usize] += 1;

			let channel_type = self.get_channel_type(channel);
			if channel_type == ChannelType::Reliable || channel_type == ChannelType::Sequenced {
				self.store_packet(peer, channel, packet_id, &packet);
			}
			self.internal_send(&peer, &packet);
			
		} else {
			println!("WARNING! Sending to a non-connected peer.");
		}
	}

	#[allow(dead_code)]
	pub fn send_to_all(&mut self, channel: u8, packet: Packet) {
		let mut peers = VecDeque::new();
		for peer in self.connections.keys() {
			peers.push_back(peer.clone());
		}
		for peer in peers {
			self.send_to_peer(&peer, channel, packet.clone());
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

	#[allow(dead_code)]
	pub fn setup_channel(&mut self, channel: u8, channel_type: ChannelType) {

		assert!(channel < 32, "Channel id too high. Range is 0..32");

		let rel;
		let seq;
		match channel_type {
			ChannelType::Reliable => {seq = false; rel = true;}
			ChannelType::Sequenced => {seq = true; rel = true;}
			ChannelType::Nonreliable => {seq = false; rel = false;}
			ChannelType::NonreliableDropable => {seq = true; rel = false;}
		}

		if rel {
			self.reliable = self.reliable | (1 << channel);
		} else {
			self.reliable = !(!self.reliable | (1 << channel));
		}
		if seq {
			self.sequence = self.sequence | (1 << channel);
		} else {
			self.sequence = !(!self.sequence | (1 << channel));
		}
	}
}