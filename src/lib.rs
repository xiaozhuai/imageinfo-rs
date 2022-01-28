mod defs;
mod raw_buffer;
mod read_interface;
mod try_avif_heic;
mod try_bmp;
mod try_cur_ico;
mod try_dds;
mod try_gif;
mod try_hdr;
mod try_icns;
mod try_jp2_jpx;
mod try_jpg;
mod try_ktx;
mod try_png;
mod try_psd;
mod try_tiff;
mod try_webp;
mod try_tga;

use std::io::{BufReader, Seek, BufRead, SeekFrom};
use std::fs::File;
pub use defs::ImageSize;
pub use defs::ImageInfoError;
pub use defs::ImageInfoResult;
use raw_buffer::RawBuffer;
use read_interface::ReadInterface;
use try_avif_heic::try_avif_heic;
use try_bmp::try_bmp;
use try_cur_ico::try_cur_ico;
use try_dds::try_dds;
use try_gif::try_gif;
use try_hdr::try_hdr;
use try_icns::try_icns;
use try_jp2_jpx::try_jp2_jpx;
use try_jpg::try_jpg;
use try_ktx::try_ktx;
use try_png::try_png;
use try_psd::try_psd;
use try_tiff::try_tiff;
use try_webp::try_webp;
use try_tga::try_tga;

#[derive(PartialEq)]
#[derive(Debug)]
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
    JP2,
    JPX,
    JPEG,
    KTX,
    PNG,
    PSD,
    TIFF,
    WEBP,
    TGA,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct ImageInfo {
    pub format: ImageFormat,
    pub ext: &'static str,
    pub full_ext: &'static str,
    pub mimetype: &'static str,
    pub size: ImageSize,
    pub entry_sizes: Vec<ImageSize>,
}

impl ImageInfo {
    pub fn from_reader<R>(reader: &mut R) -> ImageInfoResult<ImageInfo>
        where R: BufRead + Seek {
        let length = reader.seek(SeekFrom::End(0))? as usize;
        let mut ri = ReadInterface { reader };

        if let Result::Ok(image_info) = try_avif_heic(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_bmp(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_cur_ico(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_dds(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_gif(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_hdr(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_icns(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_jp2_jpx(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_jpg(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_ktx(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_png(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_psd(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_tiff(&mut ri, length) {
            return Ok(image_info);
        }

        if let Result::Ok(image_info) = try_webp(&mut ri, length) {
            return Ok(image_info);
        }

        // !!! keep tga last !!!
        if let Result::Ok(image_info) = try_tga(&mut ri, length) {
            return Ok(image_info);
        }

        Err(ImageInfoError::UnrecognizedFormat)
    }

    pub fn from_file(file: &mut File) -> ImageInfoResult<ImageInfo> {
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }

    pub fn from_file_path(filepath: &str) -> ImageInfoResult<ImageInfo> {
        let mut file = File::open(filepath)?;
        Self::from_file(&mut file)
    }
}

