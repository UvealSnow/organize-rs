use std::fmt::{Debug, Display, Formatter, Result};
use std::path::PathBuf;

pub struct FileEntry<'a> {
    path: &'a PathBuf,
    extension: &'a str,
}

pub enum FileType {
    File,
    Dir,
    SymLink,
    Unknown,
}

impl<'a> FileEntry<'a> {
    pub fn new(path: &'a PathBuf, extension: &'a str) -> Self {
        FileEntry { path, extension }
    }

    pub fn get_category(&self) -> &str {
        match self.extension {
            "jpg" | "png" => "Images",
            "docx" | "pdf" => "Documents",
            _ => "Misc",
        }
    }
}

impl<'a> Debug for FileEntry<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "FileEntry {{ path: {:?}, extension: {:?}, category: {:?} }}",
            self.path,
            self.extension,
            self.get_category(),
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
