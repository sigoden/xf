use assert_cmd::Command;

#[cfg(unix)]
mod unix_tests;

#[cfg(windows)]
mod windows_tests;

#[macro_export]
macro_rules! assert_output {
    ($assert:expr, $output:literal, $from:expr, $to:expr) => {
        let output = $assert.get_output();
        let content = String::from_utf8_lossy(&output.stdout);
        let content = content.replace($from, $to);
        assert_eq!(&content, $output)
    };
}

fn env_config_path() -> String {
    format!("{}_CONFIG_PATH", assert_cmd::crate_name!())
}

fn xf() -> Command {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.env_remove(&env_config_path());
    cmd
}

fn fixtures_dir(paths: &[&str]) -> String {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("tests");
    dir.push("fixtures");
    for p in paths {
        dir.push(p)
    }
    dir.to_string_lossy().escape_debug().to_string()
}
