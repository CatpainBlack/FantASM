	org	32768+1024-1024

start
	call	start / 2 * 2 << 2 >> 2
	ret

data
	byte	0,1,2
	db	1,2,3,4,5,6,7,8,9
	;db	start
	db	data-start
	word 0,1,2
	dw	0,start,data,data-start,0x5555
	block	20
	block	5,0x80
	ds	20
	ds	21,255
	block	5,start>>8

