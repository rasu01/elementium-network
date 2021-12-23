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

    pub fn write<T>(&mut self, value: &T) {
        unsafe {
            let pointer = value as *const T;
            let size = std::mem::size_of::<T>();
            self.data.extend_from_slice(std::slice::from_raw_parts(std::mem::transmute::<*const T, *const u8>(pointer), size));
        }
    }

    pub fn read<T: Copy>(&mut self) -> T {
        unsafe {
            let pointer = std::mem::transmute::<* const u8, *const T>(self.data[self.read_position..self.read_position + std::mem::size_of::<T>()].as_ptr());
            self.read_position += std::mem::size_of::<T>();
            return pointer.as_ref().unwrap().clone();
        }
    }

    pub fn write_bytes(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice);
    }
}