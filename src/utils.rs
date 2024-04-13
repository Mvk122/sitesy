use std::{fs, path::PathBuf};


pub struct ReadDirIter {
    read_dir: fs::ReadDir,
}

impl ReadDirIter {
    pub fn new(path: &PathBuf) -> Result<Self, std::io::Error> {
        let read_dir = fs::read_dir(path)?;
        Ok(Self { read_dir })
    }
}

impl Iterator for ReadDirIter {
    type Item = fs::DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_dir.next() {
            Some(Ok(entry)) => Some(entry),
            Some(Err(err)) => {
                eprintln!("Error reading directory: {}", err);
                None
            }
            None => None,
        }
    }
}