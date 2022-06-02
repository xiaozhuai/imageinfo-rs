# imageinfo-rs

一个高性能的Rust库，在不加载/解码图片的情况下，获取图片文件类型和大小。

imageinfo 并不是通过扩展名来识别图片格式，而是通过文件头和文件格式特征来判断图片格式。

使用Rust重写 c++ 版的 [imageinfo](https://github.com/xiaozhuai/imageinfo)

部分测试图片文件来源于 [image-size](https://github.com/image-size/image-size) ，感谢 [@netroy](https://github.com/netroy)

## 支持格式

* [x] avif
* [x] bmp
* [x] cur
* [x] dds
* [x] gif
* [x] hdr (pic)
* [x] heic (heif)
* [x] icns
* [x] ico
* [x] jp2
* [x] jpeg (jpg)
* [x] jpx
* [x] ktx
* [x] png
* [x] psd
* [x] qoi
* [ ] svg
* [x] tga
* [x] tiff (tif)
* [x] webp
* [ ] more coming...

## 示例

```toml
[dependencies]
imageinfo = "0.6.0"
```

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

很简单不是吗？

请不要吝啬你的Star : )
