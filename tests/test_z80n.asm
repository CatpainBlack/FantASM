	org	0x8000

wibble = 0x9000
register = 0xff

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
	add		de,start
	add		bc,data

	SWAPNIB
	MIRROR

	PUSH 	49152
	push	start
	;push	data

	nextreg	register-register+20,20+0x13
	NEXTREG	register,register
	NEXTREG 0x1f,0xff
	NEXTREG 0x22,A

	PIXELDN
	PIXELAD
	SETAE

	test	register
	test	end
	test	register-register+2
	TEST	0x22

    ret

data
	db	end-data,0,0,0
	dw	end

end