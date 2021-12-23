use std::u128;

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

    //write
    pub fn write_u8(&mut self, value: &u8) {
        let bytes = value.to_le_bytes();
        self.data.extend_from_slice(&bytes);
    }

    pub fn write_u32(&mut self, value: &u32) {
        let bytes = value.to_le_bytes();
        self.data.extend_from_slice(&bytes);
    }

    //read
    pub fn read_u8(&mut self) -> u8 {
        let mut array = [0u8;1];
        array.clone_from_slice(&self.data[self.read_position..self.read_position+1]);
        self.read_position += 1;
        return u8::from_le_bytes(array);
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut array = [0u8; 4];
        array.clone_from_slice(&self.data[self.read_position..self.read_position+4]);
        self.read_position += 4;
        return u32::from_le_bytes(array);
    }
    
}

impl PacketHeader {
    pub fn new(packet_type: u8, channel_id: u8, packet_id: u32) -> PacketHeader {
        return PacketHeader {packet_type,channel_id,packet_id}
    }
}