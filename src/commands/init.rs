use std::fs;
use std::env;

pub fn init() {
    let git_dir = if env::var("GIT_DIR").is_ok() {
        Some(env::var("GIT_DIR").unwrap())
    } else {
        None
    };

    let path = if git_dir.is_none() {
        ".git".to_string()
    } else {
        format!("{}/.git", git_dir.unwrap())
    };

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

    #[test]
    fn test_init() {
        init();

        assert!(fs::metadata(".git").is_ok());
    }
}
