use std::fs;
use crate::zlib;

pub struct TreeObject {
    mode: String,
    name: String,
    object_hash: String,
}

impl TreeObject {
    pub fn build(mode: String, name: String, object_hash: String) -> TreeObject {
        Self { mode, name, object_hash, }
    }

    pub fn kind(&self) -> String {
        if self.mode == "40000" {
            "tree".to_string()
        } else {
            "blob".to_string()
        }
    }
}

pub fn ls_tree(name_only: bool, object_hash: String) {
    let path = format!(".git/objects/{}/{}", &object_hash[0..2], &object_hash[2..]);

    let file = fs::read(path).unwrap();
    let body = zlib::decode_tree_obj(file);

    for obj in body {
        output_obj(&obj, name_only);
    }
}

fn output_obj(obj: &TreeObject, name_only: bool) {
    if name_only {
        println!("{}", obj.name);
    } else {
        println!("{} {} {}\t{}", obj.mode, obj.kind(), obj.object_hash, obj.name);
    }
}
