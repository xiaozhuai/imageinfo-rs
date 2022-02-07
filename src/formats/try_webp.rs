use std::cmp::min;
use std::io::{BufRead, Seek};
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

// https://developers.google.com/speed/webp/docs/riff_container
pub fn try_webp<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 16 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, min(length, 30))?;
    if !buffer.cmp_any_of(0, 4, vec![b"RIFF", b"WEBP"]) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let mut ret = ImageInfo {
        format: ImageFormat::WEBP,
        ext: "webp",
        full_ext: "webp",
        mimetype: "image/webp",
        size: ImageSize {
            width: 0,
            height: 0,
        },
        entry_sizes: vec![],
    };

    if buffer.cmp(12, 4, b"VP8 ") && buffer.len() >= 30 {
        ret.size.width = (buffer.read_u16_le(26) & 0x3FFF) as i64;
        ret.size.height = (buffer.read_u16_le(28) & 0x3FFF) as i64;
    } else if buffer.cmp(12, 4, b"VP8L") && buffer.len() >= 25 {
        let n = buffer.read_u32_le(21);
        ret.size.width = ((n & 0x3FFF) + 1) as i64;
        ret.size.height = (((n >> 14) & 0x3FFF) + 1) as i64;
    } else if buffer.cmp(12, 4, b"VP8X") && buffer.len() >= 30 {
        let extended_header = buffer.read_u8(20);
        let valid_start = (extended_header & 0xC0) == 0;
        let valid_end = (extended_header & 0x01) == 0;
        if valid_start && valid_end {
            ret.size.width = ((buffer.read_u32_le(24) & 0x00FFFFFF) + 1) as i64;
            ret.size.height = (((buffer.read_u32_le(26) & 0xFFFFFF00) >> 8) + 1) as i64;
        }
    }

    Ok(ret)
}

