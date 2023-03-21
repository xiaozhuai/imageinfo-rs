use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

// TODO Not rigorous enough, keep it as last detector
// https://www.fileformat.info/format/tga/corion.htm
pub fn try_tga<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 18 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(length - 18, 18)?;
    if buffer.cmp(0, 18, b"TRUEVISION-XFILE.\x00") {
        if length < 18 + 16 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        let buffer = ri.read(0, 18)?;
        return Ok(ImageInfo {
            format: ImageFormat::TGA,
            ext: "tga",
            full_ext: "tga",
            mimetype: "image/tga",
            size: ImageSize {
                width: buffer.read_u16_le(12) as i64,
                height: buffer.read_u16_le(14) as i64,
            },
            entry_sizes: vec![],
        });
    }

    let buffer = ri.read(0, 18)?;
    let id_len = buffer.read_u8(0) as usize;
    if length < id_len + 18 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let color_map_type = buffer.read_u8(1);
    let image_type = buffer.read_u8(2);
    let first_color_map_entry_index = buffer.read_u16_le(3);
    let color_map_length = buffer.read_u16_le(5);
    let color_map_entry_size = buffer.read_u8(7);
    let width = buffer.read_u16_le(12) as i64;
    let height = buffer.read_u16_le(14) as i64;

    if color_map_type == 0 {
        if image_type == 0
            || image_type == 2
            || image_type == 3
            || image_type == 10
            || image_type == 11
            || image_type == 32
            || image_type == 33
        {
            if first_color_map_entry_index == 0
                && color_map_length == 0
                && color_map_entry_size == 0
            {
                return Ok(ImageInfo {
                    format: ImageFormat::TGA,
                    ext: "tga",
                    full_ext: "tga",
                    mimetype: "image/tga",
                    size: ImageSize { width, height },
                    entry_sizes: vec![],
                });
            }
        }
    } else if color_map_type == 1 {
        if image_type == 1 || image_type == 9 {
            return Ok(ImageInfo {
                format: ImageFormat::TGA,
                ext: "tga",
                full_ext: "tga",
                mimetype: "image/tga",
                size: ImageSize { width, height },
                entry_sizes: vec![],
            });
        }
    }

    return Err(ImageInfoError::UnrecognizedFormat);
}
