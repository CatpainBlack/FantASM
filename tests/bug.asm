	org 32768

start
    ld  hl,bug
    ld  de,0x4000
    ld  bc,SIZEOF(bug)


global
bug        incbin  "bug.asm"

