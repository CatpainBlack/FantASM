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
-N, --z80n                Enable z80n extended opcodes.
-c, --cspect              Enable CSpect pseudo ops, BREAK and QUIT.
-I, --include             Add a directory to search when looking for includes. 
                          May be used more than once to add multiple directories.
-e, --export-labels       Exports labels to the given text file.
-O, --origin              Address at which to start assembling code.
-M, --max-code-size       Limit the size of assembled code to nnnn bytes.
-n, --nologo              Suppress the startup banner.
-v, --verbose             Enable verbose output.
-h, --help                Display command line help.
-V, --version             Display the program version and exit.
</pre>

## ToDo
- [ ] ```--define``` - Add commandline switch to define a constant.
- [ ] ```Expressions``` - Fix remaining opcodes that don't use expressions, eg: ```ld (ix+n)),r```.
- [ ] ```SizeOf``` - Give the size of an included binary file.
- [ ] ```Structs``` - Define data structures, similar to C.
- [ ] ```Modules``` - Group code into modules.
- [ ] ```Exports``` - Define a way of exporting only certain labels.
- [ ] ```Conditionals``` - Conditional assembly using DEFINE, IFDEF, ELSE, ENDIF.
- [ ] ```Include``` - Include needs to check the same directory as the source file that requested it.
- [ ] ```zx7/lz4``` - Add support for compression, either as output or when including binary files.