	org	 32768+64-64

	!opt z80n on
	!opt cspect on
	!opt verbose on

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