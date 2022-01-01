use super::PeerData;

impl PeerData {
	pub fn new() -> PeerData {
		return PeerData {
			timer: std::time::Instant::now(),
			receive_packet_count: [0u128; 32],
			send_packet_count: [0u128; 32],
			stored_sequenced_packets: Default::default(),
			packets_already_received: Default::default()
		}
	}

	pub fn has_timed_out(&self) -> bool {
		if self.timer.elapsed().as_millis() >= 5000 {
			return true;
		} else {
			return false;
		}
	}
	pub fn update_timeout(&mut self) {
		self.timer = std::time::Instant::now();
	}
}