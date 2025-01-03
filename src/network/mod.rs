use std::collections::VecDeque;

pub mod server;
pub mod client;
pub mod packet;
pub mod peerdata;
pub mod packetserialize;

const INTERNAL_CHANNEL: u8 = 100;
const PACKET_HEADER_SIZE: usize = 18;
#[derive(Copy, Clone)]
pub enum PacketType {
	Connect,
	Disconnect,
	Data,
	Ping,
	Receipt,
	Undefined
}

pub enum ServerEvent {
	Connect(String), //Client Address
	Disconnect(String), //Client Address
	Timeout(String), //Client Address
	Data(Packet, String), //Packet, Client Address
	ServerFull(String), //Client Address
	Ping(String), //Client address
}

pub enum ClientEvent {
	Connect,
	ConnectionDenied,
	Disconnect,
	Timeout,
	Reconnecting,
	Data(Packet), //Packet
	Ping,
}

#[derive(PartialEq)]
pub enum ChannelType {
	Reliable,
	Sequenced,
	Nonreliable,
	NonreliableDropable
}

pub struct Server {
	socket: std::net::UdpSocket,
	max_connections: usize,
	connections: std::collections::HashMap<String, PeerData>,
	receive_buffer: [u8; 60000],
	events: std::collections::VecDeque<ServerEvent>,
	internal_packet_count: u128,
	stored_packets: std::collections::HashMap<StoredPacketIdentifier, StoredPacket>,
	sequence: u32,
	reliable: u32,
	stored_packets_to_remove: VecDeque<u128>,
}

pub struct Client {
	socket: std::net::UdpSocket,
	receive_buffer: [u8; 60000],
	receive_packet_count: [u128; 32],
	send_packet_count: [u128; 32],
	stored_sequenced_packets: [std::collections::HashMap<u128, Packet>; 32],
	packets_already_received: [std::collections::HashMap<u128, std::time::Instant>; 32],
	address: String,
	sequence: u32,
	reliable: u32,
	is_connected: bool,
	events: std::collections::VecDeque<ClientEvent>,
	stored_packets: std::collections::HashMap<StoredPacketIdentifier, StoredPacket>,
	connection_timeout: std::time::Instant,
	internal_packet_count: u128,
	ping_timer: std::time::Instant,
	stored_packets_to_remove: VecDeque<u128>,
}

#[derive(Eq, PartialEq, Hash)]
struct StoredPacketIdentifier {
	packet_id: u128,
	channel_id: u8,
	peer: String,
}

struct StoredPacket {
	timer: std::time::Instant,
	packet: Packet,
}

struct PeerData {
	timer: std::time::Instant,
	receive_packet_count: [u128; 32],
	send_packet_count: [u128; 32],
	stored_sequenced_packets: [std::collections::HashMap<u128, Packet>; 32],
	packets_already_received: [std::collections::HashMap<u128, std::time::Instant>; 32]
}

pub struct Packet {
	data: Vec<u8>,
	read_position: usize,
}

#[derive(Copy, Clone)]
struct PacketHeader {
	packet_id: u128,
	packet_type: PacketType,
	channel_id: u8,
}

pub trait PacketSerialize {
	type T;
	fn serialize(&self, packet: &mut Packet);
	fn deserialize(packet: &mut Packet) -> (Self::T, usize);
}