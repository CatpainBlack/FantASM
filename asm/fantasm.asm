    include "include/p3dos.asm"
    include "include/esx_dos.asm"
    include "include/zxnext.asm"
    include "include/keyboard.asm"
    include "include/tilemap.asm"
    include "include/pseudo.asm"

start   tm_clip 0,159,0,255
        ld      hl,message
        call    strlen
        ld      b,h
        ld      c,l
        ret

; Hl = address of string
strlen: ld      bc,256
        xor     a
        cpir
        ret     z
        ld      hl,256
        sbc     hl,bc
        ret

message:    dz  "Hello, World!"
