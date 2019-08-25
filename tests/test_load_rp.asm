	org	0x8000

constant = 0x1234

start
	ld	(0x4567),hl
	ld	(constant),hl
	ld	(start),hl
	ld	(end),hl

	ld	(0x4567),ix
	ld	(constant),ix
	ld	(start),ix
	ld	(end),ix

	ld	(0x4567),iy
	ld	(constant),iy
	ld	(start),iy
	ld	(end),iy

	ld	(0x4567),de
	ld	(constant),de
	ld	(start),de
	ld	(end),de

	ld	(0x4567),bc
	ld	(constant),bc
	ld	(start),bc
	ld	(end),bc

	ld	(0x4567),ix
	ld	(constant),ix
	ld	(start),ix
	ld	(end),ix

	ld	(0x4567),iy
	ld	(constant),iy
	ld	(start),iy
	ld	(end),iy

	ld	de,0
	ld	de,constant
	ld	de,start
	ld	de,end

	ld	bc,0
	ld	bc,constant
	ld	bc,start
	ld	bc,end

	ld	hl,0
	ld	hl,constant
	ld	hl,start
	ld	hl,end
	ld	hl,(0x4567)
	ld	hl,(start)
	ld	hl,(end)
	ld	hl,(constant)

	ld	ix,0
	ld	ix,constant
	ld	ix,start
	ld	ix,end
	ld	ix,(0x4567)
	ld	ix,(start)
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