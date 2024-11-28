use codecrafters_git::commands;
use clap::{Parser, Subcommand};

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
    },
    LsTree {
        #[clap(long = "name-only")]
        name_only: bool,
        object_hash: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            commands::init::init();
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            commands::cat_file::cat_file(pretty_print, object_hash);
        },
        Command::HashObject {
            write,
            file,
        } => {
            commands::hash_object::hash_object(write, file);
        },
        Command::LsTree {
            name_only,
            object_hash,
        } => {
            commands::ls_tree::ls_tree(name_only, object_hash);
        }
    }
}
