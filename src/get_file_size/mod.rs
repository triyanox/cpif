use std::{fs::File, io::Read, vec};

pub fn get_file_size(path: &str) -> vec::Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut size = vec::Vec::new();
    file.read_to_end(&mut size).unwrap();
    size
}
