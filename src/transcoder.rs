use crate::formats::Format;

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

fn transcode(contents: &str, input_format: Format, output_format: Format) {
    match (input_format, output_format) {
        (Format::Yaml, Format::Json) => {
            yaml_to_json(contents);
        }
        (Format::Json, Format::Yaml) => {
            json_to_yaml(contents);
        }
        _ => panic!("Won't go here"),
    };
}

fn yaml_to_json(contents: &str) -> String {
    let json_value: JsonValue = serde_yaml::from_str(contents).unwrap();
    let json_string = serde_json::to_string(&json_value).unwrap();

    json_string.to_string()
}

fn json_to_yaml(contents: &str) -> String {
    let yaml_value: YamlValue = serde_json::from_str(contents).unwrap();
    let yaml_string = serde_yaml::to_string(&yaml_value).unwrap();

    yaml_string.to_string()
}

#[cfg(test)]
mod test_helpers {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Basic {
        name: String,
    }
    impl Basic {
        pub fn new() -> Self {
            Self {
                name: "John Doe".to_string(),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Details {
        age: u8,
        height: u8,
        likes: [String; 3],
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Intermediate {
        name: String,
        details: Details,
    }

    impl Intermediate {
        pub fn new() -> Self {
            Self {
                name: "John Doe".to_string(),
                details: Details {
                    age: 25,
                    height: 186,
                    likes: [
                        "cheese".to_string(),
                        "the color blue".to_string(),
                        "rock music".to_string(),
                    ],
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;

    #[test]
    fn json_to_yaml_basic() {
        let input_format = Format::Json;
        let output_format = Format::Yaml;
        let input = Basic::new();
        let input_contents = serde_json::to_string(&input).unwrap();

        let output = transcode(&input_contents, input_format, output_format);

        let desired_output = serde_yaml::to_string(&input).unwrap();

        assert_eq!(output, desired_output);
    }

    #[test]
    fn json_to_yaml_with_array() {
        let input_format = Format::Json;
        let output_format = Format::Yaml;
        let input = Intermediate::new();
        let input_contents = serde_json::to_string(&input).unwrap();
        let output = transcode(&input_contents, input_format, output_format);

        let desired_output = serde_yaml::to_string(&input).unwrap();
        assert_eq!(output, desired_output);
    }
}
