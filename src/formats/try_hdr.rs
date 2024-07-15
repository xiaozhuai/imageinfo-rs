use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::cmp::min;
use std::io::{BufRead, Seek};
use std::str::FromStr;

// http://paulbourke.net/dataformats/pic/
pub fn try_hdr<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 6 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 6)?;
    if !buffer.cmp_any_of(0, 6, vec![b"#?RGBE", b"#?XYZE"]) {
        if length < 10 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        let buffer = ri.read(0, 10)?;
        if !buffer.cmp(0, 10, b"#?RADIANCE") {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
    }

    let piece = 64usize;
    let mut header = String::new();
    let mut resolution_start = 0usize;
    let mut resolution = "";
    let mut offset = 0usize;
    while offset < length {
        let buffer = ri.read(offset, min(length - offset, piece))?;
        offset += buffer.len();
        header += &(buffer.read_str_all());
        if resolution_start == 0 {
            if let Some(pos) = header.find("\n\n") {
                resolution_start = pos + 2;
            } else {
                continue;
            }
        }
        if let Some(pos) = &header[resolution_start..].find('\n') {
            resolution = &header[resolution_start..resolution_start + pos];
            break;
        }
    }
    if resolution.is_empty() {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let tokens: Vec<&str> = resolution.split(' ').collect();
    if tokens.len() != 4 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let y_str = tokens[1];
    let x_str = tokens[3];

    if let Ok(width) = i64::from_str(x_str) {
        if let Ok(height) = i64::from_str(y_str) {
            return Ok(ImageInfo {
                format: ImageFormat::HDR,
                ext: "hdr",
                full_ext: "hdr",
                mimetype: "image/vnd.radiance",
                size: ImageSize { width, height },
                entry_sizes: vec![],
            });
        }
    }
    Err(ImageInfoError::UnrecognizedFormat)
}
