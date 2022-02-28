use crate::{assert_output, fixtures_dir, xf, ENV_NAME};

#[test]
fn with_config() {
    let assert = xf()
        .current_dir("tests/fixtures/dir3")
        .env(ENV_NAME, &fixtures_dir(&["dir3", "config1"]))
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>\\dir3\\Taskfile
fileDir <dir>\\dir3
currentDir: <dir>\\dir3
args: a b
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}
