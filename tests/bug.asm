	org 32768

wibble=1

IFNDEF wibble
    ld  a,0
ELSE
    ld  a,1
ENDIF

    db  0,0