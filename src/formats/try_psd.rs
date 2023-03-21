use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

pub fn try_psd<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 22 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 22)?;
    if !buffer.cmp(0, 6, b"8BPS\x00\x01") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    Ok(ImageInfo {
        format: ImageFormat::PSD,
        ext: "psd",
        full_ext: "psd",
        mimetype: "image/psd",
        size: ImageSize {
            width: buffer.read_u32_be(18) as i64,
            height: buffer.read_u32_be(14) as i64,
        },
        entry_sizes: vec![],
    })
}
