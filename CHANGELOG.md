# Changelog

## [0.6.0](https://github.com/ysthakur/gen-completions/compare/v0.5.1...v0.6.0) (2024-03-20)


### ⚠ BREAKING CHANGES

* generate carapace but remove yaml support

### Features

* Basic support for Darwin ([#17](https://github.com/ysthakur/gen-completions/issues/17)) ([4a849ca](https://github.com/ysthakur/gen-completions/commit/4a849ca88040547984a6386b0f35622e535759ef))
* generate carapace but remove yaml support ([c429a78](https://github.com/ysthakur/gen-completions/commit/c429a784ab742cd97ee060802fd5edc857b1f17b))
* Generate descriptions for strings arg type for nu ([c70d189](https://github.com/ysthakur/gen-completions/commit/c70d1891ecfe9e51895e016298c77d624bc20114))
* Make Nushell try generating types ([898537b](https://github.com/ysthakur/gen-completions/commit/898537bbc267007e2efb47848eea18140ad7e9e8))
* Parse arg types ([0417e65](https://github.com/ysthakur/gen-completions/commit/0417e65cf76200ab891a4fc19ea99f8771475cd0))


### Bug Fixes

* Don't include subsection headings at end ([590e695](https://github.com/ysthakur/gen-completions/commit/590e695090a6aae07adb4cbffce322aeff310dfb))
* Fix duplicate flag span ([e486366](https://github.com/ysthakur/gen-completions/commit/e4863660baecb998d2f046f4552ad6b0a4834d32))

## [0.5.1](https://github.com/ysthakur/man-completions/compare/v0.5.0...v0.5.1) (2023-12-25)


### Features

* Parse type (kdl) ([51303ba](https://github.com/ysthakur/man-completions/commit/51303ba11620b17df9a7e325df89ebb4aa221e18))

## [0.5.0](https://github.com/ysthakur/man-completions/compare/v0.4.1...v0.5.0) (2023-12-25)


### ⚠ BREAKING CHANGES

* Changes to KDL format

### Features

* Also generate completions from KDL/JSON/YAML ([#12](https://github.com/ysthakur/man-completions/issues/12)) ([1c36fd2](https://github.com/ysthakur/man-completions/commit/1c36fd2a32b266b35e40840bcc84ee1ff7b52a78))
* Changes to KDL format ([366d05a](https://github.com/ysthakur/man-completions/commit/366d05aa009ed9d433813479d0941da5ba0f5a5c))

## [0.4.1](https://github.com/ysthakur/man-completions/compare/v0.4.0...v0.4.1) (2023-12-24)


### Features

* trigger release ([e384d7e](https://github.com/ysthakur/man-completions/commit/e384d7e24dc4ddf4fdf3f2643a4aa88348a8d04a))

## [0.4.0](https://github.com/ysthakur/man-completions/compare/v0.3.2...v0.4.0) (2023-11-18)


### ⚠ BREAKING CHANGES

* <X>-completions.nu instead of <X>.nu

### Features

* &lt;X&gt;-completions.nu instead of <X>.nu ([6d776b3](https://github.com/ysthakur/man-completions/commit/6d776b3e3700e6243f7cb4da82beb03c5b3968f8))
* Support Pod::Man ([31abecf](https://github.com/ysthakur/man-completions/commit/31abecf639aadf47b184cc37945f90cbf35f096c))


### Bug Fixes

* call completion function in generated zsh ([35cf344](https://github.com/ysthakur/man-completions/commit/35cf3442d760f14dbfde42a1bafe24502b4155b3))
* Escape [] in zsh ([0b2ebcf](https://github.com/ysthakur/man-completions/commit/0b2ebcf28ed58a8037bbdb4930f1a69f754957a9))
* Remove print debugging from podman.rs ([43e2d94](https://github.com/ysthakur/man-completions/commit/43e2d945d65745fdc783e61ead010b3d0101b585))
* Turn '\ ' into ' ' ([fd38364](https://github.com/ysthakur/man-completions/commit/fd3836428900ef0f7f863a2aa8ac8a6cb49ebc88))

## [0.3.2](https://github.com/ysthakur/man-completions/compare/v0.3.1...v0.3.2) (2023-08-19)


### Bug Fixes

* Make multiple subcommands work ([72d1c20](https://github.com/ysthakur/man-completions/commit/72d1c20f24ebb88a03ff6efdb5f049670c818ea1))
* Nested subcommands ([7aa4801](https://github.com/ysthakur/man-completions/commit/7aa4801c83d06ac5285acdc15ed6758c709df481))

## [0.3.1](https://github.com/ysthakur/man-completions/compare/v0.3.0...v0.3.1) (2023-08-18)


### Features

* Add type4 parser (untested) ([0745256](https://github.com/ysthakur/man-completions/commit/074525613fa89597d9ae6ad9ee5b86b16e8e4ed1))
* Implement type3 ([03c8e2f](https://github.com/ysthakur/man-completions/commit/03c8e2fcf9c1f0c75e2d9dfa21340f05c938d7b8))
* scdoc parser ([857e11e](https://github.com/ysthakur/man-completions/commit/857e11ee31f6a734124cf063a9593f07187b6a7a))

## [0.3.0](https://github.com/ysthakur/man-completions/compare/v0.2.1...v0.3.0) (2023-08-17)


### ⚠ BREAKING CHANGES

* Rename Arg to Flag
* Move manpage-finding to main.rs

### Features

* Output (n parsed/total) statistic ([b89debf](https://github.com/ysthakur/man-completions/commit/b89debf1c077ee3ca95011c9f50e60242a31f6b3))
* Read .bz2 ([02b5b27](https://github.com/ysthakur/man-completions/commit/02b5b27337e6c97ae6cd528b01071d47bd2ee9c0))
* Untested Bash impl ([1f43bd4](https://github.com/ysthakur/man-completions/commit/1f43bd47c87fb1496176db9c40a53ad11785be43))


### Code Refactoring

* Move manpage-finding to main.rs ([d5a57cc](https://github.com/ysthakur/man-completions/commit/d5a57cc1c59d30efc52c500bf297f546e89a1b7e))
* Rename Arg to Flag ([ec2dcce](https://github.com/ysthakur/man-completions/commit/ec2dcce0c57e3ac52883f2724f997b35859fa4b2))

## [0.2.1](https://github.com/ysthakur/man-completions/compare/v0.2.0...v0.2.1) (2023-08-11)


### Features

* Detect subcommands with hyphens ([d89b021](https://github.com/ysthakur/man-completions/commit/d89b0212fcf58794bf0584f55a59e84db9b29d6e))
* Explicitly give subcommands ([7146648](https://github.com/ysthakur/man-completions/commit/714664835f299d7e86589cd5d009fd816345f9ea))
* Nushell support ([00bb571](https://github.com/ysthakur/man-completions/commit/00bb571955444876eb378e0076ce4b2ca09ecf78))

## [0.2.0](https://github.com/ysthakur/man-completions/compare/v0.1.0...v0.2.0) (2023-08-09)


### Features

* Add env_logger backend ([9c9633c](https://github.com/ysthakur/man-completions/commit/9c9633ce450cdba09af39110ab1b1d876669beba))
* Add option to generate JSON ([204f4ad](https://github.com/ysthakur/man-completions/commit/204f4ad8a4547b3be43e1724593f654f75fd8a26))
* Allow descriptions to be empty ([51175d1](https://github.com/ysthakur/man-completions/commit/51175d13180ab6eacc62be1fd9fb128bd854e5a2))
* Allow marking commands as not subcommands ([89aaed1](https://github.com/ysthakur/man-completions/commit/89aaed1d73fca9ef5e4d3b80690f1171a58a18a8))
* Detect and merge subcommands ([33c8ed5](https://github.com/ysthakur/man-completions/commit/33c8ed54a11e0ce099cad0d9eca6189162da436f))
* Implement type 2 parser ([cef1e21](https://github.com/ysthakur/man-completions/commit/cef1e21546e837df7467985a493ef50504b3aaff))
* include or exclude multiple commands ([77471e4](https://github.com/ysthakur/man-completions/commit/77471e4c151d83fa14131e99e8ec8fe9234d0192))
* Make JSON output subcommands ([85887da](https://github.com/ysthakur/man-completions/commit/85887da3434a9ca9ea40506653281ee71786e1ae))
* Simpler interface to parser ([3bf1f8d](https://github.com/ysthakur/man-completions/commit/3bf1f8d5f322cdd1fa1e72b8d2d20252a291e4a2))


### Bug Fixes

* conflicting option names ([7bb9b0f](https://github.com/ysthakur/man-completions/commit/7bb9b0f019eb46cc0fccbf0ba18a20648bd3d452))
* fix Zsh subcommand comp functions ([245414b](https://github.com/ysthakur/man-completions/commit/245414b06bbe4230d9ef391ba25e37b1c8779a91))
* Make order of options deterministic ([b3a76d4](https://github.com/ysthakur/man-completions/commit/b3a76d4af325489553e6dc5be921ee08b4606b52))
* Remove .sp from descriptions ([140815f](https://github.com/ysthakur/man-completions/commit/140815f08a1f9f09f435ca8082a7659135a817a0))
* Remove angle brackets ([f90b02b](https://github.com/ysthakur/man-completions/commit/f90b02b00947705d2bf8a066b3e53a12a0bd2882))

## 0.1.0 (2023-08-08)


### Features

* Allow excluding man sections ([8387520](https://github.com/ysthakur/man-completions/commit/8387520e4add4ee96969f644d08fb6ed6e301d95))
* broken implementation for zsh ([f19d161](https://github.com/ysthakur/man-completions/commit/f19d1611ff480f9e8503e8d5083a0f5826888285))
* Implement find_manpage ([f55c162](https://github.com/ysthakur/man-completions/commit/f55c162d721cc9a2a6b10c6c214383933393246e))
* read .gz manpages ([fcfa538](https://github.com/ysthakur/man-completions/commit/fcfa5389ad630e0ad71fb7838cc9dc7e780a194c))
