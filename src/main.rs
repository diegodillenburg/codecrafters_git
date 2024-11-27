use std::env;
use std::fs;
use std::io::BufRead;
use std::process;

use std::io::BufReader;
use std::io::Read;

use flate2::read::ZlibDecoder;

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
        let dir = &blob_sha[0..2];
        let file = &blob_sha[2..];
        let path = format!(".git/objects/{}/{}", dir, file);
        let compressed_content = fs::read(path).unwrap();

        let decoder = ZlibDecoder::new(&compressed_content[..]);
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

        let body = String::from_utf8(body).unwrap();

        print!("{}", body);

    } else {
        println!("unknown command: {}", args[1])
    }
}
