
# Usage

After installing `mchdir`, use the following commands:

## `mcd` Command

- Create a new directory and change into it:

  ```bash
  mcd my_new_directory
  ```

- Change to the home directory (when no argument is provided):

  ```bash
  mcd
  ```

- Display help for the `mcd` command:

  ```bash
  mcd --help
  ```

## `mct` Command

- Create a new directory in the system temporary directory and change into it:

  ```bash
  mct my_temp_directory
  ```

- Create a truly random directory in the system temporary directory and change
  into it:

  ```bash
  mct
  ```

- Display help for the `mct` command:

  ```bash
  mct --help
  ```

## `mcl` Command

- Changes to the previous directory the shell was in:

  ```terminal
  mcl
  ```

- Display help for the `mcl` command:

  ```terminal
  mcl --help
  ```

## `mchdir` Command

The `mchdir` command supports the following subcommands:

- Display the shell integration code:

  ```bash
  mchdir init
  ```

- Install the shell integration automatically:

  ```bash
  mchdir install
  ```
