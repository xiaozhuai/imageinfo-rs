use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use regex::Regex;
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
    let buffer = ri.read(0, min(length, 6))?;
    let buffer2 = ri.read(0, min(length, 10))?;
    if !buffer.cmp_any_of(0, 6, vec![b"#?RGBE", b"#?XYZE"]) && !buffer2.cmp(0, 10, b"#?RADIANCE") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let mut offset = 6usize;
    let piece = 64usize;
    let mut header = String::new();
    let x_pattern = Regex::new(r"\s[-+]X\s(\d+)\s").unwrap();
    let y_pattern = Regex::new(r"\s[-+]Y\s(\d+)\s").unwrap();
    while offset < length {
        let buffer = ri.read(offset, min(length - offset, piece))?;
        offset += buffer.len();
        header += &(buffer.to_string());
        let x_captures = x_pattern.captures(&header);
        let y_captures = y_pattern.captures(&header);
        if let (Some(x_captures), Some(y_captures)) = (x_captures, y_captures) {
            if x_captures.len() < 2 || y_captures.len() < 2 {
                return Err(ImageInfoError::UnrecognizedFormat);
            }
            let width = &x_captures[1];
            let height = &y_captures[1];
            return Ok(ImageInfo {
                format: ImageFormat::HDR,
                ext: "hdr",
                full_ext: "hdr",
                mimetype: "image/vnd.radiance",
                size: ImageSize {
                    width: i64::from_str(width).unwrap(),
                    height: i64::from_str(height).unwrap(),
                },
                entry_sizes: vec![],
            });
        }
    }
    Err(ImageInfoError::UnrecognizedFormat)
}
