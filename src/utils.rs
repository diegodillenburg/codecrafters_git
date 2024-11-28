pub struct Path {
    base_dir: String,
    path: Option<String>,
    object_hash: Option<String>,
}

impl Path {
    pub fn build(path: Option<String>, object_hash: Option<String>) -> Path {
        let base_dir = if std::env::var("GIT_DIR").is_ok() {
            format!("{}/.git", std::env::var("GIT_DIR").unwrap())
        } else {
            ".git".to_string()
        };

        Self { base_dir, path, object_hash, }
    }

    pub fn build_path(&self) -> String {
        let path = self.path.as_ref().map(|hash| &hash[..]);
        let object_hash_dir = self.object_hash.as_ref().map(|hash| &hash[0..2]);
        let object_hash_filename = self.object_hash.as_ref().map(|hash| &hash[2..]);

        let components: Vec<&str> = [
            self.base_dir.as_str(),
            path.unwrap_or(""),
            object_hash_dir.unwrap_or(""),
            object_hash_filename.unwrap_or(""),
        ]
        .iter()
        .filter(|&component| !component.is_empty())
        .cloned()
        .collect();

        components.join("/")
    }
}
