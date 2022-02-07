use std::cmp::min;
use std::io::{BufRead, Seek};
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

// https://www.fileformat.info/format/png/corion.htm
pub fn try_png<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 4 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, min(length, 40))?;
    if !buffer.cmp(0, 4, b"\x89PNG") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let ret =
        if buffer.cmp(12, 4, b"IHDR") && buffer.len() >= 24 {
            ImageInfo {
                format: ImageFormat::PNG,
                ext: "png",
                full_ext: "png",
                mimetype: "image/png",
                size: ImageSize {
                    width: buffer.read_u32_be(16) as i64,
                    height: buffer.read_u32_be(20) as i64,
                },
                entry_sizes: vec![],
            }
        } else if buffer.cmp(12, 4, b"CgBI") {
            ImageInfo {
                format: ImageFormat::PNG,
                ext: "png",
                full_ext: "png",
                mimetype: "image/png",
                size: ImageSize {
                    width: buffer.read_u32_be(32) as i64,
                    height: buffer.read_u32_be(36) as i64,
                },
                entry_sizes: vec![],
            }
        } else {
            return Err(ImageInfoError::UnrecognizedFormat);
        };

    Ok(ret)
}

