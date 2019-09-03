	org	0x8000

constant = 0x1234

start
	call	start+1
	call	start
	call	end
	call	0
	call	65535

	jp	start+1
end
	ret