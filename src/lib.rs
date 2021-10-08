use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use ini::Ini;

#[derive(Default, Debug)]
pub struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
}

impl Repository {
    pub fn init(&mut self, path: &Path) {
        if !path.is_dir() {
            panic!("{} is not a directory", path.display());
        }
        if path.read_dir().unwrap().count() > 0 {
            panic!("{} is not empty", path.display());
        }

        self.worktree = path.to_path_buf();
        self.gitdir = path.join(".git");

        fs::create_dir(path.join(".git")).unwrap();
        fs::create_dir(self.gitdir.join("objects")).unwrap();
        fs::create_dir_all(self.gitdir.join("refs/tags")).unwrap();
        fs::create_dir(self.gitdir.join("refs/heads")).unwrap();

        let mut description = File::create(self.gitdir.join("description")).unwrap();
        description
            .write_all(b"Unnamed repository; edit this file 'description' to name the repository")
            .unwrap();

        let config = self.create_config();
        config
            .write_to_file(self.gitdir.join("config").to_str().unwrap())
            .unwrap();
    }

    fn create_config(&self) -> Ini {
        let mut conf = Ini::new();
        conf.with_section(Some("core"))
            .set("repositoryformatversion", "0")
            .set("filemode", "false")
            .set("bare", "false");

        conf
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_init() {
        fs::create_dir("/tmp/test").unwrap();
        let mut repo = Repository::default();
        repo.init(&Path::new("/tmp/test"));

        // TODO run cleanup after failed assert too
        assert!(Path::new("/tmp/test/.git").exists());
        assert!(Path::new("/tmp/test/.git/objects").exists());
        assert!(Path::new("/tmp/test/.git/refs/tags").exists());
        assert!(Path::new("/tmp/test/.git/refs/heads").exists());
        assert!(Path::new("/tmp/test/.git/config").exists());

        fs::remove_dir_all("/tmp/test").unwrap();
    }
}
