	org 32768

    ld  sp,nt_
    ld  a,(ix+1)
    ld  a,(ix-1)
    ret

nt_    db  0