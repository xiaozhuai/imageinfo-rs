use std::io::{BufRead, Seek};
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

pub fn try_cur_ico<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 6 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 6)?;

    let mut ret =
        // ico type == 1
        if buffer.cmp(0, 4, &[0x00, 0x00, 0x01, 0x00]) {
            ImageInfo {
                format: ImageFormat::ICO,
                ext: "ico",
                full_ext: "ico",
                mimetype: "image/ico",
                size: ImageSize {
                    width: 0,
                    height: 0,
                },
                entry_sizes: vec![],
            }
        }
        // cur type == 2
        else if buffer.cmp(0, 4, &[0x00, 0x00, 0x02, 0x00]) {
            ImageInfo {
                format: ImageFormat::CUR,
                ext: "cur",
                full_ext: "cur",
                mimetype: "image/cur",
                size: ImageSize {
                    width: 0,
                    height: 0,
                },
                entry_sizes: vec![],
            }
        }
        // invalid
        else {
            return Err(ImageInfoError::UnrecognizedFormat);
        };


    let entry_count = buffer.read_u16_le(4) as usize;
    if entry_count == 0 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let entry_size = 16;
    let entry_total_size = entry_count * entry_size;

    let mut offset = 6usize;
    if length < offset + entry_total_size {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let buffer = ri.read(offset, entry_total_size)?;
    offset += entry_total_size;

    for i in 0..entry_count {
        let width = buffer.read_u8(i * entry_size);
        let height = buffer.read_u8(i * entry_size + 1);
        let width = if width == 0 {
            256i64
        } else {
            width as i64
        };
        let height = if height == 0 {
            256i64
        } else {
            height as i64
        };
        ret.entry_sizes.push(ImageSize { width, height });

        let bytes = buffer.read_i32_le(i * entry_size + 8) as usize;
        offset += bytes;
    }

    ret.size.width = ret.entry_sizes.first().unwrap().width;
    ret.size.height = ret.entry_sizes.first().unwrap().height;

    Ok(ret)
}

