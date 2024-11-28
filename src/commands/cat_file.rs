use std::fs;
use crate::utils::Path;
use crate::zlib::decode;

pub fn cat_file(_: bool, object_hash: String) {
    let path = Path::build(Some("objects".to_string()), Some(object_hash));

    let path = path.build_path();

    let file = fs::read(path).unwrap();
    let body = decode(file, "blob");

    print!("{}", body);
}
