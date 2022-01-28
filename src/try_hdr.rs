use std::cmp::min;
use std::io::{BufRead, Seek};
use std::str::FromStr;
use regex::Regex;
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

// http://paulbourke.net/dataformats/pic/
pub fn try_hdr<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 6 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    // TODO Max header size ? Or just read header line by line
    let buffer = ri.read(0, min(length, 256))?;
    if !buffer.cmp_any_of(0, 6, vec![b"#?RGBE", b"#?XYZE"]) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let header = buffer.read_str(0, buffer.len());
    let x_pattern = Regex::new(r"\s([-+])X\s(\d+)\s").unwrap();
    let y_pattern = Regex::new(r"\s([-+])Y\s(\d+)\s").unwrap();
    let x_captures = x_pattern.captures(&header);
    let y_captures = y_pattern.captures(&header);
    if x_captures.is_none() || y_captures.is_none() {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let x_captures = x_captures.unwrap();
    let y_captures = y_captures.unwrap();
    if x_captures.len() < 3 || y_captures.len() < 3 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let width = &x_captures[2];
    let height = &y_captures[2];
    Ok(ImageInfo {
        format: ImageFormat::HDR,
        ext: "hdr",
        full_ext: "hdr",
        mimetype: "image/vnd.radiance",
        size: ImageSize {
            width: i64::from_str(width).unwrap(),
            height: i64::from_str(height).unwrap(),
        },
        entry_sizes: vec![],
    })
}

