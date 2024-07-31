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

    let mut orientation = 1u16;
    let mut offset = 2usize;
    while offset + 9 <= length {
        let buffer = ri.read(offset, 9)?;
        let section_size = buffer.read_u16_be(2) as usize;
        if !buffer.cmp(0, 1, b"\xFF") {
            // skip garbage bytes
            offset += 1;
            continue;
        }

        // 0xFFE1 is application 1 (APP1)
        if buffer.cmp(0, 2, b"\xFF\xE1") {
            if offset + section_size + 2 > length {
                return Err(ImageInfoError::UnrecognizedFormat);
            }
            let exif_buffer = ri.read(offset, section_size + 2)?;
            if exif_buffer.cmp(4, 5, b"Exif\x00") {
                let big_endian = !exif_buffer.cmp(10, 1, b"I");
                let first_ifd_offset = if big_endian {
                    exif_buffer.read_u32_be(14)
                } else {
                    exif_buffer.read_u32_le(14)
                };
                if first_ifd_offset < 8
                    || (first_ifd_offset as u64) + 12u64 > (section_size as u64 + 2u64)
                {
                    return Err(ImageInfoError::UnrecognizedFormat);
                }
                let ifd_main_entries_count = if big_endian {
                    exif_buffer.read_u16_be(first_ifd_offset as usize + 10)
                } else {
                    exif_buffer.read_u16_le(first_ifd_offset as usize + 10)
                };
                for i in 0..ifd_main_entries_count {
                    let entry_offset = first_ifd_offset as usize + 12 + 12 * i as usize;
                    if entry_offset + 12 > section_size + 2 {
                        return Err(ImageInfoError::UnrecognizedFormat);
                    }
                    let tag = if big_endian {
                        exif_buffer.read_u16_be(entry_offset)
                    } else {
                        exif_buffer.read_u16_le(entry_offset)
                    };
                    // Orientation Tag
                    if tag == 274 {
                        orientation = if big_endian {
                            exif_buffer.read_u16_be(entry_offset + 8)
                        } else {
                            exif_buffer.read_u16_le(entry_offset + 8)
                        };
                    }
                }
            }
            offset += section_size + 2;
            continue;
        }

        // 0xFFC0 is baseline standard (SOF0)
        // 0xFFC1 is baseline optimized (SOF1)
        // 0xFFC2 is progressive (SOF2)
        if buffer.cmp_any_of(0, 2, vec![b"\xFF\xC0", b"\xFF\xC1", b"\xFF\xC2"]) {
            let mut size = ImageSize {
                width: buffer.read_u16_be(7) as i64,
                height: buffer.read_u16_be(5) as i64,
            };
            if orientation == 5 || orientation == 6 || orientation == 7 || orientation == 8 {
                std::mem::swap(&mut size.width, &mut size.height);
            }
            ret.size = size;
            return Ok(ret);
        }
        offset += section_size + 2;
    }

    Err(ImageInfoError::UnrecognizedFormat)
}
