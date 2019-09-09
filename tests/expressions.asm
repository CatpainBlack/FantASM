	org	$8000

start:	;ld	hl, start-10*100+50/2
		;ld	de,finish*40
		;ret

		;call	start / 2 * ((2 << 2) >> 2)

		ld	hl,$-1 : ld	ix,$-1
		ld	ix,$-1

		dz	"Captain Black"
		db	"(c)2019 Captain Black",0
		dw	0xFFFF,ASMPC,0xFFFF,$
finish