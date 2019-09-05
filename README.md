# FantASM - A Z80 assembler

## Usage

`fantasm <source> <output> [options]`

## Commandline options

* `-N, --z80n` enable z80n extended opcodes
* `-c, --cspect` enable CSpect pseudo ops, BREAK and QUIT
* `-n, --nologo` suppress the startup banner
* `-v, --verbose` enable verbose output
* `-h, --help` display command line help
* `-V, --version` display the program version and exit
* `-D, --debug` Dumps information about the assembly (only useful for FantASM devs)

## Overview

FantASM is a Z80 assembler written in Rust started as a way for me to learn the Rust language. The project can be considered alpha, as there are many bugs, and many features left to implement.

The assembler supports all Z80 opcodes and most undocumented ones, except for undocumented indexed shft instructions (DDCB).

## Building

`cargo build --release` currently builds with rust >= 1.35.0
