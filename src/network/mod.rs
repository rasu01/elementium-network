pub mod server;

pub struct Server {
	socket: std::net::UdpSocket,
	max_connections: u32,
}