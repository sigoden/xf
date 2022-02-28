use crate::{assert_output, fixtures_dir, xf, ENV_NAME};

#[test]
fn basic() {
    let assert = xf()
        .current_dir("tests/fixtures/dir1")
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Taskfile
fileDir <dir>/dir1
currentDir: <dir>/dir1
args: a b
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}

#[test]
fn with_config() {
    let assert = xf()
        .current_dir("tests/fixtures/dir1")
        .env(ENV_NAME, &fixtures_dir(&["dir1", "config1"]))
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Taskfile
fileDir <dir>/dir1
currentDir: <dir>/dir1
args: foo a b
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}

#[test]
fn with_config_contains_spaces_and_comment() {
    let assert = xf()
        .current_dir("tests/fixtures/dir1")
        .env(ENV_NAME, &fixtures_dir(&["dir1", "config2"]))
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Taskfile
fileDir <dir>/dir1
currentDir: <dir>/dir1
args: foo a b
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}

#[test]
fn search_parent_path() {
    let assert = xf()
        .current_dir("tests/fixtures/dir1/dir2")
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Taskfile
fileDir <dir>/dir1
currentDir: <dir>/dir1/dir2
args: a b
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}
