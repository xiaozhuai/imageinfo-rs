use imageinfo::{ImageFormat, ImageInfo, ImageInfoError, ImageSize};

macro_rules! assert_eq_ok {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert_eq!(left_val.is_ok(), true);
                if let Ok(info) = left_val {
                    assert_eq!(info, right_val)
                }
            }
        }
    }};
}

macro_rules! assert_eq_io_err {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, $right) {
            (left_val, right_val) => match left_val {
                Err(err) => match err {
                    ImageInfoError::IoError(io_err) => {
                        assert_eq!(io_err.kind(), right_val)
                    }
                    _ => {
                        panic!()
                    }
                },
                _ => {
                    panic!()
                }
            },
        }
    }};
}

macro_rules! assert_unrecognized_err {
    ($left:expr) => {{
        match (&$left) {
            left_val => match left_val {
                Err(err) => match err {
                    ImageInfoError::UnrecognizedFormat => {}
                    _ => {
                        panic!()
                    }
                },
                _ => {
                    panic!()
                }
            },
        }
    }};
}

#[test]
fn test_avif() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/avif/sample.avif"),
        ImageInfo {
            format: ImageFormat::AVIF,
            ext: "avif",
            full_ext: "avif",
            mimetype: "image/avif",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/avif/sample2.avif"),
        ImageInfo {
            format: ImageFormat::AVIF,
            ext: "avif",
            full_ext: "avif",
            mimetype: "image/avif",
            size: ImageSize {
                width: 800,
                height: 533
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_heic() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/heic/sample.heic"),
        ImageInfo {
            format: ImageFormat::HEIC,
            ext: "heic",
            full_ext: "heic",
            mimetype: "image/heic",
            size: ImageSize {
                width: 122,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/heic/sample2.heic"),
        ImageInfo {
            format: ImageFormat::HEIC,
            ext: "heic",
            full_ext: "heic",
            mimetype: "image/heic",
            size: ImageSize {
                width: 1440,
                height: 960
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/heic/sample3.heic"),
        ImageInfo {
            format: ImageFormat::HEIC,
            ext: "heic",
            full_ext: "heic",
            mimetype: "image/heic",
            size: ImageSize {
                width: 1280,
                height: 854
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_bmp() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/bmp/sample.bmp"),
        ImageInfo {
            format: ImageFormat::BMP,
            ext: "bmp",
            full_ext: "bmp",
            mimetype: "image/bmp",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/bmp/sample2.bmp"),
        ImageInfo {
            format: ImageFormat::BMP,
            ext: "bmp",
            full_ext: "bmp",
            mimetype: "image/bmp",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_cur() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/cur/sample.cur"),
        ImageInfo {
            format: ImageFormat::CUR,
            ext: "cur",
            full_ext: "cur",
            mimetype: "image/cur",
            size: ImageSize {
                width: 32,
                height: 32
            },
            entry_sizes: vec![ImageSize {
                width: 32,
                height: 32
            },],
        }
    );
}

#[test]
fn test_ico() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/ico/multi-size.ico"),
        ImageInfo {
            format: ImageFormat::ICO,
            ext: "ico",
            full_ext: "ico",
            mimetype: "image/ico",
            size: ImageSize {
                width: 256,
                height: 256
            },
            entry_sizes: vec![
                ImageSize {
                    width: 256,
                    height: 256
                },
                ImageSize {
                    width: 128,
                    height: 128
                },
                ImageSize {
                    width: 96,
                    height: 96
                },
                ImageSize {
                    width: 72,
                    height: 72
                },
                ImageSize {
                    width: 64,
                    height: 64
                },
                ImageSize {
                    width: 48,
                    height: 48
                },
                ImageSize {
                    width: 32,
                    height: 32
                },
                ImageSize {
                    width: 24,
                    height: 24
                },
                ImageSize {
                    width: 16,
                    height: 16
                },
            ],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/ico/multi-size-compressed.ico"),
        ImageInfo {
            format: ImageFormat::ICO,
            ext: "ico",
            full_ext: "ico",
            mimetype: "image/ico",
            size: ImageSize {
                width: 256,
                height: 256
            },
            entry_sizes: vec![
                ImageSize {
                    width: 256,
                    height: 256
                },
                ImageSize {
                    width: 128,
                    height: 128
                },
                ImageSize {
                    width: 96,
                    height: 96
                },
                ImageSize {
                    width: 72,
                    height: 72
                },
                ImageSize {
                    width: 64,
                    height: 64
                },
                ImageSize {
                    width: 48,
                    height: 48
                },
                ImageSize {
                    width: 32,
                    height: 32
                },
                ImageSize {
                    width: 24,
                    height: 24
                },
                ImageSize {
                    width: 16,
                    height: 16
                },
            ],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/ico/sample.ico"),
        ImageInfo {
            format: ImageFormat::ICO,
            ext: "ico",
            full_ext: "ico",
            mimetype: "image/ico",
            size: ImageSize {
                width: 32,
                height: 32
            },
            entry_sizes: vec![ImageSize {
                width: 32,
                height: 32
            },],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/ico/sample-256.ico"),
        ImageInfo {
            format: ImageFormat::ICO,
            ext: "ico",
            full_ext: "ico",
            mimetype: "image/ico",
            size: ImageSize {
                width: 256,
                height: 256
            },
            entry_sizes: vec![ImageSize {
                width: 256,
                height: 256
            },],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/ico/sample-256-compressed.ico"),
        ImageInfo {
            format: ImageFormat::ICO,
            ext: "ico",
            full_ext: "ico",
            mimetype: "image/ico",
            size: ImageSize {
                width: 256,
                height: 256
            },
            entry_sizes: vec![ImageSize {
                width: 256,
                height: 256
            },],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/ico/sample-compressed.ico"),
        ImageInfo {
            format: ImageFormat::ICO,
            ext: "ico",
            full_ext: "ico",
            mimetype: "image/ico",
            size: ImageSize {
                width: 32,
                height: 32
            },
            entry_sizes: vec![ImageSize {
                width: 32,
                height: 32
            },],
        }
    );
}

#[test]
fn test_dds() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/dds/sample.dds"),
        ImageInfo {
            format: ImageFormat::DDS,
            ext: "dds",
            full_ext: "dds",
            mimetype: "image/dds",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_gif() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/gif/sample.gif"),
        ImageInfo {
            format: ImageFormat::GIF,
            ext: "gif",
            full_ext: "gif",
            mimetype: "image/gif",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_hdr() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/hdr/sample.hdr"),
        ImageInfo {
            format: ImageFormat::HDR,
            ext: "hdr",
            full_ext: "hdr",
            mimetype: "image/vnd.radiance",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_icns() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/icns/sample.icns"),
        ImageInfo {
            format: ImageFormat::ICNS,
            ext: "icns",
            full_ext: "icns",
            mimetype: "image/icns",
            size: ImageSize {
                width: 128,
                height: 128
            },
            entry_sizes: vec![
                ImageSize {
                    width: 16,
                    height: 16
                },
                ImageSize {
                    width: 16,
                    height: 16
                },
                ImageSize {
                    width: 32,
                    height: 32
                },
                ImageSize {
                    width: 32,
                    height: 32
                },
                ImageSize {
                    width: 48,
                    height: 48
                },
                ImageSize {
                    width: 48,
                    height: 48
                },
                ImageSize {
                    width: 128,
                    height: 128
                },
                ImageSize {
                    width: 128,
                    height: 128
                },
            ],
        }
    );
}

#[test]
fn test_jp2() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jp2/sample.jp2"),
        ImageInfo {
            format: ImageFormat::JP2,
            ext: "jp2",
            full_ext: "jp2",
            mimetype: "image/jp2",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jp2/jpx_disguised_as_jp2.jp2"),
        ImageInfo {
            format: ImageFormat::JP2,
            ext: "jp2",
            full_ext: "jp2",
            mimetype: "image/jp2",
            size: ImageSize {
                width: 2717,
                height: 3701
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_jpx() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpx/sample.jpx"),
        ImageInfo {
            format: ImageFormat::JPX,
            ext: "jpx",
            full_ext: "jpx",
            mimetype: "image/jpx",
            size: ImageSize {
                width: 2717,
                height: 3701
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_jpg() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/1x2-flipped-big-endian.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 1,
                height: 2
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/1x2-flipped-little-endian.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 1,
                height: 2
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/large.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 1600,
                height: 1200
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/optimized.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/progressive.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/sample.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/sampleExported.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/jpg/very-large.jpg"),
        ImageInfo {
            format: ImageFormat::JPEG,
            ext: "jpg",
            full_ext: "jpeg",
            mimetype: "image/jpeg",
            size: ImageSize {
                width: 4800,
                height: 3600
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_ktx() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/ktx/sample.ktx"),
        ImageInfo {
            format: ImageFormat::KTX,
            ext: "ktx",
            full_ext: "ktx",
            mimetype: "image/ktx",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_png() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/png/sample.png"),
        ImageInfo {
            format: ImageFormat::PNG,
            ext: "png",
            full_ext: "png",
            mimetype: "image/png",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/png/sample_fried.png"),
        ImageInfo {
            format: ImageFormat::PNG,
            ext: "png",
            full_ext: "png",
            mimetype: "image/png",
            size: ImageSize {
                width: 128,
                height: 68
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/png/sample_apng.png"),
        ImageInfo {
            format: ImageFormat::PNG,
            ext: "png",
            full_ext: "png",
            mimetype: "image/png",
            size: ImageSize {
                width: 480,
                height: 400
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_psd() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/psd/sample.psd"),
        ImageInfo {
            format: ImageFormat::PSD,
            ext: "psd",
            full_ext: "psd",
            mimetype: "image/psd",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_qoi() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/qoi/sample.qoi"),
        ImageInfo {
            format: ImageFormat::QOI,
            ext: "qoi",
            full_ext: "qoi",
            mimetype: "image/qoi",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_tiff() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/tiff/big-endian.tiff"),
        ImageInfo {
            format: ImageFormat::TIFF,
            ext: "tif",
            full_ext: "tiff",
            mimetype: "image/tiff",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/tiff/jpeg.tiff"),
        ImageInfo {
            format: ImageFormat::TIFF,
            ext: "tif",
            full_ext: "tiff",
            mimetype: "image/tiff",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/tiff/little-endian.tiff"),
        ImageInfo {
            format: ImageFormat::TIFF,
            ext: "tif",
            full_ext: "tiff",
            mimetype: "image/tiff",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_webp() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/webp/lossless.webp"),
        ImageInfo {
            format: ImageFormat::WEBP,
            ext: "webp",
            full_ext: "webp",
            mimetype: "image/webp",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/webp/extended.webp"),
        ImageInfo {
            format: ImageFormat::WEBP,
            ext: "webp",
            full_ext: "webp",
            mimetype: "image/webp",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );

    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/webp/lossy.webp"),
        ImageInfo {
            format: ImageFormat::WEBP,
            ext: "webp",
            full_ext: "webp",
            mimetype: "image/webp",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_tga() {
    assert_eq_ok!(
        ImageInfo::from_file_path("images/valid/tga/sample.tga"),
        ImageInfo {
            format: ImageFormat::TGA,
            ext: "tga",
            full_ext: "tga",
            mimetype: "image/tga",
            size: ImageSize {
                width: 123,
                height: 456
            },
            entry_sizes: vec![],
        }
    );
}

#[test]
fn test_io_error() {
    assert_eq_io_err!(
        ImageInfo::from_file_path("not_found.png"),
        std::io::ErrorKind::NotFound
    );
}

#[test]
fn test_unrecognized() {
    assert_unrecognized_err!(ImageInfo::from_file_path("images/invalid/sample.png"));
}
