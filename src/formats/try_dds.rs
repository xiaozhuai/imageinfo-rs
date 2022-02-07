use std::io::{BufRead, Seek};
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

pub fn try_dds<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 20 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 20)?;
    if !buffer.cmp(0, 4, b"DDS ") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    Ok(ImageInfo {
        format: ImageFormat::DDS,
        ext: "dds",
        full_ext: "dds",
        mimetype: "image/dds",
        size: ImageSize {
            width: buffer.read_u32_le(16) as i64,
            height: buffer.read_u32_le(12) as i64,
        },
        entry_sizes: vec![],
    })
}

