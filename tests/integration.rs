use assert_cmd::Command;
use once_cell::sync::Lazy;
use serde_json;
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::Write;
use tempfile;
use toml;

extern crate x2y;
use x2y::test_helpers::Basic;

static LOGGER: Lazy<()> = Lazy::new(|| {
    let _ = env_logger::builder().is_test(true).try_init();
});

#[test]
fn a_yaml_file_is_converted_to_json() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let yaml_string = serde_yaml::to_string(&test_example).unwrap();
    let file_name = "yaml_file";
    let file_path = format!("{}.yaml", file_name);

    let mut file = File::create(&file_path).unwrap();
    file.write_all(yaml_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y json").arg(&file_path).assert();

    fs::remove_file(format!("{}.json", file_name)).unwrap();
    assert.success();
}

#[test]
fn a_toml_file_is_converted_to_json() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let toml_string = toml::to_string_pretty(&test_example).unwrap();
    let file_name = "toml_file";
    let file_path = format!("{}.toml", file_name);

    let mut file = File::create(&file_path).unwrap();
    file.write_all(toml_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y json").arg(&file_path).assert();

    fs::remove_file(format!("{}.json", file_name)).unwrap();
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
    let file_name = "json_file";
    let file_path = format!("{}.json", file_name);

    let mut file = File::create(&file_path).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y yaml").arg(&file_path).assert();
    fs::remove_file(format!("{}.yaml", file_name)).unwrap();

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

    fs::remove_file(file_name).unwrap();

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

#[cfg(target_family = "unix")]
#[test]
fn supplying_a_symlink_on_unix_returns_an_error() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let json_string = serde_json::to_string_pretty(&test_example).unwrap();
    let file_path = "json_link_file.json";

    let mut file = File::create(&file_path).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    std::os::unix::fs::symlink(&file_path, "link.json").unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y yaml").arg("link.json").assert();

    fs::remove_file(file_path).unwrap();
    fs::remove_file("link.json").unwrap();
    assert.failure();
}

#[cfg(target_family = "windows")]
#[test]
fn supplying_a_symlink_on_windows_returns_an_error() {
    Lazy::force(&LOGGER);
    let test_example = Basic::new();
    let json_string = serde_json::to_string_pretty(&test_example).unwrap();
    let file_path = "json_link_file.json";

    let mut file = File::create(&file_path).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    std::os::windows::fs::symlink_file(&file_path, "link.json").unwrap();

    let mut cmd = Command::cargo_bin("x2y").unwrap();
    let assert = cmd.arg("-y yaml").arg("link.json").assert();

    fs::remove_file(file_path).unwrap();
    fs::remove_file("link.json").unwrap();
    assert.failure();
}
