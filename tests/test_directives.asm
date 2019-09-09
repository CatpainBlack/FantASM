	org 32768

meaning_of_life = 42

start:
	nop
	ld	hl,hex_string
.loop
	ld	de,meaning_of_life
	ret

	db	"Captain Black",0

hex_string:
	hex	"0123456789ABCDEF00"