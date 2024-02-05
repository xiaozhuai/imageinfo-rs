use crate::ImageFormat::{PAM, PBM, PFM, PGM, PPM};
use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::cmp::min;
use std::collections::HashMap;
use std::io::{BufRead, Seek};
use std::str::FromStr;

pub fn try_pnm<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 3 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let pnm_types: HashMap<&str, (ImageFormat, &str, &str)> = [
        ("P1", (PBM, "pbm", "image/x-portable-bitmap")),
        ("P2", (PGM, "pgm", "image/x-portable-graymap")),
        ("P3", (PPM, "ppm", "image/x-portable-pixmap")),
        ("P4", (PBM, "pbm", "image/x-portable-bitmap")),
        ("P5", (PGM, "pgm", "image/x-portable-graymap")),
        ("P6", (PPM, "ppm", "image/x-portable-pixmap")),
        ("P7", (PAM, "pam", "image/x-portable-arbitrarymap")),
        ("PF", (PFM, "pfm", "image/x-portable-floatmap")),
    ]
    .iter()
    .cloned()
    .collect();
    let buffer = ri.read(0, 3)?;
    if !buffer.cmp(2, 1, b"\n") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let t = if let Some(t) = pnm_types.get(buffer.read_str(0, 2).as_str()) {
        t
    } else {
        return Err(ImageInfoError::UnrecognizedFormat);
    };
    let format = t.0.clone();
    let ext = t.1;
    let mime = t.2;
    let is_pam = format == PAM;
    let needed_lines = if is_pam { 2 } else { 1 };

    let mut offset = 3usize;
    let piece = 64usize;
    let mut lines: Vec<&str> = vec![];
    let mut header = String::new();

    while offset < length && lines.len() < needed_lines {
        let buffer = ri.read(offset, min(length - offset, piece))?;
        offset += buffer.len();
        header += &(buffer.to_string());
        lines = header
            .split('\n')
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
            .take(needed_lines)
            .collect();
    }

    if is_pam {
        let mut width: i64 = -1;
        let mut height: i64 = -1;
        for line in lines {
            if line.starts_with("WIDTH ") {
                if let Ok(w) = i64::from_str(&line[6..]) {
                    width = w;
                } else {
                    return Err(ImageInfoError::UnrecognizedFormat);
                }
            } else if line.starts_with("HEIGHT ") {
                if let Ok(h) = i64::from_str(&line[7..]) {
                    height = h;
                } else {
                    return Err(ImageInfoError::UnrecognizedFormat);
                }
            }
        }
        if width < 0 || height < 0 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        return Ok(ImageInfo {
            format,
            ext,
            full_ext: ext,
            mimetype: mime,
            size: ImageSize { width, height },
            entry_sizes: vec![],
        });
    } else {
        let width: i64;
        let height: i64;
        let line = lines[0];
        let tokens: Vec<&str> = line.split(' ').collect();
        if tokens.len() < 2 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        if let Ok(w) = i64::from_str(tokens[0]) {
            width = w;
        } else {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        if let Ok(h) = i64::from_str(tokens[1]) {
            height = h;
        } else {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        return Ok(ImageInfo {
            format,
            ext,
            full_ext: ext,
            mimetype: mime,
            size: ImageSize { width, height },
            entry_sizes: vec![],
        });
    }
}
