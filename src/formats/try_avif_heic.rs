use std::io::{BufRead, Seek};
use std::collections::HashSet;
use crate::{ImageInfoResult, ImageFormat, ImageInfo, ImageInfoError, ImageSize, ReadInterface};

// https://nokiatech.github.io/heif/technical.html
// https://www.jianshu.com/p/b016d10a087d
pub fn try_avif_heic<R>(
    ri: &mut ReadInterface<R>,
    length: usize,
) -> ImageInfoResult<ImageInfo>
    where R: BufRead + Seek {
    if length < 4 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 4)?;
    let ftyp_box_length = buffer.read_u32_be(0) as usize;
    if length < ftyp_box_length + 12 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, ftyp_box_length + 12)?;
    if !buffer.cmp(4, 4, b"ftyp") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }


    //
    // Major Brand
    //
    // AVIF: "avif"
    // HEIF: "mif1", "msf1"
    // HEIC: "heic", "heix", "hevc", "hevx"
    //
    if !buffer.cmp_any_of(8, 4, vec![b"avif", b"mif1", b"msf1", b"heic", b"heix", b"hevc", b"hevx"]) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let compatible_brand_size = (ftyp_box_length - 16) / 4;
    let mut compatible_brands = HashSet::new();
    for i in 0..compatible_brand_size {
        compatible_brands.insert(buffer.read_str(16 + i * 4, 4));
    }

    let mut ret =
        // contains "avif"
        if compatible_brands.contains("avif") {
            ImageInfo {
                format: ImageFormat::AVIF,
                ext: "avif",
                full_ext: "avif",
                mimetype: "image/avif",
                size: ImageSize {
                    width: 0,
                    height: 0,
                },
                entry_sizes: vec![],
            }
        }
        // contains "heic"
        else if compatible_brands.contains("heic") {
            ImageInfo {
                format: ImageFormat::HEIC,
                ext: "heic",
                full_ext: "heic",
                mimetype: "image/heic",
                size: ImageSize {
                    width: 0,
                    height: 0,
                },
                entry_sizes: vec![],
            }
        }
        // invalid
        else {
            return Err(ImageInfoError::UnrecognizedFormat);
        };

    if !buffer.cmp(ftyp_box_length + 4, 4, b"meta") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let meta_length = buffer.read_u32_be(ftyp_box_length) as usize;
    if length < ftyp_box_length + 12 + meta_length {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let buffer = ri.read(ftyp_box_length + 12, meta_length)?;
    let mut offset = 0usize;
    let mut end = meta_length;

    //
    // find ispe box
    //
    // meta
    //   - ...
    //   - iprp
    //       - ...
    //       - ipco
    //           - ...
    //           - ispe
    //
    while offset < end {
        let box_size = buffer.read_u32_be(offset) as usize;
        if buffer.cmp_any_of(offset + 4, 4, vec![b"iprp", b"ipco"]) {
            end = offset + box_size;
            offset += 8;
        } else if buffer.cmp(offset + 4, 4, b"ispe") {
            ret.size.width = buffer.read_u32_be(offset + 12) as i64;
            ret.size.height = buffer.read_u32_be(offset + 16) as i64;
            return Ok(ret);
        } else {
            offset += box_size;
        }
    }

    Err(ImageInfoError::UnrecognizedFormat)
}

