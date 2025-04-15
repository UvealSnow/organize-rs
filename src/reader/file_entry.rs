use std::fmt::{Debug, Display, Formatter, Result};
use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;

pub struct FileEntry {
    pub file_type: FileType,
    pub path: PathBuf,
    pub extension: String,
    metadata: Metadata,
}

#[derive(PartialEq)]
pub enum FileType {
    File,
    Dir,
    SymLink,
    Unknown,
}

impl FileEntry {
    pub fn new(file: &DirEntry) -> Self {
        let metadata = file.metadata().unwrap();
        let file_type = file
            .file_type()
            .map(|ft| match ft {
                ft if ft.is_dir() => FileType::Dir,
                ft if ft.is_file() => FileType::File,
                ft if ft.is_symlink() => FileType::SymLink,
                _ => FileType::Unknown,
            })
            .unwrap_or(FileType::Unknown);
        let path = file.path();
        let extension = match file.path().extension() {
            None => String::new(),
            Some(i) => i.to_str().unwrap_or("").to_string(),
        };

        FileEntry {
            file_type,
            path,
            extension,
            metadata,
        }
    }

    pub fn get_category(&self) -> String {
        match self.extension.as_str() {
            "jpg" | "png" => "Images".to_string(),
            "docx" | "pdf" => "Documents".to_string(),
            _ => "Misc".to_string(),
        }
    }
}

impl Debug for FileEntry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "FileEntry {{ path: {:?}, type: {:?}, extension: {:?}, category: {:?} }}",
            self.path,
            self.file_type,
            self.extension,
            self.get_category()
        )
    }
}

impl FileType {
    fn message(&self) -> &str {
        match self {
            Self::File => "File",
            Self::Dir => "Directory",
            Self::SymLink => "SymLink",
            Self::Unknown => "Unknown",
        }
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for FileType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message())
    }
}
