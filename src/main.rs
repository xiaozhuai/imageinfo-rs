use imageinfo::ImageInfo;

fn main() {
    for filepath in std::env::args().skip(1) {
        println!("File: {}", filepath);
        match ImageInfo::from_file_path(&filepath) {
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
                println!("  - Error     : {}", err);
            }
        }
    }
}
