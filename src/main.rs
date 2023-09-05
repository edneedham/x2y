use clap::{Arg, ArgMatches, Command};
use std::fs::{self, *};
use std::io::Write;
use std::path::{Path, PathBuf};
use x2y::error::X2YError;
use x2y::formats::Format;
use x2y::transcoder;
use x2y::traversal::walk_dir;

fn main() {
    env_logger::init();
    let app = App::matches();
    let Ok(()) = app.run() else {
        panic!("Unable to run app");
    };
}

#[derive(Debug)]
pub struct App {
    pub matches: ArgMatches,
}

impl App {
    pub fn matches() -> Self {
        let app = Command::new("x2y")
            .version("0.1.0")
            .author("Ed Needham <ed@edneedham.dev>")
            .about("A data-serialization language transcoder.")
            .arg(
                Arg::new("INPUT")
                    .help("A filesystem entry, can be a file or directory.")
                    .required(true),
            )
            .arg(
                Arg::new("input-format")
                    .long("input-format")
                    .short('x')
                    .help("The input format of the filesystem entry.")
                    .long_help(
                        "The file format of the input file(s).\n\n\
                        Possible values:\n  
                        yaml\n
                        toml\n
                        json\n",
                    ),
            )
            .arg(
                Arg::new("output-format")
                    .long("output-format")
                    .short('y')
                    .help("The output format.")
                    .long_help(
                        "The output format.\n\n\
                        Possible values:\n  
                        yaml\n
                        toml\n
                        json\n",
                    )
                    .required(true),
            );

        Self {
            matches: app.get_matches(),
        }
    }
    pub fn run(&self) -> Result<(), X2YError> {
        let input = self.matches.get_one::<String>("INPUT").unwrap();
        let Ok(metadata) = fs::metadata(input) else {
            // Need a valid input.
            // If we can't determine the file type we don't know how to process it.
            return Err(X2YError::InvalidInput(format!(
                "failed to get input metadata: {:?}", 
                &input
            )));
        };
        let file_type = metadata.file_type();

        let input_format = self.matches.get_one::<String>("input-format");
        let output_format = self.matches.get_one::<String>("output-format");
        // What file formats are we going to look for
        if file_type.is_dir() && input_format.is_some() {
            log::debug!(
                "Processing directory: {:?}\t{:?}\t{:?}",
                input,
                input_format,
                output_format
            );
            process_directory(
                input.into(),
                input_format.unwrap().as_ref(),
                output_format.unwrap().as_ref(),
            )?;
        } else if file_type.is_file() {
            log::debug!(
                "Processing file: {:?}\t{:?}\t{:?}",
                input,
                input_format,
                output_format
            );
            process_file(input.into(), output_format.unwrap().as_ref())?;
        } else {
            return Err(X2YError::InvalidInput(format!(
                "unable to perform operations on file type: {:?}",
                file_type
            )));
        };
        Ok(())
    }
}
pub fn process_directory(
    directory: PathBuf,
    input_format: &Path,
    output_format: &Path,
) -> Result<(), X2YError> {
    let mut files: Vec<DirEntry> = Vec::new();
    walk_dir(directory, &mut files);
    for f in files {
        let input_format = input_format.try_into().unwrap();
        let output_format = output_format.try_into().unwrap();
        log::debug!("File formats: {}\t{}", input_format, output_format);
        let file_path = f.path();
        let contents = fs::read_to_string(&file_path).unwrap();
        let output_contents = transcoder::transcode(&contents, input_format, output_format);

        remove_file(&file_path)?;

        // The file could be invalid unicode when part of a directory.
        // Ideally we would continue processing files and log the error.
        // As of right now, a non-unicode file name stops the program.
        let new_path = new_path(&file_path, input_format)?;
        let mut file = create_file(new_path, output_format)?;
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

pub fn process_file(file: PathBuf, output_format: &Path) -> Result<(), X2YError> {
    let input_format: Format = file.as_path().try_into().unwrap();
    let output_format = output_format.try_into().unwrap();
    log::debug!(
        "File formats:\n Input Format: {} Output Format: {}",
        input_format,
        output_format
    );
    let contents = fs::read_to_string(&file).unwrap();
    let output_contents = transcoder::transcode(&contents, input_format, output_format);

    remove_file(&file)?;

    // The file could be invalid unicode when part of a directory.
    // Ideally we would continue processing files and log the error.
    // As of right now, a non-unicode file name stops the program.
    let new_path = new_path(&file, input_format)?;
    let mut file = create_file(new_path, output_format)?;
    file.write_all(output_contents.as_bytes())?;
    Ok(())
}

/// Wrappers around common file system operations.
pub fn open_file(file: &Path) -> Result<File, X2YError> {
    match File::open(file) {
        Ok(f) => Ok(f),
        Err(e) => Err(X2YError::IO(e)),
    }
}

pub fn remove_file(file: &Path) -> Result<(), X2YError> {
    match fs::remove_file(file) {
        Ok(()) => Ok(()),
        Err(e) => Err(X2YError::IO(e)),
    }
}

pub fn create_file(file_name: &str, format: Format) -> Result<File, X2YError> {
    match File::create(format!("{}{}", file_name, format)) {
        Ok(f) => Ok(f),
        Err(e) => Err(X2YError::IO(e)),
    }
}
