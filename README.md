# xf

[![CI](https://github.com/sigoden/xf/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/xf/actions/workflows/ci.yaml)
[![Crates](https://img.shields.io/crates/v/xf.svg)](https://crates.io/crates/xf)

File-aware dynamic command runner.

Xf try to find a file from the current directory and upwards, and execute different command according to the different file found.

 ## Install

### With cargo

```
cargo install xf
```

### Binaries on macOS, Linux, Windows

Download from [Github Releases](https://github.com/sigoden/xf/releases), unzip and add xf to your $PATH.
## Usage

Xf loads rules from configuration file.

> The default path of configuration file is `$HOME/.xf`, which can be specified with the `XF_CONFIG_PATH` environment variable.

Rule format is:

```
<file>: <command>
```

`<file>` tell `xf` what file to find, `<command>` tell `xf` what command to execute if found.

> `xf` has a built-in lowest priority rule: `Xfile: $file $@`


Configure the following rules:

```
Taskfile: bash $file $@
```

Run `xf foo`.

`xf` try to find for `Taskfile` file in the current directory, and if found, execute `bash $file foo` .

If not found, continue finding for `Xfile` file in the current directory, if found, execute `Xfile foo` (built-in rule).

If not found, enter the parent directory to continue this process.
 
File matching rules:

1. Ignore case. `Xfile` can match files `xfile`, `xFile`.

2. Find the filename that contains the rule filename. `Xfile` can match the files `Xfile.sh`, `Xfile.cmd`.

## Variables

The following built-in variables can be used in the command part of rule.

- `$@` - pass-through command line parameters
- `$file` - file path
- `$fileDir` - file directory, process's cwd will be set to this value
- `$currentDir` - the current directory

These variables(exclude `$@`) are also synced to environment variables:

- `$file` => `XF_FILE`
- `$fileDir` => `XF_FILE_DIR`
- `$currentDir` => `XF_CURRENT_DIR`

## Command Name

Actually, the command name affect builtin-rule and environment variable prefix.

If you rename executable file `xf`  to `task`:

1. The built-in rule will be `Taskfile: $file $@`

2. The default configuration file path will be `$HOME/.task`.

3. The environment variable `XF_CONFIG_PATH` will be `TASK_CONFIG_PATH`.

4. The environment variable for `$file` will be `TASK_FILE`。

## License

Copyright (c) 2022 xf-developers.

argc is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.