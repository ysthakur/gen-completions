# man-completions

[![Tests](https://github.com/ysthakur/man-completions/actions/workflows/test.yml/badge.svg)](https://github.com/ysthakur/man-completions/actions)
[![Lint](https://github.com/ysthakur/man-completions/actions/workflows/lint.yml/badge.svg)](https://github.com/ysthakur/man-completions/actions)
[![Latest version](https://img.shields.io/crates/v/man-completions.svg)](https://crates.io/crates/man-completions)
[![License](https://img.shields.io/crates/l/man-completions.svg)](./LICENSE.md)

This is a crate for parsing manpages to get completions for Zsh, Bash, Nushell,
and, in the future, other shells.

It also generates JSON files, in case your shell isn't supported, so you can process
it and generate completions yourself.

Currently, only a couple kinds of manpages are supported.

- [Installation](#installation)
- [Usage](#usage)
  - [Flags](#flags)
  - [Zsh](#zsh)
  - [Bash](#bash)
  - [Nushell](#nushell)
- [Contributing](#contributing)

Ported from [Fish's completions script](https://github.com/fish-shell/fish-shell/blob/master/share/tools/create_manpage_completions.py)

For examples of the kinds of files this generates, look at the [`expected`](./tests/resources/expected/) folder inside the [`tests`](./tests) folder.

For some example man pages, look at the [`samples`](/samples/) folder.

Detects subcommands (very basic): If a manpage is named `git-commit-tree`, it will
look for the text `git commit tree`, `git-commit tree`, and `git commit-tree` in
the file. When it finds the text `git commit-tree` in the man page, it will
assume that `commit-tree` is a subcommand of `git`. I'm not sure how the Fish
script generates subcommands--I've been too lazy to do anything but skim over it--but
I will eventually get around to porting Fish's subcommand detection.

## Installation

- Using Cargo: `cargo install man-completions`
- From the [Releases](https://github.com/ysthakur/man-completions/releases) page:
  Simply download the right executable for your platform from the latest release
- As a Nix flake: `github:ysthakur/man-completions`
  - Try it out with `nix shell github:ysthakur/man-completions`
- Build it yourself:
  - Download this repository (`git clone git@github.com:ysthakur/man-completions.git`)
  - `cd man-completions && cargo build --release`

## Usage

You can periodically run `man-completions` to generate completions for any commands you want.

For example, if you have a directory `~/generated-completions` for all your generated
completions, and you want to generate Zsh completions for `ncdu`, you can use:

```shell
man-completions -o ~/generated-completions -s zsh --cmds="ncdu" # For Bash, use -s bash
```

## Flags

| Short form | Long form | Description |
|-|-|-|
| `-o` | `--out` | Directory to output files to (required) |
| `-s` | `--shells` | Shells to generate completions for: `zsh`, `bash`, `nu`, or `json` (required) |
| `-d` | `--dirs` | Directories to search in (comma-separated) |
| `-c` | `--cmds` | Regex to search for only specific commands |
| `-C` | `--exclude-cmds` | Regex to exclude certain commands |
| `-n` | `--not-subcmds` | Commands that are not to be treated as subcommands (comma-separated) |
| | `--subcmds` | Explicitly list subcommands that may not be detected, e.g. `foobar=foo bar,git-commit=git commit` |
| `-h` | `--help` | Show help information |

To search for man pages in a specific set of directories, set `$MANPATH` explicitly.
You can also use `--dirs`, but note that `--dirs` will search directly inside the
given directories, not inside `<dir>/man1`, `<dir>/man2`, etc.

The CLI uses [`env_logger`](https://docs.rs/env_logger/) as the backend for logging,
so to configure that, set the `RUST_LOG` environment variable (the link has instructions).

### Zsh

You can either generate completions to a directory that's already in `$fpath`, where
Zsh looks for functions, or you can make a new directory. If you choose to do the latter,
make sure to add it to your `$fpath` in your `~/.zshrc` (**before** you call `compinit`)

```zsh
fpath=(path/to/my/directory $fpath)
```

### Bash

TODO

### Nushell

TODO

## Contributing

Any contributions are welcome.

Here are some things that need work:

- Port darwin and degroff parsers
- Test the type 4 parser
- Find samples of type 4, Darwin, and Degroff to test
- Add .gz files to the tests folder
- Test excluding/including commands and directories
- Figure out why fish only seems to use man1, man6, and man8
- Handle options like `-vv` and `-/` in Nushell
