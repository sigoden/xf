use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

pub struct Runner {
    rules: Vec<Rule>,
}

impl Runner {
    pub fn create(config: Option<String>, exe_name: &str) -> Result<Self> {
        let xf = Self {
            rules: Self::load_rules(config, exe_name)?,
        };
        Ok(xf)
    }

    pub fn run(&self, cwd: &Path, args: &[String]) -> Result<ExitStatus> {
        let (rule, file) = self
            .search_folder_recursive(cwd)
            .ok_or(anyhow!("Not found target file"))?;
        let state = State::new(cwd, &file);
        let env_vars = state.env_vars();
        let args = state
            .build_args(rule, args)
            .map_err(|_| anyhow!("Fail to build args for running {}", state.file))?;
        let mut command = Command::new(&args[0]);
        command.args(&args[1..]);
        command.current_dir(&state.file_dir);
        command.envs(&env_vars);
        command
            .status()
            .map_err(|e| anyhow!("Run file {} throw {}", &state.file, e))
    }

    fn load_rules(config_content: Option<String>, exe_name: &str) -> Result<Vec<Rule>> {
        match config_content {
            None => Ok(vec![Rule::get_exe_rule(exe_name)]),
            Some(text) => {
                let mut rules = Vec::new();
                for (idx, line) in text.lines().enumerate() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    let rule = Rule::create(line)
                        .map_err(|_| anyhow!("Config file has invalid rule at line {}", idx + 1))?;
                    rules.push(rule);
                }
                rules.push(Rule::get_exe_rule(exe_name));
                Ok(rules)
            }
        }
    }

    fn search_folder_recursive(&self, mut folder: &Path) -> Option<(&Rule, PathBuf)> {
        loop {
            let result = self.search_folder(folder);
            if result.is_some() {
                return result;
            }
            folder = folder.parent()?
        }
    }

    fn search_folder(&self, folder: &Path) -> Option<(&Rule, PathBuf)> {
        let paths = fs::read_dir(folder).ok()?;
        for path in paths {
            let path = path.ok()?.path();
            if !path.is_file() {
                continue;
            }
            if let Some(rule) = self.match_rule(&path) {
                return Some((rule, path));
            }
        }
        None
    }

    fn match_rule(&self, file: &Path) -> Option<&Rule> {
        self.rules.iter().find(|v| v.is_match_file(file))
    }
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    shell: String,
}

impl Rule {
    pub fn create(text: &str) -> Result<Self> {
        let pos = text.find(':').ok_or_else(|| anyhow!("Invalid rule"))?;
        let (name, shell) = text.split_at(pos);
        let rule = Rule {
            name: name.to_lowercase(),
            shell: shell[1..].trim().to_string(),
        };
        Ok(rule)
    }

    pub fn get_exe_rule(exe_name: &str) -> Self {
        let name = if exe_name.ends_with('f') {
            format!("{}ile", exe_name)
        } else {
            format!("{}file", exe_name)
        };
        Rule {
            name,
            shell: "$file $@".into(),
        }
    }

    pub fn is_match_file(&self, file: &Path) -> bool {
        file.file_name()
            .and_then(|v| v.to_str())
            .map(|v| v.to_lowercase().starts_with(&self.name))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
struct State {
    current_dir: String,
    file: String,
    file_dir: String,
}

impl State {
    pub fn new(current_dir: &Path, file: &Path) -> Self {
        let current_dir = stringify_path(current_dir);
        let file_dir = stringify_path(file.parent().unwrap());
        let file = stringify_path(file);
        Self {
            current_dir,
            file,
            file_dir,
        }
    }
    pub fn build_args(&self, rule: &Rule, args: &[String]) -> Result<Vec<String>> {
        let mut output = Vec::new();
        let mut shell = rule.shell.to_string();
        shell = shell.replace("$currentDir", &self.current_dir);
        shell = shell.replace("$file", &self.file);
        shell = shell.replace("$fileDir", &self.file_dir);
        let words = shell_words::split(&shell).map_err(|e| anyhow!("{}", e))?;
        for part in words {
            let part = part.trim();
            if part == "$@" {
                output.extend(args.iter().cloned())
            } else {
                output.push(part.to_string());
            }
        }
        Ok(output)
    }
    pub fn env_vars(&self) -> HashMap<String, String> {
        let mut output: HashMap<String, String> = Default::default();
        output.insert("XF_CURRENT_DIR".into(), self.current_dir.clone());
        output.insert("XF_FILE".into(), self.file.clone());
        output.insert("XF_FILE_DIR".into(), self.file_dir.clone());
        output
    }
}

fn stringify_path(path: &Path) -> String {
    path.to_string_lossy().escape_debug().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rule {

        use super::*;

        macro_rules! assert_rule {
            ($rule:literal, $name:literal, $shell:literal) => {
                let rule = Rule::create($rule).unwrap();
                assert_eq!(rule.name.as_str(), $name);
                assert_eq!(rule.shell.as_str(), $shell);
            };
        }

        #[test]
        fn test_create() {
            assert_rule!("Taskfile:$file $@", "taskfile", "$file $@");
            assert_rule!(
                "Taskfile:bash -c \"cd $currentDir; bash $file $@\"",
                "taskfile",
                "bash -c \"cd $currentDir; bash $file $@\""
            );
        }

        #[test]
        fn test_match_file() {
            let rule = Rule::create("Taskfile:$file $@").unwrap();
            let path: PathBuf = "/tmp/taskfile".parse().unwrap();
            assert!(rule.is_match_file(&path));
            let path: PathBuf = "/tmp/taskfile.sh".parse().unwrap();
            assert!(rule.is_match_file(&path));
            let path: PathBuf = "/tmp/Taskfile".parse().unwrap();
            assert!(rule.is_match_file(&path));
            let path: PathBuf = "/tmp/TaskFile".parse().unwrap();
            assert!(rule.is_match_file(&path));
        }
    }

    mod state {
        use super::*;

        macro_rules! new_state {
            ($workdir:expr, $file:expr) => {{
                let current_dir: PathBuf = $workdir.parse().unwrap();
                let file: PathBuf = $file.parse().unwrap();
                State::new(&current_dir, &file)
            }};
        }
        macro_rules! assert_build_args {
            (
                $rule:expr,
                $current_dir:expr,
                $file:expr,
                $args:expr,
                $expect_args:expr
            ) => {
                let rule = Rule::create($rule).unwrap();
                let state = new_state!($current_dir, $file);
                let args: Vec<String> = $args.iter().map(|v: &&str| v.to_string()).collect();
                let expect_args: Vec<String> = $expect_args.iter().map(|v| v.to_string()).collect();
                let build_args = state.build_args(&rule, &args).unwrap();
                assert_eq!(build_args, expect_args);
            };
        }

        #[test]
        fn test_buid_args() {
            assert_build_args!(
                "Taskfile:$file $@",
                "/tmp/test",
                "/tmp/test/Taskfile",
                &["test", "ci"],
                &["/tmp/test/Taskfile", "test", "ci"]
            );
            assert_build_args!(
                "Taskfile:bash -c \"echo $fileDir $currentDir\"",
                "/tmp/test",
                "/tmp/test/Taskfile",
                &[],
                &["bash", "-c", "echo /tmp/test/TaskfileDir /tmp/test"]
            );
        }
    }
}
