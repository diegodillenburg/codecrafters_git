use crate::zlib::encode;
use std::fs;

pub fn hash_object(_: bool, file: String) {
    let file = fs::read(file).unwrap();
    let size = file.len();
    let content = String::from_utf8(file).unwrap();
    let file = format!("blob {}\0{}", size, content);

    let hash = encode(file);

    println!("{}", hash);
}
