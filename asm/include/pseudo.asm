    macro   push_all
        push    af
        push    bc
        push    de
        push    hl
    endm

    macro   pop_all
        pop hl
        pop de
        pop bc
        pop af
    endm
