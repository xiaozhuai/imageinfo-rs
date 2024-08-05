use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

// https://docs.fileformat.com/image/jp2/
// https://docs.fileformat.com/image/jpx/
pub fn try_jpeg2000_code_stream<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 16 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    // SOC and SIZ
    let buffer = ri.read(0, 16)?;
    if buffer.cmp(0, 2, b"\xFF\x4F") && buffer.cmp(2, 2, b"\xFF\x51") {
        let siz_length = buffer.read_u16_be(4);
        if length < siz_length as usize + 4 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        return Ok(ImageInfo {
            format: ImageFormat::J2K,
            ext: "j2k",
            full_ext: "j2k",
            mimetype: "image/j2k",
            size: ImageSize {
                width: buffer.read_u32_be(8) as i64,
                height: buffer.read_u32_be(12) as i64,
            },
            entry_sizes: vec![],
        });
    }

    Err(ImageInfoError::UnrecognizedFormat)
}
