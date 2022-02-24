# Task

Find a file in the current directory and upwards, then eXecute it.


## Usage

First, you need to define rules that tell `task` what files to look for and how to execute them.

We configure the rules in the following format:

```
<file>:<command>
```

> Configuration file path, Linux: `$HOME/.config/task/task`, macOS: `$HOME/Library/Preferences/task/task`


Configure the following rules:

```
Somefile:bash $file $@
Makefile:make $@
```

Task automatically inserts a built-in rule at the end

```
Taskfile:$file $@
```

execute `task foo`

Look for a Taskfile in the current directory, and if found, execute `bash Somefile foo` .

If not found, continue to look for the Makefile in the current directory, if found, execute `make foo`.

If not found, continue to look for Taskfile in the current directory, if found, execute `Taskfile foo`.

If not found, enter the upper directory to continue this process.
 
**Ignore suffix and case** when matching files.

## Variables


The following built-in variables can be used in the config.

- `$@` - pass-through command line arguments
- `$cwd` - the current directory when executing the command
- `$file` - file path,
- `$fileFolder` - the path to the directory where the file is located

These variables(exclude `$@`) are also synced to environment variables. Such as `$file` => `$TASK_FILE`