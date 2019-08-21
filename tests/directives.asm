	org	 32768+64-64

	#pragma z80n off
	!opt z80n on
	!opt cspect true
	!opt verbose yes

start
	;!message "start"
	call	end
	djnz	end
	jr		end
	ld		hl,end
	;ld	(end),a
	break
	ret

end
	;!message "end"
	ret



message:
	db	255,"Hello, World!",0 ; db

	;!message "End "