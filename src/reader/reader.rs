use super::file_entry::{FileEntry, FileType as FT};
use std::fs;
use std::path::PathBuf;

pub struct Reader {
    target_dir: PathBuf,
}

impl Reader {
    pub fn new(target_dir: PathBuf) -> Self {
        Reader { target_dir }
    }

    pub fn read_folder(&self) -> Vec<FileEntry> {
        let files = fs::read_dir(&self.target_dir).expect("Failed to read directory");
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
