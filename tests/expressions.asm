	org	$8000

    macro   no_params
    ret
    endm

start:	no_params

        add a,start/2

        ld	hl, start-10*100+50/2
		ld	de,finish*40
		ret

		call	start / 2 * ((2 << 2) >> 2)
		add de,49152

		jr	start
		jr	$-1
		jr	nc,.loop
.loop
		jr	nz,.loop

		ld	hl,$-1 : ld	ix,$-1
		ld	ix,$-1

		dz	"Captain Black"
		dz  'Captain Black'
		db	"(c)2019 Captain Black",0
		dw	0xFFFF,$,0xFFFF,$,finish
		dw	0xffff,finish,0xffff
finish
