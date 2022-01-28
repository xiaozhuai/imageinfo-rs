use std::io::{Seek, BufRead, SeekFrom};
use crate::{RawBuffer, ImageInfoResult};

pub struct ReadInterface<R> where R: BufRead + Seek {
    pub reader: R,
}

impl<R> ReadInterface<R> where R: BufRead + Seek {
    pub fn read(&mut self, offset: usize, size: usize) -> ImageInfoResult<RawBuffer> {
        self.reader.seek(SeekFrom::Start(offset as u64))?;
        let mut buffer = RawBuffer {
            data: vec![0; size],
        };
        self.reader.read_exact(buffer.data.as_mut_slice())?;
        Ok(buffer)
    }
}
