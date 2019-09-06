	org	32768

.pointless

start:
	ld	hl,start
	ld	de,finish-start
	ld	b,10
.loop
	djnz	.loop
finish:
	ret

block_a:
	ld	b,0
	jr	.loop
.loop
	djnz	.loop
	ret
