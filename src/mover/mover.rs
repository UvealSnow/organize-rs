use crate::reader::file_entry::FileEntry;

pub struct Mover {
    files_to_move: Vec<FileEntry>,
}

impl Mover {
    pub fn new(entries: Vec<FileEntry>) -> Self {
        Mover {
            files_to_move: entries,
        }
    }

    pub fn generate_change_log() {
        todo!();
    }

    pub fn reverse_changes() {
        todo!()
    }
}
