	org	32768

start
	ld	sp,0
	inc	sp
	add	hl,sp
	dec	sp
	ex	(sp),hl
	ld	sp,hl
	sbc	hl,sp
	ld	(0xffff),sp
	add	hl,sp
	ld	sp,(0xffff)
	add	ix,sp
	ex	(sp),ix
	ld	sp,ix
	add	iy,sp
	ex	(sp),iy
	ld	sp,iy

	ld	(store),sp
	ld	sp,(store)

	ret

store db	0,0

