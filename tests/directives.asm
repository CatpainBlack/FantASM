	org	 32768 + 1024

start
	call	end
	call	end-start
	push	32768
	push	start
	push	end
	ret
end
