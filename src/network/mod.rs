pub mod server;

pub struct Server {
	socket: std::net::UdpSocket,
	max_connections: u32,
	connections: std::collections::HashMap<String, PeerData>
}

pub struct PeerData {
	
}