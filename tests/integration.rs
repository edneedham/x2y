use assert_cmd::Command;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json;
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::Write;
use tempfile;

extern crate x2y;
use x2y::test_helpers::{Basic, Intermediate};

static LOGGER: Lazy<()> = Lazy::new(|| {
    let _ = env_logger::builder().is_test(true).try_init();
});

#[test]
fn a_yaml_file_is_converted_to_json() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let yaml_string = serde_yaml::to_string(&test_example).unwrap();
    let file_name = "basic_file.yaml";

    let mut file = File::create(file_name).unwrap();
    file.write_all(yaml_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y json").arg(file_name).assert();

    assert.success();
}

#[test]
fn a_yaml_file_in_a_directory_is_converted_to_json() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let yaml_string = serde_yaml::to_string(&test_example).unwrap();
    let file_name = "basic_dir.yaml";

    let dir = tempfile::tempdir().unwrap();
    let directory_path = dir.path();
    let mut file = File::create(directory_path.join(file_name)).unwrap();
    file.write_all(yaml_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd
        .arg("-x yaml")
        .arg("-y json")
        .arg(directory_path)
        .assert();

    assert.success();
}

#[test]
fn a_json_file_is_converted_to_yaml() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let json_string = serde_json::to_string_pretty(&test_example).unwrap();
    let file_name = "basic_file.json";

    let mut file = File::create(file_name).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y yaml").arg(file_name).assert();

    assert.success();
}

#[test]
fn a_json_file_in_a_directory_is_converted_to_yaml() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let json_string = serde_json::to_string_pretty(&test_example).unwrap();
    let file_name = "basic_dir.json";

    let dir = tempfile::tempdir().unwrap();
    let directory_path = dir.path();
    let mut file = File::create(directory_path.join(file_name)).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd
        .arg("-x json")
        .arg("-y yaml")
        .arg(directory_path)
        .assert();

    assert.success();
}

#[test]
fn supplying_the_same_format_for_input_and_output_returns_an_error() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let json_string = serde_json::to_string_pretty(&test_example).unwrap();
    let file_name = "basic_fail.json";

    let mut file = File::create(file_name).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y json").arg(file_name).assert();

    assert.failure();
}

#[test]
fn supplying_an_empty_directory_returns_an_error() {
    Lazy::force(&LOGGER);
    let dir = tempfile::tempdir().unwrap();
    let directory_path = dir.path();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd
        .arg("-x json")
        .arg("-y yaml")
        .arg(directory_path)
        .assert();

    assert.failure();
}
