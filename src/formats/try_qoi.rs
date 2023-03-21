use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

pub fn try_qoi<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 12 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 12)?;
    if !buffer.cmp(0, 4, b"qoif") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    Ok(ImageInfo {
        format: ImageFormat::QOI,
        ext: "qoi",
        full_ext: "qoi",
        mimetype: "image/qoi",
        size: ImageSize {
            width: buffer.read_u32_be(4) as i64,
            height: buffer.read_u32_be(8) as i64,
        },
        entry_sizes: vec![],
    })
}
