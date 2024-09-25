
# Shell Integration

To enable the `mcd`, `mcl` and `mct` commands in your shell, you need to
integrate `mchdir` with your shell configuration.

## Automatic Installation

You can automatically install the shell integration by running:

```bash
mchdir install
```

This command will detect your shell and append the necessary integration code to
the bottom of your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`, or
`~/.config/fish/config.fish`). If it detects that the integration code is
already present, it will not add it again.

## Manually Add the Integration Code

If automatic installation does not work for your shell, you can manually add the
integration code to your shell configuration file.

### Bash, Zsh (and other Bash-Like Shells)

Add the following code to your shell's configuration file:

```bash
eval "$(mchdir init)"
```

### Fish Shell

Add the following code to your `~/.config/fish/config.fish` file:

```bash
eval (mchdir init)
```

> Note that after shell integration, you should no longer need to run the
> `mchdir` command again, unless you need to reinstall the shell integration. It
> DOES need to remain in your PATH for the shell integration to work however.
