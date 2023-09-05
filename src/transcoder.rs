use crate::formats::Format;

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

pub fn transcode(contents: &str, input_format: Format, output_format: Format) -> String {
    let result: String = match (input_format, output_format) {
        (Format::Yaml, Format::Json) => yaml_to_json(contents),
        (Format::Json, Format::Yaml) => json_to_yaml(contents),
        _ => panic!("Won't go here"),
    };
    result
}

fn yaml_to_json(contents: &str) -> String {
    let json_value: JsonValue = serde_yaml::from_str(contents).unwrap();
    let json_string = serde_json::to_string_pretty(&json_value).unwrap();

    json_string.to_string()
}

fn json_to_yaml(contents: &str) -> String {
    log::debug!("File contents: {}", contents);
    let yaml_value: YamlValue = serde_json::from_str(contents).unwrap();
    let yaml_string = serde_yaml::to_string(&yaml_value).unwrap();

    yaml_string.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

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
