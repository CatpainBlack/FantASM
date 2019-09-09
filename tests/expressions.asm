	org	$8000

start	ld	hl, start-10*100+50/2
		ret

		db	"Captain Black",0
		db	finish-start,0,0,0,0
finish