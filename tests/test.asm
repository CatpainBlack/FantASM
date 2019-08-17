	org	 32768+64-64

;!message "Start"

start
	call	end
	djnz	end
	jr		end
	ld		hl,end
	;ld	(end),a
	ret

end
	ret



message:
	db	255,"Hello, World!",0 ; db

;!message "End "