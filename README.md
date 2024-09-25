# mchdir <!-- omit in toc -->

A utility for creating a new folder then changing into it in one command,
changing to the last remembered directory or creating and changing into a
temporary folder.

Written in Rust and using shell integration.

- [Description](#description)
  - [Why?](#why)
  - [Why the need for shell integration too?](#why-the-need-for-shell-integration-too)
- [Features](#features)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Install from Crates.io](#install-from-cratesio)
  - [Build from Source](#build-from-source)
  - [Install from GitHub](#install-from-github)
- [Shell Integration](#shell-integration)
  - [Automatic Installation](#automatic-installation)
  - [Manually Add the Integration Code to Your Shell Configuration](#manually-add-the-integration-code-to-your-shell-configuration)
    - [Bash, Zsh (and other Bash-Like Shells)](#bash-zsh-and-other-bash-like-shells)
    - [Fish Shell](#fish-shell)
  - [Fully Manual Installation](#fully-manual-installation)
- [Usage](#usage)
  - [`mcd` Command](#mcd-command)
  - [`mct` Command](#mct-command)
  - [`mcl` Command](#mcl-command)
  - [mchdir Command](#mchdir-command)
- [License](#license)
- [Contributing](#contributing)

## Description

`mchdir` is a command-line tool written in Rust that allows you to create a new
directory and immediately change into it. It includes shell integration that
defines an a couple of helpfull commands in your shell, simplifying directory
creation and navigation.

Documentation is available at
[https://seapagan.github.io/mchdir/](https://seapagan.github.io/mchdir/).

### Why?

I was getting tired of creating a new directory and then changing into it with
two separate commands. I wanted a single command that could do both. I wrote a
utility to do this decades ago on Windows, and I wanted the same functionality
on Linux/macOS.

It's true that you can do the same thing with a simple shell function or alias!
However, this project is a demonstration of how to create a command-line tool in
Rust that provides (and installs!) shell integration. It will also probably get
more features in the future.

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
- Change back to the last directory with the `mcl` command.
- Shell integration for bash, zsh, and fish shells as first-class citizens,
  though it should work in most POSIX-compliant shells too.
- Supports automatic installation of shell integration scripts.

## Installation

In the future, there will be pre-built binaries available for download. For now,
you can install `mchdir` from crates.io, build it from source, or install it
directly from GitHub.

### Prerequisites

- Rust programming language (for building from source)
- cargo package manager
- Linux or macOS (**Windows is not supported**)

### Install from Crates.io

You can install the `mchdir` command from the crates.io registry using the
following command:

```terminal
cargo install mchdir
```

### Build from Source

Clone the repository:

```terminal
git clone https://github.com/seapagan/mchdir.git
cd mchdir
```

Build the project:

```terminal
cargo build --release
```

This will produce an executable in the target/release directory.

Install the Executable:

Copy the executable to a directory in your $PATH, for example:

```terminal
sudo cp target/release/mchdir /usr/local/bin/
```

### Install from GitHub

You can also install the `mchdir` command directly from GitHub using the following
command:

```terminal
cargo install --git https://github.com/seapagan/mchdir.git
```

## Shell Integration

To enable the mcd and mct commands in your shell, you need to integrate `mchdir` with
your shell configuration.

### Automatic Installation

You can automatically install the shell integration by running:

```terminal
mchdir install
```

This command will detect your shell and append the necessary integration code to
the bottom of your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`, or
`~/.config/fish/config.fish`).

**Note**: Currently, automatic installation supports bash, zsh, and fish shells.
For other shells, please follow the manual installation instructions. The shell
integration code is POSIX-compliant and should work in most shells, or only need
minor modifications.

After running the installation command, restart your shell or source your
configuration file to apply the changes:

```terminal
# For bash and zsh
source ~/.bashrc  # or source ~/.zshrc

# For fish
source ~/.config/fish/config.fish
```

### Manually Add the Integration Code to Your Shell Configuration

Instead of using the automatic installation, you can manually add the shell
integration code to your shell configuration file. Below is an example of how to
do this for different shells.

#### Bash, Zsh (and other Bash-Like Shells)

For `bash`, add the following code to your `~/.bashrc` file:

```terminal
eval "$(mchdir init)"
```

For `zsh`, add the following code to your `~/.zshrc` file:

```terminal
eval "$(mchdir init)"
```

#### Fish Shell

For the `fish` shell, add the following code to your
`~/.config/fish/config.fish` file:

```terminal
eval (mchdir init)
```

After adding this command to your shell configuration command, restart your
shell or source your configuration file to apply the changes.

### Fully Manual Installation

If you prefer manual installation or your shell is not supported, you can
generate the shell integration code and add it to your shell configuration
manually.

Generate the integration code:

```terminal
mchdir init
```

This command will output the shell function definition for mcd and mct. Copy the output
and paste it into your shell's configuration file.

After pasting this code into your shell configuration command, restart your
shell or source your configuration file to apply the changes.

## Usage

Note that after installation, you no longer need to run the `mchdir` command
again, unless you need to reinstall the shell integration. It DOES need to
remain in your PATH for the shell integration to work however.

### `mcd` Command

- Create a new directory and change into it:

  ```terminal
  mcd my_new_directory
  ```

- Change to the home directory (when no argument is provided):

  ```terminal
  mcd
  ```

- Display help for the mcd command:

  ```terminal
  mcd --help
  ```

Output:

```terminal
Usage: mcd <directory>
Creates a new directory and changes into it.
If no directory is specified, changes to the home directory.
```

### `mct` Command

- Create a new directory in the system temporary directory and change into it:

  ```terminal
  mct my_temp_directory
  ```

- Create a truly random directory in the system temporary directory and change into it (when no argument is provided):

  ```terminal
  mct
  ```

- Display help for the `mct` command:

  ```terminal
  mct --help
  ```

Output:

```terminal
Usage: mct <directory>
    Creates a new directory in the system temporary directory and changes into it.
    If no directory is specified, creates a random directory in the temp folder.
```

### `mcl` Command

- Changes to the previous directory the shell was in:

  ```terminal
  mcl
  ```

- Display help for the `mcl` command:

  ```terminal
  mcl --help
  ```

Output:

```terminal
Usage: mcl
  Changes to the previous directory the shell was in.
```

### mchdir Command

The `mchdir` command supports the following subcommands:

- Display the shell integration code:

  ```terminal
  mchdir init
  ```

- Install the shell integration automatically:

  ```terminal
  mchdir install
  ```

Again, you only need to run the `mchdir` command when initially installing the
shell integration or when you want to reinstall it.

## License

```pre
The MIT License (MIT)
Copyright (c) 2024 Grant Ramsay

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
OR OTHER DEALINGS IN THE SOFTWARE.
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on
GitHub.
