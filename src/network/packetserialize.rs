use super::{PacketSerialize, Packet, PacketHeader};

//primitive types!
impl PacketSerialize for u8 {
    type T = u8;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for u16 {
    type T = u16;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for u32 {
    type T = u32;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for u64 {
    type T = u64;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for u128 {
    type T = u128;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for i8 {
    type T = i8;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for i16 {
    type T = i16;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for i32 {
    type T = i32;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for i64 {
    type T = i64;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for i128 {
    type T = i128;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for f32 {
    type T = f32;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for f64 {
    type T = f64;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (Self::T::from_le_bytes(array), std::mem::size_of::<Self::T>());
    }
}

impl PacketSerialize for bool {
    type T = bool;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&(*self as u8).to_le_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let mut array =[0u8; std::mem::size_of::<Self::T>()];
        array.copy_from_slice(&packet.data[packet.read_position..packet.read_position+std::mem::size_of::<Self::T>()]);
        return (if u8::from_le_bytes(array) == 1 {true} else {false}, std::mem::size_of::<Self::T>());
    }
}

//nonprimitive
impl PacketSerialize for String {
    type T = String;
    fn serialize(&self, packet: &mut Packet) {
        packet.data.extend_from_slice(&self.as_bytes());
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        return (String::new(), 0); //TODO
    }
}

impl PacketSerialize for PacketHeader {
    type T = PacketHeader;
    fn serialize(&self, packet: &mut Packet) {
        packet.push::<u128>(&self.packet_id);
        packet.push::<u8>(&self.packet_type);
        packet.push::<u8>(&self.channel_id);
    }
    fn deserialize(packet: &mut Packet) -> (Self::T, usize) {
        let id = packet.read::<u128>();
        let packet_type = packet.read::<u8>();
        let channel = packet.read::<u8>();
        return (PacketHeader {
            packet_id: id,
            channel_id: channel,
            packet_type: packet_type
        }, 0); //since we use read here that already changes the read position, we should send a 0 here.
    }
}