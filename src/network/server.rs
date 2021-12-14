use super::Server;

impl Server {

	pub fn new(port: u16, max_connections: u32) -> std::result::Result<Server, std::io::Error> {

		match std::net::UdpSocket::bind(format!("0.0.0.0:{}", port)) {
			Ok(sock) => {
				sock.set_nonblocking(true);
				return Ok(Server {
					socket: sock,
					max_connections: max_connections
				});
			}

			Err(_e) => {
				return Err(_e);
			}
		}
	}
}