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
  -D,--define DEFINE    Define 1 more constants
  -e,--export-labels    Export labels to a file
  -O,--origin ORIGIN    Address to start assembling code
  -M,--max-code-size=   Limit the size of assembled code
</pre>

## To-Do

General
- [ ] Improve the documentation

1.1.3
- [x] ```--define```        Add commandline switch to define a constant.
- [x] ```Expressions```     Fix remaining opcodes that don't use expressions, eg: ```ld (ix+n)),r```.
- [x] ```SizeOf```          Give the size of an included binary file.
- [x] ```Conditionals```    Conditional assembly using IF, ELSE, ENDIF.
- [x] ```Include```         Include needs to check the same directory as the source file that requested it.
- [x] ```Exports```         Added GLOBAL directive (similar to NASM).

1.1.4
- [x] ```IFDEF```           Condition assembly.
- [x] Fix/Update all errors to be more descriptive and helpful
- [x] Fix expression parser bitwise operations
- [x] ```#define```         Single line macros
- [x] ```ENUM```            Enumerator directive.

1.1.x
- [ ] ```Structs```         Define data structures, similar to C.
- [ ] ```Modules```         Group code into modules.
- [ ] ```zx7/lz4```         Add support for compression, either as output or when including binary files.
- [ ] Source code formatting/linter