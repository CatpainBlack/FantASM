	org 32768

    ld  hl,bug
    ld  de,0x4000
    ld  bc,SIZEOF(bug)

bug        incbin  "tests/bug.asm"

