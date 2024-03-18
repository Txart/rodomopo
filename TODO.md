## Todos


Do not print "Creating application files at..." etc always!!

main.rs: rename and refactor if_first_time_set_up_files()

Bundle as standalone software
  - instructions to use it as a command in linux
Add subcommand to print and edit current config, maybe with Clap

Improve error handling. No unwrap() or expects()!

Improve the looks of the output.
  - colors and bolds

Let the user open the timestamps file to modify it by hand by one command line argument.

Read The Book chapter about writing CLI app:
  - write errors to stdout.

## Some day
- plot weekly, monthly, yearly stats.


## Done

Write a configuration file in .config/rodomopo/config.yaml with arguments for user.
Read config options (such as the global constants) from a config file.
Show current work status. Sum and print all timestamps done today.
Files in current directory?
  - Progress bar
Let user decide to go against the rules if duration is too short.
Error: if there are no timestamps for a day, the code panics because it cannot subtract anything!!
  - timestamps.dat and status.txt should be in a relative path somehow. Of course, timestamps.dat and status.txt cannot be part of the standalone binary, because they change! It should be written in the user's machine somewhere. But where?
