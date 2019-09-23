	org 32768

    testing = 2

    break
	ld	c,testing
	ld	a,(cmd_flags)
	or	c
	ld	(cmd_flags),a

    ld  a,''

cmd_flags    db  0

cack    dz  "Wibble",12,"Abc"