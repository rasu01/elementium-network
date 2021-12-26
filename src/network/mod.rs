pub mod server;
pub mod packet;
pub mod peerdata;
pub mod packetserialize;

const INTERNAL_CHANNEL: u8 = 100;
pub enum PacketType {
	Connect = 0,
	Disconnect = 1,
	Data = 2,
	Ping = 3,
	Receipt = 4,
}
impl PacketType {
	pub fn from_u8(num: u8) -> Option<PacketType> {
		match num {
			0 => return Some(PacketType::Connect),
			1 => return Some(PacketType::Disconnect),
			2 => return Some(PacketType::Data),
			3 => return Some(PacketType::Ping),
			4 => return Some(PacketType::Receipt),
			_ => return None
		}
	}
}

pub enum EventType {
	Connect(String),
	Disconnect(String),
	Timeout(String),
	Data(Packet),
	ServerFull
}

pub struct Server {
	socket: std::net::UdpSocket,
	max_connections: usize,
	connections: std::collections::HashMap<String, PeerData>,
	receive_buffer: [u8; 60000],
	events: std::collections::VecDeque<EventType>,
	internal_packet_count: u128,
	stored_packets: std::collections::HashMap<StoredPacketIdentifier, StoredPacket>,
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

pub struct PeerData {
	timer: std::time::Instant,
	receive_packet_count: [u128; 32],
	send_packet_count: [u128; 32],
	stored_packets: [std::collections::HashMap<u128, Packet>; 32],
	packets_already_received: [std::collections::HashMap<u128, f32>; 32]
}

#[repr(packed(8))]
pub struct Packet {
	data: Vec<u8>,
	read_position: usize,
}

#[derive(Copy,Clone)]
pub struct PacketHeader {
	packet_id: u128,
	packet_type: u8,
	channel_id: u8,
}

pub trait PacketSerialize {
	type T;
	fn serialize(&self, packet: &mut Packet);
	fn deserialize(packet: &mut Packet) -> (Self::T, usize);
}