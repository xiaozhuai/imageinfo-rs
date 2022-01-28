use std::io::{BufRead, Seek};
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

pub fn try_ktx<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 44 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 44)?;
    if !buffer.cmp(1, 6, b"KTX 11") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    Ok(ImageInfo {
        format: ImageFormat::KTX,
        ext: "ktx",
        full_ext: "ktx",
        mimetype: "image/ktx",
        size: ImageSize {
            width: buffer.read_u32_le(36) as i64,
            height: buffer.read_u32_le(40) as i64,
        },
        entry_sizes: vec![],
    })
}

