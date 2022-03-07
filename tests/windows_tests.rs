use crate::{assert_output, fixtures_dir, xf_with_config};

#[test]
fn with_config() {
    let assert = xf_with_config(&["dir3", "config1"])
        .current_dir("tests/fixtures/dir3")
        .args(&["a", "b"])
        .assert()
        .success();
    assert_output!(
        assert,
        r#"file: <dir>\\dir3\\Xfile
fileDir <dir>\\dir3
currentDir: <dir>\\dir3
args: a b
"#,
        &fixtures_dir(&[]),
        "<dir>"
    );
}
