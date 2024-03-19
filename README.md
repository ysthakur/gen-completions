# gen-completions

[![Tests](https://github.com/ysthakur/gen-completions/actions/workflows/test.yml/badge.svg)](https://github.com/ysthakur/gen-completions/actions)
[![Lint](https://github.com/ysthakur/gen-completions/actions/workflows/lint.yml/badge.svg)](https://github.com/ysthakur/gen-completions/actions)
[![Latest version](https://img.shields.io/crates/v/gen-completions.svg)](https://crates.io/crates/gen-completions)
[![License](https://img.shields.io/crates/l/gen-completions.svg)](./LICENSE.md)

> [!warning]
> This project is a work in progress so it's extremely unstable and mostly broken.

This is a crate for parsing manpages to generate shell completions either by parsing
manpages or from KDL/JSON files. There's both a library and a binary, and if
you're looking for documentation on the library, see https://docs.rs/gen-completions/.
But you're probably here for the binary, and if you want information on that, read on.

Currently, it generates Bash, Zsh, and Nushell completions, although I've only
tested out Zsh and Nushell properly. If you're using another shell, it also generates
[Carapace](https://github.com/rsteube/carapace-bin) specs. In addition to that,
it generates KDL and JSON files so you can process the command information
to generate completions yourself or something else.

The manpage parsing has been mainly ported from [Fish's completions script](https://github.com/fish-shell/fish-shell/blob/master/share/tools/create_manpage_completions.py),
although this crate doesn't yet support every kind of manpage that the Fish script supports. In particular, MacOS man pages cannot yet be parsed. Any help with that would be greatly appreciated.

## Installation

- Using Cargo: `cargo install gen-completions`
- From the [Releases](https://github.com/ysthakur/gen-completions/releases) page:
  Simply download the right executable for your platform from the latest release
- As a Nix flake: `github:ysthakur/gen-completions`
  - Try it out with `nix shell github:ysthakur/gen-completions`
- Build it yourself:
  - Download this repository (`git clone git@github.com:ysthakur/gen-completions.git`)
  - `cd gen-completions && cargo build --release`

## Usage

You can periodically run `gen-completions` to generate completions for any commands you want.

For example, if you have a directory `~/generated-completions` for all your generated
completions, and you want to generate Zsh completions from the `ncdu` manpage, you can use:

```shell
gen-completions man zsh ~/generated-completions --cmds="ncdu"
```

If you have a config file to generate completions from, you can use:

```shell
gen-completions for zsh ncdu-completions.kdl ~/generated-completions
```

The CLI uses [`env_logger`](https://docs.rs/env_logger/) as the backend for logging,
so to configure that, set the `RUST_LOG` environment variable (the link has instructions).

See below for specific flags and whatnot.

### Generating from manpages

```
Usage: gen-completions man [OPTIONS] <SHELL> <PATH>

Arguments:
  <SHELL>
          Shell(s) to generate completions for

          Possible values:
          - zsh:      Generate completions for Zsh
          - bash:     Generate completions for Bash
          - nu:       Generate completions for Nushell
          - kdl:      Output parsed options as KDL
          - json:     Output parsed options as JSON
          - carapace: Output Carapace spec

  <PATH>
          Directory to output completions to

Options:
  -d, --dirs <PATH,...>
          Directories to search for man pages in, e.g. `--dirs=/usr/share/man/man1,/usr/share/man/man6` Note that `--dirs` will search directly inside the given directories, not inside `<dir>/man1`, `<dir>/man2`, etc. If you want to search for man pages in a specific set of directories, set `$MANPATH` before running this command

  -c, --cmds <REGEX>
          Commands to generate completions for. If omitted, generates completions for all found commands. To match the whole name, use "^...$"

  -C, --exclude-cmds <REGEX>
          Commands to exclude (regex). To match the whole name, use "^...$"

      --not-subcmds <COMMAND-NAME,...>
          Commands that should not be treated as subcommands, to help deal with false positives when detecting subcommands

      --subcmds <man-page=sub cmd,...>
          Explicitly list which man pages are for which subcommands. e.g. `git-commit=git commit,foobar=foo bar`

  -h, --help
          Print help (see a summary with '-h')
```

### Generating from KDL/JSON/YAML

```
Usage: gen-completions for <SHELL> <CONF> [OUT]

Arguments:
  <SHELL>
          Shell(s) to generate completions for

          Possible values:
          - zsh:      Generate completions for Zsh
          - bash:     Generate completions for Bash
          - nu:       Generate completions for Nushell
          - kdl:      Output parsed options as KDL
          - json:     Output parsed options as JSON
          - carapace: Output Carapace spec

  <CONF>
          File to generate completions from

  [OUT]
          File to generate completions to. Outputted to stdout if not given

Options:
  -h, --help
          Print help (see a summary with '-h')
```

### Zsh

You can either generate completions to a directory that's already in `$fpath`, where
Zsh looks for functions, or you can make a new directory. If you choose to do the latter,
make sure to add it to your `$fpath` in your `~/.zshrc`:

```zsh
fpath=(path/to/my/directory $fpath)
```

> [!note]
> `fpath` must be updated **before** `compinit` is called.

After this, if your chosen directory is `~/generated-completions`, you can run

```zsh
gen-completions man zsh ~/generated-completions --cmds="^ncdu"
```

and when you try `ncdu <TAB>`, you should see completions for all of ncdu's flags.

### Bash

TODO

### Nushell

TODO

## Contributing

Any and all contributions are welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md) for information on what needs to be worked on.

### Details on how it works

For examples of the kinds of files this generates, look at the
[`expected`](./tests/resources/expected/) folder inside the [`tests`](./tests) folder.

For some example man pages, look at the [`samples`](/samples/) folder.

It has very basic subcommand detection. If a manpage is named `git-commit-tree`,
it will look for the text `git commit tree`, `git-commit tree`, and `git commit-tree` in
the file. When it finds the text `git commit-tree` in the man page, it will
assume that `commit-tree` is a subcommand of `git`. I'm not sure how the Fish
script generates subcommands--I've been too lazy to do anything but skim over it--but
I will eventually get around to porting Fish's subcommand detection.
