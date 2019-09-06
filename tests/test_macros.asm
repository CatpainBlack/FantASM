	org	32768

	macro cls fore,back
		ld	hl,fore
		ld	de,back
		ret
	endm

init:
	cls	0x12,0xffff-init
