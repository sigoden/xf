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
    let rules = load_config_file(exe_name)?;
    let runner = Runner::create(rules, exe_name)?;
    runner.run(&cwd, &args[1..])
}

fn load_config_file(exe_name: &str) -> Result<Option<String>> {
    let env_name = format!("{}_CONFIG_PATH", exe_name);
    let config_file = match env::var(&env_name) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    let path = Path::new(&config_file);
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Fail to load config at {}, {}", path.to_string_lossy(), e))?;
    Ok(Some(content))
}
