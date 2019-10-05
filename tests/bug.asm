	org 32768

    ld  a,(ix+1)
    ld  a,(ix-1)
    ret

    db  0