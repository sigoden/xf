# Task

Find a file in the current directory and upwards, then eXecute it.


## Variables


The following built-in variables can be used in the config.

- `$@` - pass-through command line arguments
- `$cwd` - the current directory when executing the command
- `$file` - file path,
- `$fileFolder` - the path to the directory where the file is located

These variables(exclude `$@`) are also synced to environment variables. Such as `$file` => `$TASK_FILE`