use super::file_entry::{FileEntry, FileType as FT};
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Reader {
    target_dir: PathBuf,
    exclude: Vec<String>,
}

impl Reader {
    pub fn new(target_dir: PathBuf, exclude: Vec<String>) -> Self {
        Reader {
            target_dir,
            exclude,
        }
    }

    pub fn read_folder(&self) -> Vec<FileEntry> {
        let files = fs::read_dir(&self.target_dir).expect("Failed to read directory");
        let mut file_entries: Vec<FileEntry> = vec![];
        // let exclude = Regex::new(self.exclude).unwrap();

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
