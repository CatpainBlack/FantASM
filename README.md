## FantASM

FantASM is a two pass non optimising assembler for the Z80 processor.

It supports all undocumented op-codes and the extended instruction set of the ZX Next and additional pseudo opcodes used by the CSpect emulator to control debugging.

[Documentation](https://catpainblack.github.io/FantASM/)

## Build

To build from source (requires rust 1.35+):

`cargo build --release` 

and copy the binary somewhere in your path.

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


