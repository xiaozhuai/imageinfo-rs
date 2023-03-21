use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

// https://www.fileformat.info/format/bmp/corion.htm
pub fn try_bmp<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 26 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 26)?;
    if !buffer.cmp(0, 2, b"BM") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let mut ret = ImageInfo {
        format: ImageFormat::BMP,
        ext: "bmp",
        full_ext: "bmp",
        mimetype: "image/bmp",
        size: ImageSize {
            width: 0,
            height: 0,
        },
        entry_sizes: vec![],
    };

    ret.size.width = buffer.read_i32_le(18) as i64;
    // bmp height can be negative, it means flip Y
    ret.size.height = buffer.read_i32_le(22).abs() as i64;

    Ok(ret)
}
