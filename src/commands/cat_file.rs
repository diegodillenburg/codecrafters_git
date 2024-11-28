use std::fs;
use crate::zlib::decode;

pub fn cat_file(_: bool, object_hash: String) {
    let path = format!(".git/objects/{}/{}", &object_hash[0..2], &object_hash[2..]);

    let file = fs::read(path).unwrap();
    let body = decode(file, "blob");

    print!("{}", body);
}
