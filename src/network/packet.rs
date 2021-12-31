use super::*;

#[allow(dead_code)]
impl Packet {
    pub fn new() -> Packet {
        return Packet {
            data: Vec::new(),
            read_position: 0,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.reset_read_position();
    }

    pub fn reset_read_position(&mut self) {
        self.read_position = 0;
    }

    pub fn set_read_position(&mut self, position: usize) {
        self.read_position = position;
    }

    pub fn slice(&self) -> &[u8] {
        return &self.data[0..self.len()];
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    //other push functions
    pub fn push_bytes(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice);
    }

    pub fn push<T: PacketSerialize>(&mut self, value: &T) { // if you wish to push structs, use this and implement the PacketSerialize trait for it.
        value.serialize(self);
    }
    pub fn read<T: PacketSerialize>(&mut self) -> <T as PacketSerialize>::T {
        let (value, size) = T::deserialize(self);
        self.read_position += size;
        return value;
    }

}

impl Clone for Packet {
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), read_position: self.read_position }
    }
}

impl PacketHeader {
    pub fn new(packet_type: PacketType, channel_id: u8, packet_id: u128) -> PacketHeader {
        return PacketHeader {packet_type,channel_id,packet_id}
    }
}

impl StoredPacket {
    pub fn new(packet: &Packet) -> StoredPacket {
        return StoredPacket {
            timer: std::time::Instant::now(),
            packet: packet.clone(), //since the stored packet takes ownership here, this function should be called last.
        }
    }
    pub fn has_timed_out(&self) -> bool {
		if self.timer.elapsed().as_millis() >= 500 { //TODO: Add the peers ping to this.. What if the ping is over 500 ms???
			return true;
		} else {
			return false;
		}
	}
    pub fn update_timeout(&mut self) {
		self.timer = std::time::Instant::now();
	}
}

impl StoredPacketIdentifier {
    pub fn new(peer: String, channel_id: u8, packet_id: u128) -> StoredPacketIdentifier {
        return StoredPacketIdentifier {
            peer: peer,
            channel_id: channel_id, 
            packet_id: packet_id
        }
    }

    pub fn clone(&self) -> StoredPacketIdentifier {
        return StoredPacketIdentifier {
            peer: self.peer.to_string(),
            channel_id: self.channel_id,
            packet_id: self.packet_id
        }
    }
}