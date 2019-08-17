# FantASM - A Z80 assembler

## Usage

`fantasm <source> <output>`

## Commandline options

* `--verbose` enable verbose output
* `--help display` command line help

## Overview

FantASM is a Z80 assembler written in Rust started as a way for me to learn the Rust language. The project can be considered alpha, as there are many bugs, and many features left to implement.

The assembler supports all Z80 opcodes and most undocumented ones, except for undocumented indexed shft instructions (DDCB).

## Building

`curgo build` should be sufficient, currently builds with rust 1.35.0 - 1.37.0

## ToDo

- [ ] Implement all undocumented opcodes
- [ ] Implement Z80n extended instructions (ZX Next)
- [ ] Improve the expression parser to handle nested expresions in brackets
- [ ] Add support for more directives
- - [ ] Incbin/Binary includes
- - [ ] Macros