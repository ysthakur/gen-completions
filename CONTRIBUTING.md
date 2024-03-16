# Contributing

Any and all contributions are welcome.

Here are some things that need work:

- Port all of Fish's functionality
  - Specifically, Fish's Darwin and Degroff parsers
  - Figure out why fish only seems to use man1, man6, and man8
- Tests:
  - Add .gz and .bz2 files to the tests folder?
  - Test excluding/including commands and directories
  - Find samples of type 4 man pages, Darwin, and Degroff to test
- Handle options like `-vv` and `-/` in Nushell
- Infer types of parameters (e.g. from `-d=DIR`)
  - Probably very tough but nice to have in future sometime
