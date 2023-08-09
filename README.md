# man-completions

[![Tests](https://github.com/ysthakur/man-completions/actions/workflows/test.yml/badge.svg)](https://github.com/ysthakur/man-completions/actions)
[![Latest version](https://img.shields.io/crates/v/man-completions.svg)](https://crates.io/crates/man-completions)
[![License](https://img.shields.io/crates/l/man-completions.svg)](./LICENSE.md)

This is an unfinished project to parse manpages to get completions for Zsh, and other shells.
Also generates JSON files, in case your shell isn't supported, so you can process
it and generate completions yourself.

Detects subcommands (very basic): If a manpage is named `foo-bar`, that's detect
as the subcommand `foo bar`.

Ported from [Fish's completions script](https://github.com/fish-shell/fish-shell/blob/master/share/tools/create_manpage_completions.py)

Things to do:

- Remove `.sp`
- Port type 3, type 4, darwin, scdoc, and degroff parsers
- Don't generate unnecessary files in tests
- Allow configuring tests more, test .gz, test excluding/including commands, directories, sections

The CLI uses [`env_logger`](https://docs.rs/env_logger/) as the backend for logging,
so to configure that, set the `RUST_LOG` environment variable (the link has instructions).
