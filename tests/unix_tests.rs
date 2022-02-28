use assert_cmd::Command;

macro_rules! assert_output {
    ($assert:expr, $output:literal, $from:expr, $to:expr) => {
        let output = $assert.get_output();
        let content = String::from_utf8_lossy(&output.stdout);
        let content = content.replace($from, $to);
        assert_eq!(&content, $output)
    };
}

fn xf() -> Command {
    Command::cargo_bin(assert_cmd::crate_name!()).unwrap()
}

fn fixtures_dir(paths: &[&str]) -> String {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("tests");
    dir.push("fixtures");
    for p in paths {
        dir.push(p)
    }
    dir.to_string_lossy().to_string()
}

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
        .env("XF_CONFIG_PATH", &fixtures_dir(&["dir1", "config1"]))
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
