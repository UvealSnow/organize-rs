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

    pub fn generate_change_log(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        for entry in &self.files_to_move {
            let file_name = entry.get_file_name();
            result.push(format!(
                "{}/{} => {}",
                entry.get_current_path(),
                file_name,
                entry.get_new_path(),
            ));
        }

        result
    }

    #[allow(dead_code)]
    pub fn reverse_changes(&self) {
        todo!()
    }
}
