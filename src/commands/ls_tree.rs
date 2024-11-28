pub fn ls_tree(_: bool, object_hash: String) {
    let path = format!(".git/objects/{}/{}", &object_hash[0..2], &object_hash[2..]);
}

