use crate::error::X2YError;
use crate::format::Format;

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use toml::Value as TomlValue;

pub fn transcode(
    contents: &str,
    input_format: Format,
    output_format: Format,
) -> Result<String, X2YError> {
    match (input_format, output_format) {
        (Format::Yaml, Format::Json) => yaml_to_json(contents),
        (Format::Yaml, Format::Toml) => yaml_to_toml(contents),
        (Format::Json, Format::Toml) => json_to_toml(contents),
        (Format::Json, Format::Yaml) => json_to_yaml(contents),
        (Format::Toml, Format::Json) => toml_to_json(contents),
        (Format::Toml, Format::Yaml) => toml_to_yaml(contents),
        _ => Err(X2YError::InvalidInput(format!(
            "Incompatible input and output formats: {} -> {}",
            input_format, output_format
        ))),
    }
}

fn yaml_to_json(contents: &str) -> Result<String, X2YError> {
    let json_value: JsonValue = match serde_yaml::from_str(contents) {
        Ok(v) => v,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    let json_string = match serde_json::to_string_pretty(&json_value) {
        Ok(s) => s,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    Ok(json_string.to_string())
}

fn yaml_to_toml(contents: &str) -> Result<String, X2YError> {
    let toml_value: TomlValue = match serde_yaml::from_str(contents) {
        Ok(v) => v,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    let toml_string = match toml::to_string_pretty(&toml_value) {
        Ok(s) => s,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    Ok(toml_string.to_string())
}

fn json_to_toml(contents: &str) -> Result<String, X2YError> {
    let toml_value: TomlValue = match serde_json::from_str(contents) {
        Ok(v) => v,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    let toml_string = match toml::to_string_pretty(&toml_value) {
        Ok(s) => s,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    Ok(toml_string.to_string())
}

fn json_to_yaml(contents: &str) -> Result<String, X2YError> {
    let yaml_value: YamlValue = match serde_json::from_str(contents) {
        Ok(v) => v,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    let yaml_string = match serde_yaml::to_string(&yaml_value) {
        Ok(s) => s,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    Ok(yaml_string.to_string())
}

fn toml_to_yaml(contents: &str) -> Result<String, X2YError> {
    let yaml_value: YamlValue = match toml::from_str(contents) {
        Ok(v) => v,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    let yaml_string = match serde_yaml::to_string(&yaml_value) {
        Ok(s) => s,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    Ok(yaml_string.to_string())
}

fn toml_to_json(contents: &str) -> Result<String, X2YError> {
    let json_value: YamlValue = match toml::from_str(contents) {
        Ok(v) => v,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    let json_string = match serde_json::to_string_pretty(&json_value) {
        Ok(s) => s,
        Err(e) => return Err(X2YError::Transcode(e.into())),
    };
    Ok(json_string.to_string())
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

        let output = transcode(&input_contents, input_format, output_format).unwrap();

        let desired_output = serde_yaml::to_string(&input).unwrap();

        assert_eq!(output, desired_output);
    }

    #[test]
    fn json_to_yaml_with_array() {
        let input_format = Format::Json;
        let output_format = Format::Yaml;
        let input = Intermediate::new();
        let input_contents = serde_json::to_string(&input).unwrap();
        let output = transcode(&input_contents, input_format, output_format).unwrap();

        let desired_output = serde_yaml::to_string(&input).unwrap();
        assert_eq!(output, desired_output);
    }
}
