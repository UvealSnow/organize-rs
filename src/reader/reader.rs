use super::file_entry::{FileEntry, FileType as FT};
use std::fs::{self, File};
use std::path::Path;

pub struct Reader<'a> {
    base_path: &'a str,
}

impl<'a> Reader<'a> {
    pub fn new(base_path: &'a str) -> Self {
        Reader { base_path }
    }

    pub fn read_folder(&self) -> Vec<FileEntry> {
        let path = Path::new(self.base_path);
        let files = fs::read_dir(path).expect("Failed to read directory");
        let mut file_entries: Vec<FileEntry> = vec![];

        for file in files {
            let f = file.unwrap();
            let file_entry = FileEntry::new(&f);
            if file_entry.file_type == FT::File {
                file_entries.push(file_entry);
            }
        }

        file_entries
    }
}
