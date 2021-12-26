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
    }

    pub fn slice(&self) -> &[u8] {
        return &self.data[0..self.len()];
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    //the write and read functions could take in structs as types too, but I advice against it if you're gonna use it with c++, since rust and c++ may have different value padding in structs.
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

    //primitive write functions
    pub fn push_u8(&mut self, value: &u8) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_u16(&mut self, value: &u16) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_u32(&mut self, value: &u32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_u64(&mut self, value: &u64) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_u128(&mut self, value: &u128) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i8(&mut self, value: &i8) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i16(&mut self, value: &i16) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i32(&mut self, value: &i32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i64(&mut self, value: &i64) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i128(&mut self, value: &i128) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_f32(&mut self, value: &f32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_f64(&mut self, value: &f32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_bool(&mut self, value: &bool) {
        self.data.extend_from_slice(&(*value as u8).to_le_bytes());
    }

    pub fn push_char(&mut self, value: &char) {
        self.data.extend_from_slice(&value.to_digit(10).unwrap().to_le_bytes()); //this can probably be done better..
    }

    //other push functions
    pub fn push_slice(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice);
    }

}

impl PacketHeader {
    pub fn new(packet_type: u8, channel_id: u8, packet_id: u128) -> PacketHeader {
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