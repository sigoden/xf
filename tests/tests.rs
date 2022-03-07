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
    format!("{}_CONFIG_PATH", assert_cmd::crate_name!().to_uppercase())
}

fn xf() -> Command {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.env_remove(&env_config_path());
    cmd.env_remove("HOME");
    cmd.env_remove("USERPROFILE");
    cmd
}

fn xf_with_config(paths: &[&str]) -> Command {
    let mut cmd = xf();
    cmd.env(env_config_path(), &fixtures_dir(paths));
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
