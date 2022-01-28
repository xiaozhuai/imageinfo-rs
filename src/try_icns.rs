use std::cmp::max;
use std::io::{BufRead, Seek};
use std::collections::HashMap;
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

pub fn try_icns<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 8 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 8)?;
    let file_length = buffer.read_u32_be(4) as usize;
    if !buffer.cmp(0, 4, b"icns") || length < file_length {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let type_size_map: HashMap<&str, i64> = [
        ("ICON", 32),
        ("ICN#", 32),
        ("icm#", 16),
        ("icm4", 16),
        ("icm8", 16),
        ("ics#", 16),
        ("ics4", 16),
        ("ics8", 16),
        ("is32", 16),
        ("s8mk", 16),
        ("icl4", 32),
        ("icl8", 32),
        ("il32", 32),
        ("l8mk", 32),
        ("ich#", 48),
        ("ich4", 48),
        ("ich8", 48),
        ("ih32", 48),
        ("h8mk", 48),
        ("it32", 128),
        ("t8mk", 128),
        ("icp4", 16),
        ("icp5", 32),
        ("icp6", 64),
        ("ic07", 128),
        ("ic08", 256),
        ("ic09", 512),
        ("ic10", 1024),
        ("ic11", 32),
        ("ic12", 64),
        ("ic13", 256),
        ("ic14", 512),
        ("ic04", 16),
        ("ic05", 32),
        ("icsB", 36),
        ("icsb", 18),
    ].iter().cloned().collect();

    let mut ret = ImageInfo {
        format: ImageFormat::ICNS,
        ext: "icns",
        full_ext: "icns",
        mimetype: "image/icns",
        size: ImageSize {
            width: 0,
            height: 0,
        },
        entry_sizes: vec![],
    };

    let mut max_size = 0i64;
    let mut offset = 8usize;
    while offset + 8 <= length {
        let buffer = ri.read(offset, 8)?;
        let t = buffer.read_str(0, 4);
        let entry_size = buffer.read_u32_be(4) as usize;
        let s = *type_size_map.get(t.as_str()).unwrap();
        ret.entry_sizes.push(ImageSize { width: s, height: s });
        max_size = max(max_size, s);
        offset += entry_size;
    }

    ret.size.width = max_size;
    ret.size.height = max_size;

    Ok(ret)
}

