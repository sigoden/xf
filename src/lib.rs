use anyhow::{anyhow, bail, Result};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub struct Task {
    rules: Vec<Rule>,
}

impl Task {
    pub fn create(confix_content: Option<String>) -> Result<Self> {
        let task = Self {
            rules: Self::load_rules(confix_content)?,
        };
        Ok(task)
    }

    pub fn run(&self, cwd: &PathBuf, args: &[String]) -> Result<ExitStatus> {
        let (rule, folder, file) = self
            .search_folder_recursive(cwd)
            .ok_or(anyhow!("Not found target file"))?;
        let state = State::new(cwd, &file, &folder);
        let env_vars = state.env_vars();
        let args = state.build_args(rule, args);
        let mut command = Command::new(&args[0]);
        command.args(&args[1..]);
        command.current_dir(cwd);
        command.envs(&env_vars);
        command.status().map_err(|e| anyhow!("Run file {} thorw {}", &state.file, e))
    }

    fn load_rules(config_content: Option<String>) -> Result<Vec<Rule>> {
        match config_content {
            None => Ok(vec![Rule::get_defualt_rule()]),
            Some(text) => {
                let mut rules = Vec::new();
                for (idx, line) in text.lines().enumerate() {
                    let rule = Rule::create(line)
                        .map_err(|_| anyhow!("Config file has invalid rule at line {}", idx + 1))?;
                    rules.push(rule);
                }
                rules.push(Rule::get_defualt_rule());
                Ok(rules)
            }
        }
    }

    fn search_folder_recursive(&self, folder: &PathBuf) -> Option<(&Rule, PathBuf, PathBuf)> {
        let mut folder = folder.clone();
        loop {
            if let Some((rule, file)) = self.search_folder(&folder) {
                return Some((rule, folder, file));
            }
            folder = folder.parent()?.to_path_buf();
        }
    }

    fn search_folder(&self, folder: &PathBuf) -> Option<(&Rule, PathBuf)> {
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

    fn match_rule(&self, file: &PathBuf) -> Option<&Rule> {
        self.rules.iter().find(|v| v.is_match_file(file))
    }
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    cmd_parts: Vec<String>,
}

impl Rule {
    pub fn create(text: &str) -> Result<Self> {
        let pos = text.find(':').ok_or_else(|| anyhow!("Invalid rule"))?;
        let (name, cmd) = text.split_at(pos);
        let cmd_parts = Self::to_cmd_parts(cmd)?;
        if cmd_parts.is_empty() {
            bail!("No cmd");
        }
        let rule = Rule {
            name: name.to_lowercase().to_string(),
            cmd_parts,
        };
        Ok(rule)
    }

    pub fn get_defualt_rule() -> Self {
        Rule {
            name: "Taskfile".into(),
            cmd_parts: vec!["$file".into(), "$@".into()],
        }
    }

    pub fn is_match_file(&self, file: &PathBuf) -> bool {
        file.file_name()
            .and_then(|v| v.to_str())
            .map(|v| v.starts_with(&self.name))
            .unwrap_or_default()
    }

    fn to_cmd_parts(cmd: &str) -> Result<Vec<String>> {
        shell_words::split(cmd).map_err(|e| anyhow!("{}", e))
    }
}

#[derive(Debug, Clone)]
struct State {
    cwd: String,
    file: String,
    file_folder: String,
}

impl State {
    pub fn new(cwd: &PathBuf, file: &PathBuf, file_folder: &PathBuf) -> Self {
        let cwd = cwd.to_string_lossy().to_string();
        let file = file.to_string_lossy().to_string();
        let file_folder = file_folder.to_string_lossy().to_string();
        Self {
            cwd,
            file,
            file_folder,
        }
    }
    pub fn build_args(&self, rule: &Rule, args: &[String]) -> Vec<String> {
        let mut output = Vec::new();
        for part in &rule.cmd_parts {
            let part = part.trim();
            if part == "$cwd" {
                output.push(self.cwd.clone());
            } else if part == "$file" {
                output.push(self.file.clone());
            } else if part == "$fileFolder" {
                output.push(self.file_folder.clone());
            } else if part == "$@" {
                output.extend(args.iter().cloned())
            } else {
                output.push(part.to_string());
            }
        }
        output
    }
    pub fn env_vars(&self) -> HashMap<String, String> {
        let mut output: HashMap<String, String> = Default::default();
        output.insert("TASK_CWD".into(), self.cwd.clone());
        output.insert("TASK_FILE".into(), self.file.clone());
        output.insert("TASK_FILE_FOLDER".into(), self.file_folder.clone());
        output
    }
}
