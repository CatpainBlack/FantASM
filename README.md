## FantASM

FantASM is a two pass non optimising assembler for the Z80 processor.

It supports all undocumented op-codes and the extended instruction set of the ZX Next and additional pseudo opcodes used by the CSpect emulator to control debugging.

[Documentation](https://catpainblack.github.io/FantASM/)

## Build

To build from source (requires rust 1.35+):

```cargo build --release```

and copy the binary somewhere in your path.


## Usage

```fantasm <source> <output> [options]```

## Commandline options

<pre>
  -h,--help             Show this help message and exit
  -N,--z80n             Enable Z80n (ZX Next) cpu extensions
  -c,--cspect           Enable cspect "exit" and "break" instructions
  -n,--nologo           Do no display the program name and version
  -v,--verbose          Enable verbose output
  -V,--version          Displays the version and exits
  -I,--include INCLUDE  Add a directory to search for include files
  -i,--case-insensitive Enable case insensitive labels
  -D,--define DEFINE    Define 1 more constants
  -e,--export-labels    Export labels to a file
  -O,--origin ORIGIN    Address to start assembling code
  -M,--max-code-size=   Limit the size of assembled code
</pre>

## To-Do

1.1.7
- [ ] Fix: Attempt to redefine label or constant when emitting structs without label
- [ ] Add: .z/.s types to structures

General
- [ ] Improve the documentation

1.x.x
- [ ] Source code formatting/linter
- [ ] Disassembler
- [ ] Build System, config file driven with support for different output types (TAP, NEX etc)
