use std::fs;
use crate::utils::Path;

pub fn init() {
    let path = Path::build(None, None).build_path();

    fs::create_dir(&path).unwrap();
    fs::create_dir(format!("{}/objects", &path)).unwrap();
    fs::create_dir(format!("{}/refs", &path)).unwrap();
    fs::write(format!("{}/HEAD", &path), "ref: refs/heads/main\n").unwrap();

    println!("Initialized git directory")
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // TODO: mock env::var("GIT_DIR")
    #[test]
    #[ignore] // needs to handle resetting previously initialized repo
    fn test_init() {
        init();

        assert!(fs::metadata(".git").is_ok());
    }
}
