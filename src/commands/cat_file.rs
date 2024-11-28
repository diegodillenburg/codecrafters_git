use std::fs;
use std::io::{BufRead, BufReader, Read};
use flate2::read::ZlibDecoder;

pub fn cat_file(_: bool, object_hash: String) {
    let path = format!(".git/objects/{}/{}", &object_hash[0..2], &object_hash[2..]);

    let file = fs::read(path).unwrap();
    let body = decode(file);

    print!("{}", body);
}

pub fn decode(input: Vec<u8>) -> String {
    let decoder = ZlibDecoder::new(&input[..]);
    let mut reader = BufReader::new(decoder);
    let mut header = Vec::new();

    reader.read_until(0, &mut header).unwrap();

    let header = String::from_utf8(header).unwrap();

    let size = &header["blob ".len()..];
    let size: String = size.chars().filter(|&c| c != '\0').collect();
    let size = size.parse::<usize>().unwrap();

    let mut body = Vec::new();

    body.resize(size, 0);

    reader.read_exact(&mut body).unwrap();

    String::from_utf8(body).unwrap()
}

