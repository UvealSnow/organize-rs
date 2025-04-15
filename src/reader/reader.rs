use super::file_entry::{FileEntry, FileType as FT};
use std::fs;
use std::fs::DirEntry;
use std::io::Error as IOError;
use std::path::{Path, PathBuf};

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
            let (path, file_type) = Self::get_file_details(file);
            if file_type == FT::File.to_string() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let file_entry = FileEntry::new(&path, ext);
                dbg!(file_entry);
            }
        }
    }

    fn get_file_details(file: Result<DirEntry, IOError>) -> (PathBuf, String) {
        if file.is_err() {
            panic!("Could not read file")
        }
        let entry = file.ok().unwrap();
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map(|ft| match ft {
                ft if ft.is_dir() => FT::Dir,
                ft if ft.is_file() => FT::File,
                ft if ft.is_symlink() => FT::SymLink,
                _ => FT::Unknown,
            })
            .unwrap_or(FT::Unknown)
            .to_string();

        (path, file_type)
    }
}
