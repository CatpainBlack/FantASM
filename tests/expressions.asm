	org	$8000

start	ld	hl, start-10*100+50/2
		ld	de,finish*40
		ret

		call	start / 2 * ((2 << 2) >> 2)

		db	"Captain Black",0
finish