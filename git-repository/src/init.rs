use std::path::PathBuf;

use crate::Kind;

const GIT_DIR_NAME: &str = ".git";

pub fn init<T: Into<PathBuf>>(directory: T, kind: Kind) {
    let root = PathBuf::from(directory.into()).join(GIT_DIR_NAME);
    println!("{:?}, {:?}", root, kind);
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::Kind;

    use super::init;

    #[test]
    fn test_init() {
        init("test_dir", Kind::Repo);
    }
}