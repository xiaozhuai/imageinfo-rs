mod defs;
mod formats;
mod raw_buffer;
mod read_interface;

pub use defs::ImageInfoError;
pub use defs::ImageInfoResult;
pub use defs::ImageSize;
use formats::try_avif_heic;
use formats::try_bmp;
use formats::try_cur_ico;
use formats::try_dds;
use formats::try_gif;
use formats::try_hdr;
use formats::try_icns;
use formats::try_j2k;
use formats::try_jp2_jpx;
use formats::try_jpg;
use formats::try_ktx;
use formats::try_png;
use formats::try_psd;
use formats::try_qoi;
use formats::try_tga;
use formats::try_tiff;
use formats::try_webp;
use raw_buffer::RawBuffer;
use read_interface::ReadInterface;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom};
use std::path::Path;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum ImageFormat {
    AVIF,
    HEIC,
    BMP,
    CUR,
    ICO,
    DDS,
    GIF,
    HDR,
    ICNS,
    J2K,
    JP2,
    JPX,
    JPEG,
    KTX,
    PNG,
    PSD,
    QOI,
    TIFF,
    WEBP,
    TGA,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ImageInfo {
    pub format: ImageFormat,
    pub ext: &'static str,
    pub full_ext: &'static str,
    pub mimetype: &'static str,
    pub size: ImageSize,
    pub entry_sizes: Vec<ImageSize>,
}

type Detector<R> = fn(&mut ReadInterface<R>, usize) -> ImageInfoResult<ImageInfo>;

impl ImageInfo {
    pub fn from_reader<R>(reader: &mut R) -> ImageInfoResult<ImageInfo>
    where
        R: BufRead + Seek,
    {
        let length = reader.seek(SeekFrom::End(0))? as usize;
        let mut ri = ReadInterface::from_reader(reader, length);

        let dl: [(ImageFormat, Detector<_>); 20] = [
            (ImageFormat::AVIF, try_avif_heic),
            (ImageFormat::HEIC, try_avif_heic),
            (ImageFormat::BMP, try_bmp),
            (ImageFormat::CUR, try_cur_ico),
            (ImageFormat::ICO, try_cur_ico),
            (ImageFormat::DDS, try_dds),
            (ImageFormat::GIF, try_gif),
            (ImageFormat::HDR, try_hdr),
            (ImageFormat::ICNS, try_icns),
            (ImageFormat::J2K, try_j2k),
            (ImageFormat::JP2, try_jp2_jpx),
            (ImageFormat::JPX, try_jp2_jpx),
            (ImageFormat::JPEG, try_jpg),
            (ImageFormat::KTX, try_ktx),
            (ImageFormat::PNG, try_png),
            (ImageFormat::PSD, try_psd),
            (ImageFormat::QOI, try_qoi),
            (ImageFormat::TIFF, try_tiff),
            (ImageFormat::WEBP, try_webp),
            // !!! keep tga last !!!
            (ImageFormat::TGA, try_tga),
        ];

        let mut tried: HashSet<&Detector<_>> = HashSet::new();

        // let dm: HashMap<ImageFormat, Detector<_>> = dl.iter().cloned().collect();

        for d in dl.iter() {
            // let format = &d.0;
            let detector = &d.1;
            if tried.contains(detector) {
                continue;
            }
            tried.insert(detector);
            if let Ok(image_info) = detector(&mut ri, length) {
                return Ok(image_info);
            }
        }

        Err(ImageInfoError::UnrecognizedFormat)
    }

    pub fn from_file(file: &File) -> ImageInfoResult<ImageInfo> {
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }

    pub fn from_file_path(filepath: impl AsRef<Path>) -> ImageInfoResult<ImageInfo> {
        let file = File::open(filepath)?;
        Self::from_file(&file)
    }

    pub fn from_raw_data(data: &[u8]) -> ImageInfoResult<ImageInfo> {
        let mut reader = BufReader::new(Cursor::new(data));
        Self::from_reader(&mut reader)
    }
}
