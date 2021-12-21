use super::Packet;

#[allow(dead_code)]
impl Packet {
    pub fn new() -> Packet {
        return Packet {
            data: Vec::new(),
            read_position: 0,
        }
    }

    pub fn write_bytes(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice);
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn write_u8(&mut self, value: &u8) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_u16(&mut self, value: &u16) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_u32(&mut self, value: &u32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_u64(&mut self, value: &u64) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_u128(&mut self, value: &u128) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_i8(&mut self, value: &i8) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_i16(&mut self, value: &i16) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_i32(&mut self, value: &i32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_i64(&mut self, value: &i64) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_i128(&mut self, value: &i128) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_f32(&mut self, value: &f32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_f64(&mut self, value: &f64) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut array = [0u8; 1];
        array.clone_from_slice(&self.data[self.read_position..self.read_position+1]);
        self.read_position += 1;
        return u8::from_le_bytes(array);
    }
}