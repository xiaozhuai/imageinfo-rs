use crate::{ImageFormat, ImageInfo, ImageInfoError, ImageInfoResult, ImageSize, ReadInterface};
use std::io::{BufRead, Seek};

// https://www.fileformat.info/format/tiff/corion.htm
pub fn try_tiff<R>(ri: &mut ReadInterface<R>, length: usize) -> ImageInfoResult<ImageInfo>
where
    R: BufRead + Seek,
{
    if length < 8 {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let buffer = ri.read(0, 8)?;
    if !buffer.cmp_any_of(0, 2, vec![b"\x49\x49", b"\x4D\x4D"]) {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
    let little_endian = buffer.data[0] == 0x49;
    let tiff_version = if little_endian {
        buffer.read_u16_le(2)
    } else {
        buffer.read_u16_be(2)
    };

    if tiff_version == 0x2A {
        let mut offset = if little_endian {
            buffer.read_u32_le(4) as usize
        } else {
            buffer.read_u32_be(4) as usize
        };
        if length < offset + 2 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }

        let buffer = ri.read(offset, 2)?;

        let mum_entry = if little_endian {
            buffer.read_i16_le(0) as usize
        } else {
            buffer.read_i16_be(0) as usize
        };
        offset += 2;

        let mut ret = ImageInfo {
            format: ImageFormat::TIFF,
            ext: "tif",
            full_ext: "tiff",
            mimetype: "image/tiff",
            size: ImageSize {
                width: -1,
                height: -1,
            },
            entry_sizes: vec![],
        };

        let mut i = 0usize;
        while i < mum_entry && (ret.size.width == -1 || ret.size.height == -1) {
            if length < offset + 12 {
                break;
            }
            let buffer = ri.read(offset, 12)?;

            let tag = if little_endian {
                buffer.read_u16_le(0)
            } else {
                buffer.read_u16_be(0)
            };

            let t = if little_endian {
                buffer.read_u16_le(2)
            } else {
                buffer.read_u16_be(2)
            };

            if tag == 256 {
                if t == 3 {
                    ret.size.width = if little_endian {
                        buffer.read_u16_le(8) as i64
                    } else {
                        buffer.read_u16_be(8) as i64
                    };
                } else if t == 4 {
                    ret.size.width = if little_endian {
                        buffer.read_u32_le(8) as i64
                    } else {
                        buffer.read_u32_be(8) as i64
                    };
                }
            } else if tag == 257 {
                if t == 3 {
                    ret.size.height = if little_endian {
                        buffer.read_u16_le(8) as i64
                    } else {
                        buffer.read_u16_be(8) as i64
                    };
                } else if t == 4 {
                    ret.size.height = if little_endian {
                        buffer.read_u32_le(8) as i64
                    } else {
                        buffer.read_u32_be(8) as i64
                    };
                }
            }

            i += 1;
            offset += 12;
        }

        if ret.size.width != -1 && ret.size.height != -1 {
            Ok(ret)
        } else {
            Err(ImageInfoError::UnrecognizedFormat)
        }
    } else if tiff_version == 0x2B {
        if length < 16 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        let byte_size = if little_endian {
            buffer.read_u16_le(4)
        } else {
            buffer.read_u16_be(4)
        };
        let zero = if little_endian {
            buffer.read_u16_le(6)
        } else {
            buffer.read_u16_be(6)
        };
        if byte_size != 8 || zero != 0 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        let buffer = ri.read(8, 8)?;
        let mut offset = if little_endian {
            buffer.read_u64_le(0)
        } else {
            buffer.read_u64_be(0)
        };
        if (length as u64) < offset + 8 || offset > (usize::MAX as u64) - 8 {
            return Err(ImageInfoError::UnrecognizedFormat);
        }
        let buffer = ri.read(offset as usize, 8)?;
        let mum_entry = if little_endian {
            buffer.read_u64_le(0)
        } else {
            buffer.read_u64_be(0)
        };
        offset += 8;

        let mut ret = ImageInfo {
            format: ImageFormat::TIFF,
            ext: "tif",
            full_ext: "tiff",
            mimetype: "image/tiff",
            size: ImageSize {
                width: -1,
                height: -1,
            },
            entry_sizes: vec![],
        };

        let mut i = 0u64;
        while i < mum_entry && (ret.size.width == -1 || ret.size.height == -1) {
            if (length as u64) < offset + 20 || offset > (usize::MAX as u64) - 20 {
                break;
            }
            let buffer = ri.read(offset as usize, 20)?;

            let tag = if little_endian {
                buffer.read_u16_le(0)
            } else {
                buffer.read_u16_be(0)
            };

            let t = if little_endian {
                buffer.read_u16_le(2)
            } else {
                buffer.read_u16_be(2)
            };

            if tag == 256 {
                if t == 3 {
                    ret.size.width = if little_endian {
                        buffer.read_u16_le(12) as i64
                    } else {
                        buffer.read_u16_be(12) as i64
                    };
                } else if t == 4 {
                    ret.size.width = if little_endian {
                        buffer.read_u32_le(12) as i64
                    } else {
                        buffer.read_u32_be(12) as i64
                    };
                } else if t == 16 {
                    let w = if little_endian {
                        buffer.read_u64_le(12)
                    } else {
                        buffer.read_u64_be(12)
                    };
                    if w > (i64::MAX as u64) {
                        // TODO: Size > INT64_MAX is not supported
                        return Err(ImageInfoError::UnrecognizedFormat);
                    }
                    ret.size.width = w as i64;
                }
            } else if tag == 257 {
                if t == 3 {
                    ret.size.height = if little_endian {
                        buffer.read_u16_le(12) as i64
                    } else {
                        buffer.read_u16_be(12) as i64
                    };
                } else if t == 4 {
                    ret.size.height = if little_endian {
                        buffer.read_u32_le(12) as i64
                    } else {
                        buffer.read_u32_be(12) as i64
                    };
                } else if t == 16 {
                    let h = if little_endian {
                        buffer.read_u64_le(12)
                    } else {
                        buffer.read_u64_be(12)
                    };
                    if h > (i64::MAX as u64) {
                        // TODO: Size > INT64_MAX is not supported
                        return Err(ImageInfoError::UnrecognizedFormat);
                    }
                    ret.size.height = h as i64;
                }
            }

            i += 1;
            offset += 20;
        }

        if ret.size.width != -1 && ret.size.height != -1 {
            Ok(ret)
        } else {
            Err(ImageInfoError::UnrecognizedFormat)
        }
    } else {
        return Err(ImageInfoError::UnrecognizedFormat);
    }
}
