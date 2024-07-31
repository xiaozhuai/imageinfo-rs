use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Seek};

// https://nokiatech.github.io/heif/technical.html
// https://www.jianshu.com/p/b016d10a087d
// https://github.com/ksvc/MediaParser
pub fn try_avif_heic<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 4 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 4)?;
    let ftyp_box_length = buffer.read_u32_be(0) as usize;
    if (length as u64) < (ftyp_box_length as u64) + 12u64 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, ftyp_box_length + 12)?;
    if !buffer.cmp(4, 4, b"ftyp") {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    //
    // Major Brand
    //
    // AVIF: "avif", "avis"
    // HEIF: "mif1", "msf1"
    // HEIC: "heic", "heix", "hevc", "hevx"
    //
    if !buffer.cmp_any_of(
        8,
        4,
        vec![
            b"avif", b"avis", b"mif1", b"msf1", b"heic", b"heix", b"hevc", b"hevx",
        ],
    ) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    if ftyp_box_length < 16 || (ftyp_box_length - 16) % 4 != 0 {
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
        // Fall back to the major brand
        else if buffer.cmp(8, 4, b"avif") {
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
        // Fall back to the major brand
        else if buffer.cmp(8, 4, b"heic") {
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
    if (length as u64) < (ftyp_box_length as u64) + 12u64 + (meta_length as u64) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }

    let buffer = ri.read(ftyp_box_length + 12, meta_length)?;
    let mut offset = 0usize;
    let end = meta_length;

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
    let mut pitm_id = 1;
    let mut ipma_map: HashMap<u16, HashSet<u8>> = HashMap::new();
    let mut ipco_start = 0usize;
    let mut ipco_end = 0usize;
    let mut ipco_child_index = 1;
    let mut ispe_map: HashMap<u16, ImageSize> = HashMap::new();
    let mut irot_map: HashMap<u16, u8> = HashMap::new();
    while offset < end {
        if offset + 8 > end {
            break;
        }
        let box_size = buffer.read_u32_be(offset) as usize;
        if box_size < 8 || (offset as u64) + (box_size as u64) > (end as u64) {
            break;
        }

        if buffer.cmp(offset + 4, 4, b"pitm") {
            if box_size < 14 {
                return Err(ImageInfoError::UnrecognizedFormat);
            }
            pitm_id = buffer.read_u16_be(offset + 12);
            offset += box_size;
        } else if buffer.cmp(offset + 4, 4, b"ipma") {
            if box_size < 16 {
                return Err(ImageInfoError::UnrecognizedFormat);
            }
            let entry_count = buffer.read_u16_be(offset + 14);
            let mut t = offset + 16;
            for _ in 0..entry_count {
                if box_size < 18 {
                    return Err(ImageInfoError::UnrecognizedFormat);
                }
                let item_id = buffer.read_u16_be(t);
                t += 2;
                if box_size < 19 {
                    return Err(ImageInfoError::UnrecognizedFormat);
                }
                let index_count = buffer.read_u8(t);
                t += 1;
                if box_size < 19 + (index_count as usize) {
                    return Err(ImageInfoError::UnrecognizedFormat);
                }
                let mut indices = HashSet::new();
                for _ in 0..index_count {
                    indices.insert(buffer.read_u8(t) & 0x0F);
                    t += 1;
                }
                ipma_map.insert(item_id, indices);
            }
            offset += box_size;
        } else if buffer.cmp(offset + 4, 4, b"iprp") {
            offset += 8;
        } else if buffer.cmp(offset + 4, 4, b"ipco") {
            ipco_start = offset;
            ipco_end = offset + box_size;
            offset += 8;
        } else if buffer.cmp(offset + 4, 4, b"ispe") {
            if box_size < 20 {
                return Err(ImageInfoError::UnrecognizedFormat);
            }
            let size = ImageSize {
                width: buffer.read_u32_be(offset + 12) as i64,
                height: buffer.read_u32_be(offset + 16) as i64,
            };
            ispe_map.insert(ipco_child_index, size);
            ipco_child_index += 1;
            offset += box_size;
        } else if buffer.cmp(offset + 4, 4, b"irot") {
            if box_size < 9 {
                return Err(ImageInfoError::UnrecognizedFormat);
            }
            let irot = buffer.read_u8(offset + 8);
            irot_map.insert(ipco_child_index, irot);
            ipco_child_index += 1;
            offset += box_size;
        } else {
            if offset > ipco_start && offset < ipco_end {
                ipco_child_index += 1;
            }
            offset += box_size;
        }
    }

    if let Some(indices) = ipma_map.get(&pitm_id) {
        let mut irot = 0u8;
        for it in irot_map {
            if indices.contains(&(it.0 as u8)) {
                irot = it.1;
                break;
            }
        }
        for it in ispe_map {
            if indices.contains(&(it.0 as u8)) {
                let mut size = it.1;
                if irot == 1 || irot == 3 || irot == 6 || irot == 7 {
                    std::mem::swap(&mut size.width, &mut size.height);
                }
                ret.size = size;
                return Ok(ret);
            }
        }
    }

    Err(ImageInfoError::UnrecognizedFormat)
}
