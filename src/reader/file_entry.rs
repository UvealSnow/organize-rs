use std::fmt::{Debug, Display, Formatter, Result};
use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;

pub struct FileEntry {
    pub file_type: FileType,
    pub path: PathBuf,
    pub extension: String,
    _metadata: Metadata,
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
            _metadata: metadata,
        }
    }

    pub fn get_category(&self) -> &str {
        match self.extension.as_str() {
            "jpg" | "png" => "Images",
            "docx" | "pdf" => "Documents",
            "ts" | "js" => "Trash",
            _ => "Misc",
        }
    }

    pub fn get_file_name(&self) -> String {
        let file_name_os_str = self.path.file_name().unwrap();
        let file_name_str = file_name_os_str.to_str().unwrap();
        let file_name_owned = String::from(file_name_str);
        file_name_owned.split('/').last().unwrap().to_string()
    }

    pub fn get_new_path(&self) -> String {
        format!("{}/{}", self.get_category(), self.get_file_name())
    }

    pub fn get_current_path(&self) -> String {
        self.path.parent().unwrap().to_str().unwrap().to_owned()
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
