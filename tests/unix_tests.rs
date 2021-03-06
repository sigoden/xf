use crate::{assert_output, fixtures_dir, xf, xf_with_config};

#[test]
fn basic() {
    let assert = xf()
        .current_dir("tests/fixtures/dir1")
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Xfile
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
    let assert = xf_with_config(&["dir1", "config1"])
        .current_dir("tests/fixtures/dir1")
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Xfile
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
    let assert = xf_with_config(&["dir1", "config2"])
        .current_dir("tests/fixtures/dir1")
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Xfile
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
        r#"file: <dir>/dir1/Xfile
fileDir <dir>/dir1
currentDir: <dir>/dir1/dir2
args: a b
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}

#[test]
fn rule_precedence() {
    let assert = xf_with_config(&["dir1", "config3"])
        .current_dir("tests/fixtures/dir1")
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>/dir1/Zfile
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}

#[test]
fn xf_help() {
    let assert = xf().args(&["--xf-version"]).assert().success();
    let output = assert.get_output();
    let content = String::from_utf8_lossy(&output.stdout);
    assert!(content.starts_with("xf "))
}
