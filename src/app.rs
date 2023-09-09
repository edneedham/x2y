use crate::error::X2YError;
use crate::fs::*;
use clap::{Arg, ArgMatches, Command};
use std::fs;

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
