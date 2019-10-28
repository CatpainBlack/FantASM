	org 32768

    ld  hl,$4000
    ld  de,$4001
    ld  bc,6912
    ld  (hl),l
    ldir
    ret