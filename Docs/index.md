## FantASM

FantASM is a two pass non optimising assembler for the Z80 processor.

Most undocumented opcodes are supported, with the exception of the IX and IY bit instructions (for now).

It supports the extended op-codes of the ZX Next and additional pseudo opcodes used by the CSpect emulator to control debugging.

### Usage

`FantASM <source> <output> [options]`

`<source>` Assembly source code to process

`<output>` Set the name of the output file  

`--z80n` Enable Z80n (ZX Next) CPU extensions

`--cspect` Enable cspect "exit" and "break" pseudo instructions 

`--nologo` Do no display the program name and version

`--debug` Dumps addition debug information to the console after assembly

`--verbose` Enable verbose output

### Labels & Constants


### Directives

`ORG`

`!opt` / `#pragma`

`!message`

`DB`

`DW`

`DS`

`DH` - todo

`include`

`incbin`

### History

0.7.6
- Implemented expression parsing for Indirect load instructions
- Refactored the instruction encoder, reduced the amount of spaghetti logic

0.7.5
- Big code refactoring in order to better implement future expression parser improvements

0.7.4
- Implemented "Incbin" directive
- Internal code refactoring to remove some "unsafe" code

0.7.3 
- Better handling of expressions when referencing labels that have not yet been defined

0.7.2
- Improved assembly speed
- Implemented CSpect pseudo opcodes
- Implemented z80n extended opcodes

0.7.1
- First public release