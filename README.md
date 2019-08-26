# FantASM - A Z80 assembler

## Usage

`fantasm <source> <output> [options]`

## Commandline options

* `--nologo` suppress the startup banner
* `--z80n` enable z80n extended opcodes
* `--cspect` enable CSpect pseudo ops, BREAK and QUIT
* `--verbose` enable verbose output
* `--help` display command line help

## Overview

FantASM is a Z80 assembler written in Rust started as a way for me to learn the Rust language. The project can be considered alpha, as there are many bugs, and many features left to implement.

The assembler supports all Z80 opcodes and most undocumented ones, except for undocumented indexed shft instructions (DDCB).

## Building

`cargo build --release` currently builds with rust >= 1.35.0
