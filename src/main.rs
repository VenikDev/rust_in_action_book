use std::fs::File;

fn main() {
    let file = File::open("names.txt").expect("file not open");

    if let Ok(metainfo) = file.metadata() {
        metainfo;
    }
}