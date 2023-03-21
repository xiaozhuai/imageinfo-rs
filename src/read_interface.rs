use crate::{ImageInfoResult, RawBuffer};
use std::cmp::min;
use std::io::{BufRead, Seek, SeekFrom};

pub struct ReadInterface<R>
where
    R: BufRead + Seek,
{
    pub reader: R,
    pub length: usize,
    header_cache: Vec<u8>,
    header_cache_ok: bool,
}

impl<R> ReadInterface<R>
where
    R: BufRead + Seek,
{
    pub fn from_reader(reader: R, length: usize) -> ReadInterface<R> {
        return ReadInterface {
            reader,
            length,
            header_cache: vec![0; 0],
            header_cache_ok: false,
        };
    }
}

fn copy_slice<R>(
    dst: &mut [R],
    dst_offset: usize,
    dst_size: usize,
    src: &[R],
    src_offset: usize,
    src_size: usize,
) where
    R: Copy,
{
    dst[dst_offset..(dst_offset + dst_size)]
        .copy_from_slice(&src[src_offset..(src_offset + src_size)]);
}

impl<R> ReadInterface<R>
where
    R: BufRead + Seek,
{
    pub fn read(&mut self, offset: usize, size: usize) -> ImageInfoResult<RawBuffer> {
        assert!(offset + size <= self.length);
        if !self.header_cache_ok {
            self.header_cache = vec![0; min(self.length, 1024)];
            self.reader.seek(SeekFrom::Start(0u64))?;
            self.reader.read_exact(self.header_cache.as_mut_slice())?;
            self.header_cache_ok = true
        }

        let mut buffer = RawBuffer::new(size);
        if offset + size <= self.header_cache.len() {
            copy_slice(
                buffer.data.as_mut_slice(),
                0,
                size,
                self.header_cache.as_slice(),
                offset,
                size,
            );
        } else if offset < self.header_cache.len()
            && self.header_cache.len() - offset >= (self.header_cache.len() / 4)
        {
            let head = self.header_cache.len() - offset;
            copy_slice(
                buffer.data.as_mut_slice(),
                0,
                head,
                self.header_cache.as_slice(),
                offset,
                head,
            );
            self.reader.seek(SeekFrom::Start((offset + head) as u64))?;
            self.reader.read_exact(&mut buffer.data[head..])?;
        } else {
            self.reader.seek(SeekFrom::Start(offset as u64))?;
            self.reader.read_exact(buffer.data.as_mut_slice())?;
        }
        Ok(buffer)
    }
}
