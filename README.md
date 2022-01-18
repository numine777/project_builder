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
rusco <project_name> // this will init the json configuration for the project NOTE: this runs "build" by default once the initial json is created
rusco <project_name> -s set_command <command name> // this needs to be a single string
unbroken by whitespace (e.g. 'bazel' or 'docker-compose')
rusco <project_name> -s set_dir <absolute path the command needs to run in>
rusco <project_name> -s set_args <all args that should follow the command>
rusco <project_name> -s clear_args <clears all args>
rusco <project_name> -s set_flags <optional flags that should always follow args>
rusco <project_name> -s clear_flags <clears all flags>
```

