use std::fs;
use std::io::{BufWriter, Write};
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

pub fn hash_object(_: bool, file: String) {
    let file = fs::read(file).unwrap();
    let size = file.len();
    let content = String::from_utf8(file).unwrap();
    let file = format!("blob {}\0{}", size, content);

    let hash = encode(file);

    println!("{}", hash);
}

pub fn encode(file: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(file.as_bytes());

    let blob_sha = hasher.finalize();
    let blob_sha: Vec<u8> = blob_sha.to_vec();
    let blob_sha = hex::encode(blob_sha);
    let path = format!(".git/objects/{}/{}", &blob_sha[0..2], &blob_sha[2..]);

    fs::create_dir(format!(".git/objects/{}", &blob_sha[0..2])).unwrap_or_else(|_| {});
    let writer = fs::File::create(path).unwrap();

    let encoder = ZlibEncoder::new(writer, flate2::Compression::default());

    let mut writer = BufWriter::new(encoder);

    writer.write_all(file.as_bytes()).unwrap();

    blob_sha
}



