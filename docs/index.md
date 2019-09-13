## FantASM

FantASM is a two pass non optimising assembler for the Z80 processor.

It supports all undocumented op-codes and the extended instruction set of the ZX Next and additional pseudo opcodes used by the CSpect emulator to control debugging.

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

### Labels & Constants

Labels must start with a letter and may contain letters, numbers or underscores and optionally end with a semi colon.
Local labels start with a period (.) and are only valid until the next none local label is defined.

Constants must start with a letter and may contain letters, numbers or underscores, but not semi-colons. Constants are defined using the following syntax:

`<name> = <expression>` (the = may be substituted with 'equ')

`<expression>` may only reference other constants or labels that have previously defined.  

### None Decimal Number Formats

Hexadecimal numbers may be in any of the following formats

* 0x12EF
* $12EF
* 012EFh (must have leading zero, which is not included in the final value)

Binary numbers may be in any of the following formats

* %10101010
* 010101010b (must have leading zero, which is not included in the final value)  

### Directives

`ORG <addr>`
    Tells the assembler at which address to start assembling code.

`!opt` / `#pragma`
    Controls different assembly options.

`!message <string>`
    Displays a message on the console during assembly

`DB,BYTE nn[,..]`
    outputs one or more bytes (8 bit values)

`DW,WORD nnnn[,..]`
    outputs one or more words (16 bit values)
    
`DS <string>[,..]`
    outputs one or mor strings

`DH,HEX "0-F.."`
    Outputs a sequence of 8 bit values by converting each 2 characters at a time, so "12FF" would be output as 0x12,0xFF

`include <filename>`
    Includes another source file to be assembled.

`binary, incbin`
    Includes a binary file.

### Expressions

### Macros

### History

0.9.1
* Changed the expression parser, now correctly handles OR/AND ( | & )
* Fixed expression parsing on certain opcodes
* Fixed error expanding macros with no parameters

0.9.0

* Added a much better expression parser. Brackets and operator precedence are now properly implemented.
* Reworked how warnings are handled and displayed.
* Improved string handling, now checks are performed to determine if a string has non ASCII characters.
* Added DZ directive, zero terminated strings
* Implemented $/ASMPC psuedo operator, returns the current PC
* Allow multiple instructions per line separated by colon

0.8.0

* Implemented MACRO/ENDM/END directives.

0.7.7

* Added check for recursive include files.
* Implemented HEX/DH directive.
* Implemented local labels (prefixed with a .).
* Fixed an issue with labels that ended with a colon.
* Warning when bit operations specify an illegal bit number.
* Added BSD license (see license.txt).

0.7.6

* Implemented expression parsing for Indirect load instructions.
* Added undocumented bit/res/set/rotate opcodes using the index registers, all undocumented instructions are now handled.
* Refactored the instruction encoder, reduced the amount of spaghetti logic.

0.7.5

* Big code refactoring in order to better implement future expression parser improvements.

0.7.4

* Implemented "Incbin" directive.
* Internal code refactoring to remove some "unsafe" code.

0.7.3
 
* Better handling of expressions when referencing labels that have not yet been defined.

0.7.2

* Improved assembly speed.
* Implemented CSpect pseudo opcodes.
* Implemented z80n extended opcodes.

0.7.1

* First public release.