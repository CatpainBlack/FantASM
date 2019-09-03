	org	0x8000

constant = 0x1234

start
	ld	ix,(end)
	ld	ix,(constant)

	ld	iy,0
	ld	iy,constant
	ld	iy,start
	ld	iy,end
	ld	iy,(0x4567)
	ld	iy,(start)
	ld	iy,(end)
	ld	iy,(constant)
end
	ret