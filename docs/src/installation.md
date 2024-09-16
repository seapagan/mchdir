
# Installation

To install `mchdir`, you can use crates.io, build it from source, or install it
directly from GitHub.

## Prerequisites

- Rust programming language (for building from source)
- `cargo` package manager
- Linux or macOS (**Windows is not supported**)

## Install from Crates.io

You can install the `mchdir` command from the crates.io registry using the
following command:

```bash
cargo install mchdir
```

## Build from Source

Clone the repository:

```bash
git clone https://github.com/seapagan/mchdir.git
cd mchdir
```

Build the project:

```bash
cargo build --release
```

This will produce an executable in the `target/release` directory.

Install the Executable:

Copy the executable to a directory in your `$PATH`, for example:

```bash
sudo cp target/release/mchdir /usr/local/bin/
```

## Install from GitHub

You can also install the `mchdir` command directly from GitHub using the
following command:

```bash
cargo install --git https://github.com/seapagan/mchdir.git
```

Once installed, you need to enable shell integration to use the `mcd` and `mct`
commands. See the [Shell Integration](shell_integration.md) documentation (next)
for more information.
