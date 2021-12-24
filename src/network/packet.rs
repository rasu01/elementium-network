use super::*;

#[allow(dead_code)]
impl Packet {
    pub fn new() -> Packet {
        return Packet {
            data: Vec::new(),
            read_position: 0,
        }
    }

    pub fn slice(&self) -> &[u8] {
        return &self.data[0..self.len()];
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn write_bytes(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice);
    }

    pub fn write<T>(&mut self, value: &T) {
        unsafe {
            let pointer = value as *const T;
            let size = std::mem::size_of::<T>();
            self.data.extend_from_slice(std::slice::from_raw_parts(std::mem::transmute::<*const T, *const u8>(pointer), size));
        }
    }

    pub fn read<T: Copy>(&mut self) -> Option<T> {
        unsafe {
            if std::mem::size_of::<T>() + self.read_position <= self.data.len() {
                let pointer = std::mem::transmute::<* const u8, *const T>(self.data[self.read_position..self.read_position + std::mem::size_of::<T>()].as_ptr());
                self.read_position += std::mem::size_of::<T>();
                if let Some(value) = pointer.as_ref() {
                    return Some(value.clone());
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
}

impl PacketHeader {
    pub fn new(packet_type: u8, channel_id: u8, packet_id: u32) -> PacketHeader {
        return PacketHeader {packet_type,channel_id,packet_id}
    }
}

impl StoredPacket {
    pub fn new(packet: Packet) -> StoredPacket {
        return StoredPacket {
            timer: std::time::Instant::now(),
            packet: packet, //since the stored packet takes ownership here, this function should be called last.
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