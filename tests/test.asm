	org	 32768+64-64

	;!message "Start"
	;!opt z80n
	;!opt verbose


start
	call	end
	djnz	end
	jr		end
	ld		hl,end
	;ld	(end),a
	break
	ret

end
	ret



message:
	db	255,"Hello, World!",0 ; db

	;!message "End "