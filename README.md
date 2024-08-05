# imageinfo-rs

Rust library to get image size and format without loading/decoding.

The imageinfo don't get image format by file ext name, but infer by file header bytes and character.

A rewrite of c++ version [imageinfo](https://github.com/xiaozhuai/imageinfo)

Some test image files are from [image-size](https://github.com/image-size/image-size). Many thanks to [@netroy](https://github.com/netroy).

[![ci](https://github.com/xiaozhuai/imageinfo-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/xiaozhuai/imageinfo-rs/actions/workflows/ci.yml)

## Supported formats

* [x] avif
* [x] bmp
* [x] cur
* [x] dds
* [x] gif
* [x] hdr (pic)
* [x] heic (heif)
* [x] icns
* [x] ico
* [x] jpeg2000 code stream (j2k, j2c, jpc, jhc)
* [x] jpeg2000 (jp2, jpx, jph)
* [x] jpeg (jpg)
* [x] ktx
* [x] png
* [x] psd
* [x] qoi
* [ ] svg
* [x] tga
* [x] tiff (tif)
* [x] webp
* [ ] more coming...

## Installation
See [https://crates.io/crates/imageinfo](https://crates.io/crates/imageinfo)

## Example

```rust
use imageinfo::{ImageInfo};

fn main() {
    match ImageInfo::from_file_path("images/valid/bmp/sample.bmp") {
        Ok(info) => {
            println!("  - Ext       : {}", info.ext);
            println!("  - Full Ext  : {}", info.full_ext);
            println!("  - Size      : {}", info.size);
            println!("  - Mimetype  : {}", info.mimetype);
            println!("  - Entries   :");
            for size in info.entry_sizes.iter() {
                println!("    - {}", size);
            }
        }
        Err(err) => {
            println!("  - Err       : {}", err);
        }
    }
}
```

Pretty easy?

Don't be stingy with your star : )
