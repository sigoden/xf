# xf

Find a file in the current directory and upwards, then execute it.


## Usage

First, you need to define rules that tell `xf` what files to look for and how to execute them.

We configure the rules in the following format:

```
<file>:<command>
```

> Configuration file path:
>   Linux: `$HOME/.config/xf/xf`
>   macOS: `$HOME/Library/Preferences/xf/xf`
>   Windows: `$env:USERPROFILE\AppData\Roaming`


Configure the following rules:

```
Somefile:bash $file $@
Makefile:make $@
```

Task automatically inserts a built-in rule at the end

```
Taskfile:$file $@
```

execute `xf foo`

Look for a Taskfile in the current directory, and if found, execute `bash Somefile foo` .

If not found, continue to look for the Makefile in the current directory, if found, execute `make foo`.

If not found, continue to look for Taskfile in the current directory, if found, execute `Taskfile foo`.

If not found, enter the upper directory to continue this process.
 
**Ignore suffix and case** when matching files.

## Variables


The following built-in variables can be used in the config.

- `$@` - pass-through command line parameters
- `$file` - file path
- `$fileDir` - file directory, process's cwd will be set to this value
- `$currentDir` - the current directory

These variables(exclude `$@`) are also synced to environment variables. Such as `$file` => `$XF_FILE`