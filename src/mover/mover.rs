use crate::reader::file_entry::FileEntry;
use std::fs;

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
                "{} => {}",
                format!("{}/{}", entry.get_current_path(), file_name),
                format!(
                    "{}/{}/{}",
                    entry.get_current_path(),
                    entry.get_new_path(),
                    file_name
                ),
            ));
        }

        result
    }

    pub fn do_move_files(&self) -> Result<bool, String> {
        for entry in &self.files_to_move {
            let category = entry.get_new_path();
            let file_name = entry.get_file_name();
            let base_path = entry.get_current_path();
            let new_path = format!("{}/{}", base_path, category);

            // Create directory if it doesn't exist
            if !fs::exists(&new_path)
                .map_err(|e| format!("Mover: Error checking path existence: {}", e))?
            {
                fs::create_dir_all(&new_path).map_err(|e| {
                    format!("Mover: Failed to create directory at {}: {}", new_path, e)
                })?;
            }

            // Construct source and destination paths
            let from = format!("{}/{}", base_path, file_name);
            let to = format!("{}/{}", new_path, file_name);

            // Try to rename (move) the file
            if let Err(_) = fs::rename(&from, &to) {
                // If rename fails, try copy and delete as fallback
                fs::copy(&from, &to)
                    .map_err(|e| format!("Mover: Error copying file to {}: {}", &to, e))?;

                fs::remove_file(&from).map_err(|e| {
                    format!("Mover: Error deleting original file at {}: {}", &from, e)
                })?;
            }
        }
        Ok(true)
    }

    #[allow(dead_code)]
    pub fn reverse_changes(&self) {
        todo!()
    }
}
