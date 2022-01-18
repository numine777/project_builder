# Rust COMMANDER

Originally built to be a project build aliaser, this project has turned into a convenient way to
alias any complex command to a smaller call to a command line tool that keeps track of a collection
of json configuration that can be updated through the command line or in your favorite editor. This
also has the benefit of being able to be run from anywhere in your file system that has access to
the command.

To install the program, simply run:
`cargo install rusco`

Current usage examples:

```
rusco init <project_name>
rusco list // this will list configuratios for commands
rusco set_command <project_name> // this needs to be a single string unbroken by whitespace (e.g. 'bazel' or 'docker-compose')
rusco set_dir <project_name>
rusco set_args <project_name>
rusco clear_args <project_name>
rusco set_flags <project_name>
rusco clear_flags <project_name>
rusco run <project_name>
```

