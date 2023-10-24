use crate::error::X2YError;
use crate::format::Format;
use crate::transcoder;
use crate::traversal::walk_dir;
use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::Path;

pub fn process_directory(
    directory: &Path,
    input_format: &Path,
    output_format: &Path,
) -> Result<(), X2YError> {
    let mut files: Vec<DirEntry> = Vec::new();
    walk_dir(directory, &mut files);
    if files.is_empty() {
        return Err(X2YError::InvalidInput(format!(
            "Directory: {:?} contains no files with this: {:?} format",
            directory, input_format
        )));
    }
    for f in files {
        let input_format = input_format.try_into()?;
        let output_format = output_format.try_into()?;
        log::debug!("File formats: {}\n{}", input_format, output_format);
        let file_path = f.path();
        let contents = fs::read_to_string(&file_path)?;
        let output_contents = transcoder::transcode(&contents, input_format, output_format)?;

        fs::remove_file(&file_path)?;

        // The file could be invalid unicode when part of a directory.
        // Ideally we would continue processing files and log the error.
        // As of right now, a non-unicode file name stops the program.
        let new_path = new_path(&file_path, input_format)?;
        let mut file = File::create(format!("{}{}", new_path, output_format))?;
        file.write_all(output_contents.as_bytes())?;
    }
    Ok(())
}

fn new_path(file_path: &Path, input_format: Format) -> Result<&str, X2YError> {
    match file_path.to_str() {
        Some(str) => match str.strip_suffix(&input_format.to_string()) {
            Some(str) => Ok(str),
            None => Err(X2YError::InvalidInput(format!(
                "unable to strip suffix: {:?}",
                file_path
            ))),
        },
        None => Err(X2YError::InvalidInput(format!(
            "unable to convert path to str: {:?}",
            file_path
        ))),
    }
}

pub fn process_file(file: &Path, output_format: &Path) -> Result<(), X2YError> {
    let input_format: Format = file.try_into()?;
    let output_format = output_format.try_into()?;
    log::debug!(
        "File formats:\n Input Format: {}\n Output Format: {}",
        input_format,
        output_format
    );
    let contents = fs::read_to_string(file).unwrap();
    let output_contents = transcoder::transcode(&contents, input_format, output_format)?;

    fs::remove_file(file)?;

    // The file could be invalid unicode when part of a directory.
    // Ideally we would continue processing files and log the error.
    // As of right now, a non-unicode file name stops the program.
    let new_path = new_path(file, input_format)?;
    let mut file = File::create(format!("{}{}", new_path, output_format))?;
    file.write_all(output_contents.as_bytes())?;
    Ok(())
}
