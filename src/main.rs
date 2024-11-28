use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufWriter;
use std::process;

use std::io::BufReader;
use std::io::{Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Supply command");
        process::exit(1)
    }
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        let blob_sha = &args[3];
        let path = format!(".git/objects/{}/{}", &blob_sha[0..2], &blob_sha[2..]);

        let file = fs::read(path).unwrap();
        let body = decode(file);

        print!("{}", body);
    } else if args[1] == "hash-object" {
        let path = &args[3];
        let file = fs::read(path).unwrap();
        let size = file.len();
        let content = String::from_utf8(file).unwrap();
        let file = format!("blob {}\0{}", size, content);

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
        println!("{}", blob_sha);
    } else {
        println!("unknown command: {}", args[1])
    }
}

fn decode(input: Vec<u8>) -> String {
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
