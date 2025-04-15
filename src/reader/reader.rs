use super::file_entry::{FileEntry, FileType as FT};
use std::fs;
use std::path::Path;

pub struct Reader<'a> {
    base_path: &'a str,
}

impl<'a> Reader<'a> {
    pub fn new(base_path: &'a str) -> Self {
        Reader { base_path }
    }

    pub fn read_folder(&self) {
        let path = Path::new(self.base_path);
        let files = fs::read_dir(path).expect("Failed to read directory");

        for file in files {
            let f = file.unwrap();
            let file_entry = FileEntry::new(&f);
            if file_entry.file_type == FT::File {
                dbg!(file_entry);
            }
        }
    }
}
