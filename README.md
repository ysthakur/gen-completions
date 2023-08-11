# man-completions

[![Tests](https://github.com/ysthakur/man-completions/actions/workflows/test.yml/badge.svg)](https://github.com/ysthakur/man-completions/actions)
[![Latest version](https://img.shields.io/crates/v/man-completions.svg)](https://crates.io/crates/man-completions)
[![License](https://img.shields.io/crates/l/man-completions.svg)](./LICENSE.md)

This is an unfinished project to parse manpages to get completions for Zsh, Bash,
Nushell, and other shells. Bash is still a work in progress.

It also generates JSON files, in case your shell isn't supported, so you can process
it and generate completions yourself.

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
- As a Nix flake: I have no idea
- Download the right executable for your platform from the latest [release](https://github.com/ysthakur/man-completions/releases)

## Flags

| Short form | Long form | Description |
|-|-|-|
| `-o` | `--out` | Directory to output files to (required) |
| `-s` | `--shells` | Shells to generate completions for: `zsh`, `bash`, `nu`, or `json` (required) |
| `-i` | `--ignore` | Directories to ignore when searching for man pages (comma-separated) |
| `-S` | `--sections-exclude` | Man sections to exclude (1-8) (comma-separated) |
| `-c` | `--cmds` | Regex to search for only specific commands |
| `-C` | `--exclude-cmds` | Regex to exclude certain commands |
| `-n` | `--not-subcmds` | Commands that are not to be treated as subcommands (comma-separated) |
| `-h` | `--help` | Show help information |

To search for man pages in a specific set of directories, set `$MANPATH` explicitly.

The CLI uses [`env_logger`](https://docs.rs/env_logger/) as the backend for logging,
so to configure that, set the `RUST_LOG` environment variable (the link has instructions).

Things to do:

- Generate Bash
- Port type 3, type 4, darwin, scdoc, and degroff parsers
- Don't generate unnecessary files in tests
- Allow configuring tests more, test .gz, test excluding/including commands, directories, sections
  - The `--shells` flag should really be `--shell` because outside of the tests,
    no one's going to generate completions for multiple shells at once in the same
    directory.
- Speed improvements - the integration tests currently take over a second to run,
  and it seems most of that time is spent in parsing
