use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::commands::ls_tree::TreeObject;

pub fn decode_tree_obj(input: Vec<u8>) -> Vec<TreeObject> {
    let decoder = ZlibDecoder::new(&input[..]);
    let mut reader = BufReader::new(decoder);
    let mut header = Vec::new();
    reader.read_until(0, &mut header).unwrap();

    let mut tree_objects = Vec::new();

    loop {
        let mut entry = Vec::new();

        if reader.read_until(0, &mut entry).unwrap() == 0 {
            break; // EOF
        }

        if let  Some(&0) = entry.last() {
            entry.pop();
        }

        let entry = String::from_utf8(entry).unwrap();
        let parts: Vec<&str> = entry.splitn(2, ' ').collect();

        if parts.len() < 2 {
            continue;
        }

        let mode = parts[0].to_string();
        let filename = parts[1].to_string();

        let mut sha_bytes = [0; 20];
        reader.read_exact(&mut sha_bytes).unwrap();

        let sha = hex::encode(sha_bytes);

        tree_objects.push(TreeObject::build(mode, filename, sha));
    }

    tree_objects
}

pub fn decode(input: Vec<u8>, head: &str) -> String {
    let decoder = ZlibDecoder::new(&input[..]);
    let mut reader = BufReader::new(decoder);
    let mut header = Vec::new();

    reader.read_until(0, &mut header).unwrap();

    let header = String::from_utf8(header).unwrap();

    let size = &header[(head.len() + 1)..];
    let size: String = size.chars().filter(|&c| c != '\0').collect();
    let size = size.parse::<usize>().unwrap();

    let mut body = Vec::new();

    body.resize(size, 0);

    reader.read_exact(&mut body).unwrap();

    String::from_utf8(body).unwrap()
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
