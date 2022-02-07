use std::io::{BufRead, Seek};
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

// https://www.fileformat.info/format/tiff/corion.htm
pub fn try_tiff<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 8 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 8)?;
    if !buffer.cmp_any_of(0, 4, vec![b"\x49\x49\x2A\x00", b"\x4D\x4D\x00\x2A"]) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let little_endian = buffer.data[0] == 0x49;
    let mut offset = if little_endian {
        buffer.read_u32_le(4) as usize
    } else {
        buffer.read_u32_be(4) as usize
    };
    if length < offset + 2 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let buffer = ri.read(offset, 2)?;

    let mum_entry = if little_endian {
        buffer.read_i16_le(0) as usize
    } else {
        buffer.read_i16_be(0) as usize
    };
    offset += 2;

    let mut ret = ImageInfo {
        format: ImageFormat::TIFF,
        ext: "tif",
        full_ext: "tiff",
        mimetype: "image/tiff",
        size: ImageSize {
            width: -1,
            height: -1,
        },
        entry_sizes: vec![],
    };

    let mut i = 0usize;
    while i < mum_entry && length >= offset + 12 && (ret.size.width == -1 || ret.size.height == -1) {
        let buffer = ri.read(offset, 12)?;

        let tag = if little_endian {
            buffer.read_u16_le(0)
        } else {
            buffer.read_u16_be(0)
        };

        let t = if little_endian {
            buffer.read_u16_le(2)
        } else {
            buffer.read_u16_be(2)
        };

        if tag == 256 {
            if t == 3 {
                ret.size.width = if little_endian {
                    buffer.read_u16_le(8) as i64
                } else {
                    buffer.read_u16_be(8) as i64
                };
            } else if t == 4 {
                ret.size.width = if little_endian {
                    buffer.read_u32_le(8) as i64
                } else {
                    buffer.read_u32_be(8) as i64
                };
            }
        } else if tag == 257 {
            if t == 3 {
                ret.size.height = if little_endian {
                    buffer.read_u16_le(8) as i64
                } else {
                    buffer.read_u16_be(8) as i64
                };
            } else if t == 4 {
                ret.size.height = if little_endian {
                    buffer.read_u32_le(8) as i64
                } else {
                    buffer.read_u32_be(8) as i64
                };
            }
        }

        i += 1;
        offset += 12;
    }

    Ok(ret)
}

