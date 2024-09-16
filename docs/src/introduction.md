
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

## Features

- Create a new directory and change into it with a single command.
- Create a new directory in the system temporary directory and change into it
  with the `mct` command.
- Shell integration for bash, zsh, and fish shells.
- Supports automatic installation of shell integration scripts.
