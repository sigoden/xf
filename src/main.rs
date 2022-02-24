use dirs::config_dir;
use std::{
    env::{self, args},
    fs,
    path::Path,
    process::{exit, ExitStatus},
};

use anyhow::{anyhow, Result};
use xf::Runner;

const NAME: &str = env!("CARGO_CRATE_NAME");

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
    let rules = load_config_file()?;
    let runner = Runner::create(rules)?;
    runner.run(&cwd, &args[1..])
}

fn load_config_file() -> Result<Option<String>> {
    let mut config_file = match config_dir() {
        Some(dir) => dir,
        None => return Ok(None),
    };
    config_file.extend(&[NAME, NAME]);
    let path = Path::new(&config_file);
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Fail to load config at {}, {}", path.to_string_lossy(), e))?;
    Ok(Some(content))
}
