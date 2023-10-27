use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let fp = "Path To JSON File";
    let cache_file = File::open(fp).expect("Failed to open cache file");
    let reader = BufReader::new(cache_file);

    // read the json file in fp and then encode it into the file
    let cache: serde_json::Value = serde_json::from_reader(reader).unwrap();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(fp)
        .unwrap();

    let serialized_cache = serde_bencode::to_string(&cache).unwrap();

    file.write_all(
        &zstd::encode_all(serialized_cache.as_bytes(), 0)
            .expect("Failed to compress cache contents.")[..],
    )
        .unwrap();
}