use std::io::{BufRead, Seek};
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

// https://docs.fileformat.com/image/jp2/
// https://docs.fileformat.com/image/jpx/
pub fn try_jp2_jpx<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 8 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 8)?;
    if !buffer.cmp(4, 4, b"jP  ") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let signature_length = buffer.read_u32_be(0) as usize;
    let mut offset = signature_length;
    if length < offset + 12 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let buffer = ri.read(offset, 12)?;

    if !buffer.cmp(4, 4, b"ftyp") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let mut ret =
        // type == jp2
        if buffer.cmp(8, 4, b"jp2 ") {
            ImageInfo {
                format: ImageFormat::JP2,
                ext: "jp2",
                full_ext: "jp2",
                mimetype: "image/jp2",
                size: ImageSize {
                    width: 0,
                    height: 0,
                },
                entry_sizes: vec![],
            }
        }
        // type == jpx
        else if buffer.cmp(8, 4, b"jpx ") {
            ImageInfo {
                format: ImageFormat::JPX,
                ext: "jpx",
                full_ext: "jpx",
                mimetype: "image/jpx",
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

    let ftyp_length = buffer.read_u32_be(0) as usize;
    offset += ftyp_length;

    while offset + 24 <= length {
        let buffer = ri.read(offset, 24)?;
        if buffer.cmp(4, 4, b"jp2h") {
            if buffer.cmp(12, 4, b"ihdr") {
                ret.size.width = buffer.read_u32_be(20) as i64;
                ret.size.height = buffer.read_u32_be(16) as i64;
            }
            break;
        }
        let box_length = buffer.read_u32_be(0) as usize;
        offset += box_length;
    }

    Ok(ret)
}

