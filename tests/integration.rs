use assert_cmd::Command;
use serde::Serialize;
use serde_json;
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::Write;
use tempfile;

extern crate x2y;
use x2y::test_helpers::{Basic, Intermediate};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn a_yaml_file_is_converted_to_json() {
    init_logger();
    let test_example = Basic::new();
    let yaml_string = serde_yaml::to_string(&test_example).unwrap();
    let file_name = "file.yaml";

    let mut file = File::create(file_name).unwrap();
    file.write_all(yaml_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y json").arg(file_name).assert();

    assert.success();
}

#[test]
fn a_yaml_file_in_a_directory_is_converted_to_json() {
    init_logger();
    let test_example = Basic::new();
    let yaml_string = serde_yaml::to_string(&test_example).unwrap();
    let file_name = "directory.yaml";

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
