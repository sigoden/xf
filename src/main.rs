use std::{
    env::{self, args, current_exe},
    fs,
    path::Path,
    process::{exit, ExitStatus},
};

use anyhow::{anyhow, Result};
use xf::Runner;

fn main() {
    match run() {
        Ok(status) => exit(status.code().unwrap_or_default()),
        Err(err) => {
            eprintln!("error: {}", err);
        }
    }
}

fn run() -> Result<ExitStatus> {
    let args: Vec<String> = args().collect();
    let cwd = env::current_dir().map_err(|e| anyhow!("Fail to get cwd, {}", e))?;
    let exe_name = current_exe()?;
    let exe_name = exe_name
        .file_stem()
        .and_then(|v| v.to_str())
        .ok_or_else(|| anyhow!("Fail to get exe name"))?;
    let rules = load_config_by_user_var(exe_name).or_else(|| load_config_by_home_var(exe_name));
    let runner = Runner::create(rules, exe_name)?;
    runner.run(&cwd, &args[1..])
}

fn load_config_by_user_var(exe_name: &str) -> Option<String> {
    let env_name = format!("{}_CONFIG_PATH", exe_name.to_uppercase());
    let config_file = match env::var(env_name) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let config_file = Path::new(&config_file);
    load_config_file(config_file)
}

fn load_config_by_home_var(exe_name: &str) -> Option<String> {
    let env_name = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    let config_file = match env::var(env_name) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let mut config_file = Path::new(&config_file).to_path_buf();
    config_file.push(format!(".{}", exe_name));
    load_config_file(&config_file)
}

fn load_config_file(path: &Path) -> Option<String> {
    if !path.exists() {
        return None;
    }
    fs::read_to_string(path).ok()
}
