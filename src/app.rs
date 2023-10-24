use crate::error::X2YError;
use crate::fs::*;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(
    name = "x2y",
    author,
    version,
    about = "A data-serialisation file transcoder"
)]
pub struct Args {
    #[arg(
        value_name = "input",
        help = "a filesystem entry, can be a file or directory",
        long_help
    )]
    input: String,
    #[arg(
        short = 'x',
        long = "input_format",
        help = "the format of the input file(s)",
        long_help = "the format of the input file(s)
          \n\
          for single files, this value can be omitted.\n
          Possible values: \n
          yaml\n
          toml\n
          json\n"
    )]
    input_format: Option<String>,
    #[arg(
        short = 'y',
        long = "output_format",
        help = "the output format for the file(s)",
        long_help = "the output format for the file(s)
          \n\
          Possible values: \n
          yaml\n
          toml\n
          json\n"
    )]
    output_format: String,
}

impl Args {
    pub fn run(&self) -> Result<(), X2YError> {
        log::info!("Running X2Y...");
        let Ok(metadata) = fs::symlink_metadata(&self.input) else {
            // Need a valid input.
            // If we can't determine the file type we don't know how to process it.
            return Err(X2YError::InvalidInput(format!(
                "failed to get input metadata: {:?}",
                &self.input
            )));
        };
        log::info!("Checking input file type");
        let file_type = metadata.file_type();

        let input_format = &self.input_format;
        let output_format = &self.output_format;
        // What file formats are we going to look for
        if file_type.is_dir() && input_format.is_some() {
            log::info!("Processing input as directory");
            process_directory(
                self.input.as_ref(),
                input_format.as_ref().unwrap().as_ref(),
                output_format.as_ref(),
            )?;
        } else if file_type.is_file() {
            log::info!("Processing input as file");
            process_file(self.input.as_ref(), self.output_format.as_ref())?;
        } else if file_type.is_symlink() {
            return Err(X2YError::InvalidInput(format!(
                "unable to perform operations on file type: {:?}",
                file_type
            )));
        };
        Ok(())
    }
}
