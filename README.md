# xf

Find a file in the current directory and upwards, then execute it.


## Usage

First, you need to define rules that tell `xf` what files to look for and how to execute them.

Rules are specified through configuration file which is specified by the `XF_CONFIG_PATH` environment variable.

Each line in the configuration file is a rule, and its format is:

```
<file>:<command>
```

Configure the following rules:

```
Somefile:bash $file $@
Makefile:make $@
```

Task automatically inserts a built-in rule at the end

```
Xfile:$file $@
```

execute `xf foo`

Look for a Xfile in the current directory, and if found, execute `bash Xfile foo` .

If not found, continue to look for the Makefile in the current directory, if found, execute `make foo`.

If not found, continue to look for Xfile in the current directory, if found, execute `Xfile foo`.

If not found, enter the upper directory to continue this process.
 
**Ignore suffix and case** when matching files.

## Variables


The following built-in variables can be used in the config.

- `$@` - pass-through command line parameters
- `$file` - file path
- `$fileDir` - file directory, process's cwd will be set to this value
- `$currentDir` - the current directory

These variables(exclude `$@`) are also synced to environment variables:

- `$file` => `XF_FILE`
- `$fileDir` => `XF_FILE_DIR`
- `$currentDir` => `XF_CURRENT_DIR`

## Exe Name

Actually, the exe name affect builtin-rule and environment variable prefix.

If you rename executable file `xf`  to `task`, The built-in rule will be:

```
Taskfile:$file $@
```

The environment variable `XF_CONFIG_PATH` will be `TASK_CONFIG_PATH`.

The environment variable for `$file` will be `TASK_FILE`。