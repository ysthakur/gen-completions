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

Ported from [Fish's completions script](https://github.com/fish-shell/fish-shell/blob/master/share/tools/create_manpage_completions.py)

For examples of the kinds of files this generates, look at the [`expected`](./tests/resources/expected/) folder inside the [`tests`](./tests) folder.

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
  - Untested because I have no idea how to install packages that are flakes
- Build it yourself:
  - Download this repository (`git clone git@github.com:ysthakur/man-completions.git`)
  - `cd man-completions && cargo build --release`

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

Things to do:

- Port darwin and degroff parsers
- Find samples of type 4, Darwin, and Degroff to test
- Ensure nested subcommands and multiple subcommands work
- Add .gz files to the tests, test excluding/including commands and directories
- Figure out why fish only seems to use man1, man6, and man8
