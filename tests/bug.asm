	org 32768

    testing = 2

    break
	ld	c,testing
	ld	a,(cmd_flags)
	or	c
	ld	(cmd_flags),a

cmd_flags    db  0