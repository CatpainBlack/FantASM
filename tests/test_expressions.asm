	org	32768+1024-1024

start
	call	start / 2 * 2 << 2 >> 2
	ret

data
	db	1,2,3,4,5,6,7,8,9
	db	start
	db	data-start


