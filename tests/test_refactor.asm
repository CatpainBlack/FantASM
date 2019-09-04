	org	32768

wibble = 49152

start
	ld	a,a
	ld	a,b
	ld	a,c
	ld	a,d
	ld	a,e
	ld	a,h
	ld	a,l
	ld	a,ixh
	ld	a,ixl
	ld	a,iyh
	ld	a,iyl

	ld	a,0
	ld	a,wibble
	ld	a,(0x1234)
	ld	a,(wibble)
	ld	a,(start)
	ld	a,(end)
	ld	a,(end-start)
	ld	a,(hl)
	ld	a,(de)
	ld	a,(bc)
	ld	a,r
	ld	a,i
	ld	a,(ix)
	ld	a,(ix+1)
	ld	a,(iy)
	ld	a,(iy+1)

	ld	(0x1234),a
	ld	(wibble),a
	ld	(start),a
	ld	(end),a
	ld	(end-start),a

	ld	r,a
	ld	i,a
	ld	(ix),a
	ld	(ix+1),a
	ld	(iy),a
	ld	(iy+1),a

	ld	b,0
	ld	b,wibble
	ld	b,a
	ld	b,ixh
	ld	b,ixl
	ld	b,iyh
	ld	b,iyl

	ld	c,0
	ld	c,wibble
	ld	c,a
	ld	c,ixh
	ld	c,ixl
	ld	c,iyh
	ld	c,iyl

	ld	d,0
	ld	d,wibble
	ld	d,a
	ld	d,ixh
	ld	d,ixl
	ld	d,iyh
	ld	d,iyl

	ld	e,0
	ld	e,wibble
	ld	e,a
	ld	e,ixh
	ld	e,ixl
	ld	e,iyh
	ld	e,iyl

	ld	h,0
	ld	h,wibble
	ld	h,a

	ld	l,0
	ld	l,wibble
	ld	l,a

end
	ret

data
	db	0,0,0,0,0,0,0,0