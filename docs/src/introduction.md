
# Introduction

`mchdir` is a command-line tool written in Rust that allows you to create a new
directory and immediately change into it. It includes shell integration that
defines an `mcd` command in your shell, simplifying directory creation and
navigation.

## Description

`mchdir` provides a convenient way to create a new directory and change into it
in a single command. While similar functionality can be achieved with a shell
script, this project serves as a demonstration of how to create a command-line
tool in Rust with shell integration. Additionally, `mchdir` includes the `mct`
command to create and enter directories in the system's temporary folder.

### Why?

I was getting tired of creating a new directory and then changing into it with
two separate commands. I wanted a single command that could do both. I wrote a
utility to do this decades ago on Windows, and I wanted the same functionality
on Linux/macOS.

It's true that you can do the same thing with a simple shell function or alias!
However, this project is a demonstration of how to create a command-line tool in
Rust that provides shell integration. It will also probably get more features in
the future.

### Why the need for shell integration too?

It is impossible to change the current working directory of the parent process
from a child process (i.e., a command). This is because each process has its own
working directory, and changes to the working directory are not propagated to
the parent process. Hence, the mcd command cannot change the working directory
of the shell that runs it without shell integration.

## Features

- Create a new directory and change into it with a single command.
- Create a new directory in the system temporary directory and change into it
  with the `mct` command.
- Shell integration for bash, zsh, and fish shells.
- Supports automatic installation of shell integration scripts.
