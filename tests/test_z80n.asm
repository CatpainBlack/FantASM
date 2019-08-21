	org	0x8000

wibble = 0x9000

start
	LDIX
	LDWS
	LDIRX
	LDDX
	LDDRX
	LDPIRX
	OUTINB
	MUL		D,E
	ADD		HL,A
	ADD		DE,A
	ADD		BC,A
	ADD		HL,49152
	ADD		DE,49152
	ADD		BC,49152
	add		hl,wibble
	add		hl,start
	add		hl,data
	SWAPNIB
	MIRROR
	PUSH 	49152
	push	start
	push	data
	NEXTREG 0x1f,0xff
	NEXTREG 0x22,A
	PIXELDN
	PIXELAD
	SETAE
	TEST	0x22

    ret

data
	db	0,0,0,0