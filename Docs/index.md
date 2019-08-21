## FantASM

FantASM is a two pass non optimising assembler for the Z80 processor.

Most undocumented opcodes are supported, with the exception of the IX and IY bit instructions (for now).

It supports the extended op-codes of the ZX Next and addition pseudo opcodes used by the CSpect emulator to control debugging.

### Usage

`FantASM <source> <output> [options]`

### Directives

* DB
* !opt / #pragma
    
    * z80n
    * cspect
    * verbose