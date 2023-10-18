use log;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use crate::format::Format;

// We look through all of the directories starting from the input directory,
// adding only files ending with .yaml or .yml to a vector.

pub fn walk_dir(directory: &Path, files: &mut Vec<DirEntry>) {
    log::info!("Walking directory: {:?}", &directory);
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let file_type = metadata.file_type();
                if file_type.is_file() {
                    let entry_path = entry.path();
                    let as_path = entry_path.as_path();
                    if Format::try_from(as_path).is_ok() {
                        log::info!("Adding file: {:?} to file list", &as_path);
                        files.push(entry);
                    } else {
                        continue;
                    }
                } else if file_type.is_dir() {
                    walk_dir(&entry.path(), files);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile;

    fn create_file_names(count: usize) -> Vec<String> {
        let mut output = Vec::with_capacity(count - 1);
        for i in 0..count {
            output.push(format!("{}.yaml", i));
        }
        output
    }

    #[test]
    fn finds_all_target_files() {
        let directory1 = tempfile::TempDir::new().unwrap();

        let outer_path = directory1.path().to_str().unwrap();
        let directory2 = tempfile::Builder::new().tempdir_in(outer_path).unwrap();

        let file_names = create_file_names(10);
        for i in 0..file_names.len() {
            let file_path: PathBuf;
            // Add files at different depths
            if i % 2 == 0 {
                file_path = directory1.path().join(&file_names[i]);
            } else {
                file_path = directory2.path().join(&file_names[i]);
            }
            File::create(file_path).unwrap();
        }
        let mut target_files = Vec::with_capacity(file_names.len());

        walk_dir(&Path::new(outer_path), &mut target_files);

        assert_eq!(target_files.len(), 10);
    }
}
