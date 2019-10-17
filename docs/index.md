## FantASM

FantASM is a two pass non optimising assembler for the Z80 processor.

It supports all undocumented op-codes and the extended instruction set of the ZX Next and additional pseudo opcodes used by the CSpect emulator to control debugging.

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

### Labels & Constants

Labels must start with a letter and may contain letters, numbers or underscores and optionally end with a semi colon.
Local labels start with a period (.) and are only valid until the next none local label is defined.

Constants must start with a letter and may contain letters, numbers or underscores, but not semi-colons. Constants are defined using the following syntax:

```<name> = <expression>``` (the = may be substituted with 'equ')

```<expression>``` may only reference other constants or labels that have previously defined.  

### Non-Decimal Number Formats

Hexadecimal numbers may be in any of the following formats

* 0x12EF
* $12EF
* 012EFh (must have leading zero, which is not included in the final value)

Binary numbers may be in any of the following formats

* %10101010
* 010101010b (must have leading zero, which is not included in the final value)  

### Directives

Directives are not case sensitive, so any variation of ORG, org, Org etc. are completely valid.

```ORG expr```

Tells the assembler at which address to start assembling code.

```!opt / #pragma```
    
Controls different assembly options.

```!message "string"```

Displays a message on the console during assembly

```DB,BYTE nn[,..]```

outputs one or more bytes (8 bit values)

```DW,WORD nnnn[,..]```

outputs one or more words (16 bit values)
    
```DS nn,size```

creates a block of size and fills with the byte nn

```DH,HEX "0-F.."```

Outputs a sequence of 8 bit values by converting each 2 characters at a time, so "12FF" would be output as 0x12,0xFF

```include "filename"```
I
Includes another source file to be assembled.

```binary,incbin "filename" ```

Includes a binary file.

```IF / ELSE / ENDIF```

Conditinally control assembly.
    
```SIZEOF(label)```

Returns the size of an included binary file.
In order to support ```SIZEOF```, your ```INCBIN``` must be preceded by a label.


### Expressions

### Macros

Macros may have 0 or more parameters, and may only declare local labels (labels that start with a .)

#### Simple Macro Example
```    org 0x8000

    MACRO   border colour
        out (0xfe),colour
    ENDM

start
    border  0
    ret
```


### History

1.1.3

* Added: IF/ELSE/ENDIF conditional assembly
* Added: -D --define command line option
* Fixed: Support for (ix+expr) and (iy+expr)
* Added: SIZEOF(label) - Gives the size of an included binary file

1.1.2

* Fixed: Exporting of labels should not include temp/local labels
* Fixed: ld sp,label
* Fixed: (ix-n) would not assemble

1.1.1

* Fixed: ```ld r8,constant``` emitted an extra byte
* Fixed: Labels with the characters "0x" would be incorrectly parsed as a number
* Fixed: Error when using single quoted character as eight bit value
* Fixed: Encoding strings that contained £, ↑, or © (UTF-8)

1.1.0

* Added: ```--max-code-size``` commandline switch
* Added: ```--origin``` commandline switch

1.0.0-rc2

* Fixed: Issue with ```equ``` causing syntax error
* Added: ```--export-labels``` commandline switch

1.0.0-rc1

* Added: -I,--include commandline option

0.9.1

* Fixed: Error expanding macros with no parameters
* Changed the expression parser, now correctly handles OR/AND ( | & )
* Fixed: expression parsing on certain opcodes

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
