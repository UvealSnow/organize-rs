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

        for entity in &self.files_to_move {
            let file_name = entity.get_file_name();
            result.push(format!(
                "{}/{} => {}",
                entity.get_current_path(),
                file_name,
                entity.get_new_path(),
            ));
        }

        result
    }

    #[allow(dead_code)]
    pub fn reverse_changes(&self) {
        todo!()
    }
}
