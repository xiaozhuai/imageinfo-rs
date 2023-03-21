use std::convert::TryInto;
use std::mem::size_of;

pub struct RawBuffer {
    pub data: Vec<u8>,
}

#[allow(dead_code)]
impl RawBuffer {
    pub fn new(length: usize) -> RawBuffer {
        return RawBuffer {
            data: vec![0; length],
        };
    }

    pub fn piece(&self, offset: usize, length: usize) -> &[u8] {
        &(self.data[offset..(offset + length)])
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn read_i8(&self, offset: usize) -> i8 {
        i8::from_le_bytes(self.piece(offset, size_of::<i8>()).try_into().unwrap())
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
        u8::from_le_bytes(self.piece(offset, size_of::<u8>()).try_into().unwrap())
    }

    pub fn read_i16_le(&self, offset: usize) -> i16 {
        i16::from_le_bytes(self.piece(offset, size_of::<i16>()).try_into().unwrap())
    }

    pub fn read_i16_be(&self, offset: usize) -> i16 {
        i16::from_be_bytes(self.piece(offset, size_of::<i16>()).try_into().unwrap())
    }

    pub fn read_u16_le(&self, offset: usize) -> u16 {
        u16::from_le_bytes(self.piece(offset, size_of::<u16>()).try_into().unwrap())
    }

    pub fn read_u16_be(&self, offset: usize) -> u16 {
        u16::from_be_bytes(self.piece(offset, size_of::<u16>()).try_into().unwrap())
    }

    pub fn read_i32_le(&self, offset: usize) -> i32 {
        i32::from_le_bytes(self.piece(offset, size_of::<i32>()).try_into().unwrap())
    }

    pub fn read_i32_be(&self, offset: usize) -> i32 {
        i32::from_be_bytes(self.piece(offset, size_of::<i32>()).try_into().unwrap())
    }

    pub fn read_u32_le(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.piece(offset, size_of::<u32>()).try_into().unwrap())
    }

    pub fn read_u32_be(&self, offset: usize) -> u32 {
        u32::from_be_bytes(self.piece(offset, size_of::<u32>()).try_into().unwrap())
    }

    pub fn read_i64_le(&self, offset: usize) -> i64 {
        i64::from_le_bytes(self.piece(offset, size_of::<i64>()).try_into().unwrap())
    }

    pub fn read_i64_be(&self, offset: usize) -> i64 {
        i64::from_be_bytes(self.piece(offset, size_of::<i64>()).try_into().unwrap())
    }

    pub fn read_u64_le(&self, offset: usize) -> u64 {
        u64::from_le_bytes(self.piece(offset, size_of::<u64>()).try_into().unwrap())
    }

    pub fn read_u64_be(&self, offset: usize) -> u64 {
        u64::from_be_bytes(self.piece(offset, size_of::<u64>()).try_into().unwrap())
    }

    pub fn read_str(&self, offset: usize, length: usize) -> String {
        String::from_utf8_lossy(self.piece(offset, length)).to_string()
    }

    pub fn cmp(&self, offset: usize, length: usize, buf: &[u8]) -> bool {
        self.piece(offset, length) == buf
    }

    pub fn cmp_any_of(&self, offset: usize, length: usize, buf_list: Vec<&[u8]>) -> bool {
        for buf in &buf_list {
            if self.cmp(offset, length, *buf) {
                return true;
            }
        }
        false
    }
}

impl ToString for RawBuffer {
    fn to_string(&self) -> String {
        self.read_str(0usize, self.len())
    }
}
