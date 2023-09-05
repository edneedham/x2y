use std::fmt;
use std::path::Path;

use crate::error::X2YError;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Format {
    Yaml,
    Json,
}

impl TryFrom<&Path> for Format {
    type Error = X2YError;

    fn try_from(s: &Path) -> Result<Self, Self::Error> {
        if let Some(extension) = s.extension() {
            if let Some(format) = extension.to_str() {
                match format.trim() {
                    "yaml" => Ok(Self::Yaml),
                    "yml" => Ok(Self::Yaml),
                    "json" => Ok(Self::Json),
                    other => Err(X2YError::InvalidInput(format!(
                        "{} is not a supported file format.",
                        other
                    ))),
                }
            } else {
                Err(X2YError::InvalidInput(format!(
                    "{:?} contains invalid unicode.",
                    extension
                )))
            }
        } else if let Some(s) = s.to_str() {
            match s.trim() {
                "yaml" => Ok(Self::Yaml),
                "yml" => Ok(Self::Yaml),
                "json" => Ok(Self::Json),
                other => Err(X2YError::InvalidInput(format!(
                    "{} is not a supported file format.",
                    other
                ))),
            }
        } else {
            Err(X2YError::InvalidInput(format!(
                "{:?} has no retrievable extension.",
                s
            )))
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::Yaml => write!(f, "yaml"),
            Format::Json => write!(f, "json"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use claims::*;

    #[test]
    fn a_format_is_extracted_from_a_path() {
        let input = PathBuf::from("directory/test.yaml");
        let path = input.as_path();
        let format = Format::try_from(path).unwrap();
        assert_eq!(Format::Yaml, format);
    }

    #[test]
    fn a_path_containing_an_unsupported_file_format_is_not_extracted() {
        let input = PathBuf::from("directory/test.cpp");
        let path = input.as_path();
        let format = Format::try_from(path);
        assert_err!(format);
    }

    #[test]
    fn a_path_of_just_a_file_format_is_extracted() {
        let input = PathBuf::from("yaml");
        let path = input.as_path();
        let format = Format::try_from(path).unwrap();
        assert_eq!(Format::Yaml, format);
    }
}
