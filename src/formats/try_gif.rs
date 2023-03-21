use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

// https://www.fileformat.info/format/gif/corion.htm
pub fn try_gif<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 10 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 10)?;
    if !buffer.cmp_any_of(0, 6, vec![b"GIF87a", b"GIF89a"]) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let mut ret = ImageInfo {
        format: ImageFormat::GIF,
        ext: "gif",
        full_ext: "gif",
        mimetype: "image/gif",
        size: ImageSize {
            width: 0,
            height: 0,
        },
        entry_sizes: vec![],
    };

    ret.size.width = buffer.read_u16_le(6) as i64;
    ret.size.height = buffer.read_u16_le(8) as i64;

    Ok(ret)
}
