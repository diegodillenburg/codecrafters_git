use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,
        file: String,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            init();
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            cat_file(pretty_print, object_hash);
        },
        Command::HashObject {
            write,
            file,
        } => {
            hash_object(write, file);
        }
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

fn encode(file: String) -> String {
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

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

fn cat_file(_: bool, object_hash: String) {
    let path = format!(".git/objects/{}/{}", &object_hash[0..2], &object_hash[2..]);

    let file = fs::read(path).unwrap();
    let body = decode(file);

    print!("{}", body);
}

fn hash_object(_: bool, file: String) {
    let file = fs::read(file).unwrap();
    let size = file.len();
    let content = String::from_utf8(file).unwrap();
    let file = format!("blob {}\0{}", size, content);

    let hash = encode(file);

    println!("{}", hash);
}
