use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

// https://www.fileformat.info/format/jpeg/corion.htm
pub fn try_jpg<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 2 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 2)?;
    if !buffer.cmp(0, 2, b"\xFF\xD8") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let mut ret = ImageInfo {
        format: ImageFormat::JPEG,
        ext: "jpg",
        full_ext: "jpeg",
        mimetype: "image/jpeg",
        size: ImageSize {
            width: 0,
            height: 0,
        },
        entry_sizes: vec![],
    };

    let mut offset = 2usize;
    while offset + 9 <= length {
        let buffer = ri.read(offset, 9)?;
        let section_size = buffer.read_u16_be(2) as usize;
        if !buffer.cmp(0, 1, b"\xFF") {
            // skip garbage bytes
            offset += 1;
            continue;
        }

        // 0xFFC0 is baseline standard (SOF0)
        // 0xFFC1 is baseline optimized (SOF1)
        // 0xFFC2 is progressive (SOF2)
        if buffer.cmp_any_of(0, 2, vec![b"\xFF\xC0", b"\xFF\xC1", b"\xFF\xC2"]) {
            ret.size.width = buffer.read_u16_be(7) as i64;
            ret.size.height = buffer.read_u16_be(5) as i64;
            return Ok(ret);
        }
        offset += section_size + 2;
    }

    Err(ImageInfoError::UnrecognizedFormat)
}
