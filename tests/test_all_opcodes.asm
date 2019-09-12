    org $8000

set0
    nop
    ld  bc,0xbcbd
    ld  (bc),a
    inc bc
    inc b
    dec b
    ld  b,0xbb
    rlca
    ex  af,af'
    add hl,bc
    ld  a,(bc)
    dec bc
    inc c
    dec c
    ld  c,0xcc
    rrca

set1
    djnz    set1
    ld  de,0xdedf
    ld  (de),a
    inc de
    inc d
    dec d
    ld  d,0xdd
    rla
    jr  set1
    add hl,de
    ld  a,(de)
    dec de
    inc e
    dec e
    ld  e,0xee
    rra

set2
    jr  nz,set2
    ld  hl,0x7172
    ld  (0x1234),hl
    inc hl
    inc h
    dec h
    ld  h,0x22
    daa
    jr  z,set2
    add hl,hl
    ld  hl,(0x3132)
    dec hl
    inc l
    dec l
    ld  l,0x11
    cpl

set3
    jr  nc,set3
    ld  sp,0x0001
    ld  (0x0001),a
    inc sp
    inc (hl)
    dec (hl)
    ld  (hl),0xff
    scf
    jr  c,set3
    add hl,sp
    ld  a,(0xffee)
    dec sp
    inc a
    dec a
    ld  a,0xaa
    ccf

set4
    ld  b,b
    ld  b,c
    ld  b,d
    ld  b,e
    ld  b,h
    ld  b,l
    ld  b,(hl)
    ld  b,a
    ld  c,b
    ld  c,c
    ld  c,d
    ld  c,e
    ld  c,h
    ld  c,l
    ld  c,(hl)
    ld  c,a

set5
    ld  d,b
    ld  d,c
    ld  d,d
    ld  d,e
    ld  d,h
    ld  d,l
    ld  d,(hl)
    ld  d,a
    ld  e,b
    ld  e,c
    ld  e,d
    ld  e,e
    ld  e,h
    ld  e,l
    ld  e,(hl)
    ld  e,a

set6
    ld  h,b
    ld  h,c
    ld  h,d
    ld  h,e
    ld  h,h
    ld  h,l
    ld  h,(hl)
    ld  h,a
    ld  l,b
    ld  l,c
    ld  l,d
    ld  l,e
    ld  l,h
    ld  l,l
    ld  l,(hl)
    ld  l,a

set7
    ld  (hl),b
    ld  (hl),c
    ld  (hl),d
    ld  (hl),e
    ld  (hl),h
    ld  (hl),l
    halt
    ld  (hl),a
    ld  a,b
    ld  a,c
    ld  a,d
    ld  a,e
    ld  a,h
    ld  a,l
    ld  a,(hl)
    ld  a,a

set8
    add a,b
    add a,c
    add a,d
    add a,e
    add a,h
    add a,l
    add a,(hl)
    add a,a
    adc a,b
    adc a,c
    adc a,d
    adc a,e
    adc a,h
    adc a,l
    adc a,(hl)
    adc a,a

set9
    sub b
    sub c
    sub d
    sub e
    sub h
    sub l
    sub (hl)
    sub a
    sbc a,b
    sbc a,c
    sbc a,d
    sbc a,e
    sbc a,h
    sbc a,l
    sbc a,(hl)
    sbc a,a

setA
    and b
    and c
    and d
    and e
    and h
    and l
    and (hl)
    and a
    xor b
    xor c
    xor d
    xor e
    xor h
    xor l
    xor (hl)
    xor a

setB
    or b
    or c
    or d
    or e
    or h
    or l
    or (hl)
    cp a
    cp  b
    cp  c
    cp  d
    cp  e
    cp  h
    cp  l
    cp  (hl)
    cp  a

setC
    ret nz
    pop bc
    jp  nz,setC
    jp  setC
    call nz,setC
    push    bc
    add a,0xaa
    rst 0
    ret z
    ret
    jp  z,setC
    ; BITS
    call    z,setC
    call    setC
    adc    a,0xAA
    rst 8

setD
    ret nc
    pop de
    jp  nc,setD
    out (0xfe),a
    call nc,setD
    push de
    sub 0xff
    rst 0x10
    ret c
    exx
    jp  c,setD
    in  a,(0xfe)
    call nz,setD
    ; 0xDD
    sbc a,0xAA
    rst $18

setE
	ret po
	pop hl
	jp po,setE
	ex (sp),hl
	call po,setE
	push hl
	and 0xaa
	rst 0x20
	ret pe
	jp (hl)
	jp pe,setE
	ex  de,hl
	ex  hl,de
	call pe,setE
    ;EXTD
    xor 0x12
    rst 0x28

setF
	ret p
	pop af
	jp p,setF
	di
	call p,setF
	push af
	or 0x00
	rst 030h
	ret m
	ld sp,hl
	jp m,setF
	ei
	call m,setF
    ;;IY
    cp 0xcc
    rst 038h

edset4
	in b,(c)
	out (c),b
	sbc hl,bc
	ld (0xbcbd),bc
	neg
	retn
	im 0
	ld i,a
	in c,(c)
	out (c),c
	adc hl,bc
	ld bc,(0xbcbd)
	reti
	;im 0/1
	ld r,a

edset5
	in d,(c)
	out (c),d
	sbc hl,de
	ld (0xdedf),de
	neg
	retn
	im 1
	ld a,i
	in e,(c)
	out (c),e
	adc hl,de
	ld de,(0xdedf)
	neg
	retn
	im 2
	ld a,r

edset6
	in h,(c)
	out (c),h
	sbc hl,hl
	ld (0x1234),hl
	neg
	retn
	im 0
	rrd
	in l,(c)
	out (c),l
	adc hl,hl
	ld hl,(0x4321)
	neg
	retn
	;im 0/1
	rld

edset7
	in (c)
	out (c),0
	sbc hl,sp
	ld (0x1234),sp
	neg
	retn
	im 1
	in a,(c)
	out (c),a
	adc hl,sp
	ld sp,(0x1234)
	neg
	retn
	im 2

edsetA
	ldi
	cpi
	ini
	outi
	ldd
	cpd
	ind
	outd

edsetb
	ldir
	cpir
	inir
	otir
	lddr
	cpdr
	indr
	otdr

cbset0
	rlc b
	rlc c
	rlc d
	rlc e
	rlc h
	rlc l
	rlc (hl)
	rlc a
	rrc b
	rrc c
	rrc d
	rrc e
	rrc h
	rrc l
	rrc (hl)
	rrc a

cbset1
	rl b
	rl c
	rl d
	rl e
	rl h
	rl l
	rl (hl)
	rl a
	rr b
	rr c
	rr d
	rr e
	rr h
	rr l
	rr (hl)
	rr a

cbset2
	sla b
	sla c
	sla d
	sla e
	sla h
	sla l
	sla (hl)
	sla a
	sra b
	sra c
	sra d
	sra e
	sra h
	sra l
	sra (hl)
	sra a

cbset3
	sll b
	sll c
	sll d
	sll e
	sll h
	sll l
	sll (hl)
	sll a
	srl b
	srl c
	srl d
	srl e
	srl h
	srl l
	srl (hl)
	srl a

cbset4
	bit 0,b
	bit 0,c
	bit 0,d
	bit 0,e
	bit 0,h
	bit 0,l
	bit 0,(hl)
	bit 0,a
	bit 1,b
	bit 1,c
	bit 1,d
	bit 1,e
	bit 1,h
	bit 1,l
	bit 1,(hl)
	bit 1,a

cbset5
	bit 2,b
	bit 2,c
	bit 2,d
	bit 2,e
	bit 2,h
	bit 2,l
	bit 2,(hl)
	bit 2,a
	bit 3,b
	bit 3,c
	bit 3,d
	bit 3,e
	bit 3,h
	bit 3,l
	bit 3,(hl)
	bit 3,a

cbset6
	bit 4,b
	bit 4,c
	bit 4,d
	bit 4,e
	bit 4,h
	bit 4,l
	bit 4,(hl)
	bit 4,a
	bit 5,b
	bit 5,c
	bit 5,d
	bit 5,e
	bit 5,h
	bit 5,l
	bit 5,(hl)
	bit 5,a

cbset7
	bit 6,b
	bit 6,c
	bit 6,d
	bit 6,e
	bit 6,h
	bit 6,l
	bit 6,(hl)
	bit 6,a
	bit 7,b
	bit 7,c
	bit 7,d
	bit 7,e
	bit 7,h
	bit 7,l
	bit 7,(hl)
	bit 7,a

cbset8
	res 0,b
	res 0,c
	res 0,d
	res 0,e
	res 0,h
	res 0,l
	res 0,(hl)
	res 0,a
	res 1,b
	res 1,c
	res 1,d
	res 1,e
	res 1,h
	res 1,l
	res 1,(hl)
	res 1,a

cbset9
	res 2,b
	res 2,c
	res 2,d
	res 2,e
	res 2,h
	res 2,l
	res 2,(hl)
	res 2,a
	res 3,b
	res 3,c
	res 3,d
	res 3,e
	res 3,h
	res 3,l
	res 3,(hl)
	res 3,a

cbseta
	res 4,b
	res 4,c
	res 4,d
	res 4,e
	res 4,h
	res 4,l
	res 4,(hl)
	res 4,a
	res 5,b
	res 5,c
	res 5,d
	res 5,e
	res 5,h
	res 5,l
	res 5,(hl)
	res 5,a

cbsetb
	res 6,b
	res 6,c
	res 6,d
	res 6,e
	res 6,h
	res 6,l
	res 6,(hl)
	res 6,a
	res 7,b
	res 7,c
	res 7,d
	res 7,e
	res 7,h
	res 7,l
	res 7,(hl)
	res 7,a

cbsetc
	set 0,b
	set 0,c
	set 0,d
	set 0,e
	set 0,h
	set 0,l
	set 0,(hl)
	set 0,a
	set 1,b
	set 1,c
	set 1,d
	set 1,e
	set 1,h
	set 1,l
	set 1,(hl)
	set 1,a

cbsetd
	set 2,b
	set 2,c
	set 2,d
	set 2,e
	set 2,h
	set 2,l
	set 2,(hl)
	set 2,a
	set 3,b
	set 3,c
	set 3,d
	set 3,e
	set 3,h
	set 3,l
	set 3,(hl)
	set 3,a

cbsete
	set 4,b
	set 4,c
	set 4,d
	set 4,e
	set 4,h
	set 4,l
	set 4,(hl)
	set 4,a
	set 5,b
	set 5,c
	set 5,d
	set 5,e
	set 5,h
	set 5,l
	set 5,(hl)
	set 5,a

cbsetf
	set 6,b
	set 6,c
	set 6,d
	set 6,e
	set 6,h
	set 6,l
	set 6,(hl)
	set 6,a
	set 7,b
	set 7,c
	set 7,d
	set 7,e
	set 7,h
	set 7,l
	set 7,(hl)
	set 7,a

ixinstr
	add ix,bc
	add ix,de
	ld ix,0x0102
	ld (0x0203),ix
	inc ix
	inc ixh
	dec ixh
	ld ixh,0x3
	add ix,ix
	ld ix,(0x0304)
	dec ix
	inc ixl
	dec ixl
	ld ixl,0x4
    inc (ix+1)
    dec (ix+1)
    ld (ix+1),0x11
    add ix,sp
    ld b,ixh
    ld b,ixl
    ld b,(ix+1)
    ld c,ixh
    ld c,ixl
    ld c,(ix+1)
    ld d,ixh
    ld d,ixl
    ld d,(ix+1)
    ld e,ixh
    ld e,ixl
    ld e,(ix+1)
    ld ixh,b
    ld ixh,c
    ld ixh,d
    ld ixh,e
    ld ixh,ixh
    ld ixh,ixl
    ld h,(ix+1)
    ld ixh,a
    ld ixl,b
    ld ixl,c
    ld ixl,d
    ld ixl,e
    ld ixl,ixh
    ld ixl,ixl
    ld l,(ix+1)
    ld ixl,a
    ld (ix+1),b
    ld (ix+1),c
    ld (ix+1),d
    ld (ix+1),e
    ld (ix+1),h
    ld (ix+1),l
    ld (ix+1),a
    ld a,ixh
    ld a,ixl
    ld a,(ix+1)
    add a,ixh
    add a,ixl
    add a,(ix+1)
    adc a,ixh
    adc a,ixl
    adc a,(ix+1)
    sub ixh
    sub ixl
    sub (ix+1)
    sbc a,ixh
    sbc a,ixl
    sbc a,(ix+1)
    and ixh
    and ixl
    and (ix+1)
    xor ixh
    xor ixl
    xor (ix+1)
    or ixh
    or ixl
    or (ix+1)
    cp ixh
    cp ixl
    cp (ix+1)
	pop ix
	ex (sp),ix
	push ix
	jp (ix)
    ld sp,ix

iyinstr
	add iy,bc
	add iy,de
	ld iy,0x0102
	ld (0x0203),iy
	inc iy
	inc iyh
	dec iyh
	ld iyh,0x3
	add iy,iy
	ld iy,(0x0304)
	dec iy
	inc iyl
	dec iyl
	ld iyl,0x4
    inc (iy+1)
    dec (iy+1)
    ld (iy+1),0x11
    add iy,sp
    ld b,iyh
    ld b,iyl
    ld b,(iy+1)
    ld c,iyh
    ld c,iyl
    ld c,(iy+1)
    ld d,iyh
    ld d,iyl
    ld d,(iy+1)
    ld e,iyh
    ld e,iyl
    ld e,(iy+1)
    ld iyh,b
    ld iyh,c
    ld iyh,d
    ld iyh,e
    ld iyh,iyh
    ld iyh,iyl
    ld h,(iy+1)
    ld iyh,a
    ld iyl,b
    ld iyl,c
    ld iyl,d
    ld iyl,e
    ld iyl,iyh
    ld iyl,iyl
    ld l,(iy+1)
    ld iyl,a
    ld (iy+1),b
    ld (iy+1),c
    ld (iy+1),d
    ld (iy+1),e
    ld (iy+1),h
    ld (iy+1),l
    ld (iy+1),a
    ld a,iyh
    ld a,iyl
    ld a,(iy+1)
    add a,iyh
    add a,iyl
    add a,(iy+1)
    adc a,iyh
    adc a,iyl
    adc a,(iy+1)
    sub iyh
    sub iyl
    sub (iy+1)
    sbc a,iyh
    sbc a,iyl
    sbc a,(iy+1)
    and iyh
    and iyl
    and (iy+1)
    xor iyh
    xor iyl
    xor (iy+1)
    or iyh
    or iyl
    or (iy+1)
    cp iyh
    cp iyl
    cp (iy+1)
	pop iy
	ex (sp),iy
	push iy
	jp (iy)
    ld sp,iy

ixcb
	rlc (ix+1),b
	rlc (ix+1),c
	rlc (ix+1),d
	rlc (ix+1),e
	rlc (ix+1),h
	rlc (ix+1),l
	rlc (ix+1)
	rlc (ix+1),a
	rrc (ix+1),b
	rrc (ix+1),c
	rrc (ix+1),d
	rrc (ix+1),e
	rrc (ix+1),h
	rrc (ix+1),l
	rrc (ix+1)
	rrc (ix+1),a

	rl (ix+1),b
	rl (ix+1),c
	rl (ix+1),d
	rl (ix+1),e
	rl (ix+1),h
	rl (ix+1),l
	rl (ix+1)
	rl (ix+1),a
	rr (ix+1),b
	rr (ix+1),c
	rr (ix+1),d
	rr (ix+1),e
	rr (ix+1),h
	rr (ix+1),l
	rr (ix+1)
	rr (ix+1),a

	sla (ix+1),b
	sla (ix+1),c
	sla (ix+1),d
	sla (ix+1),e
	sla (ix+1),h
	sla (ix+1),l
	sla (ix+1)
	sla (ix+1),a
	sra (ix+1),b
	sra (ix+1),c
	sra (ix+1),d
	sra (ix+1),e
	sra (ix+1),h
	sra (ix+1),l
	sra (ix+1)
	sra (ix+1),a

	sll (ix+1),b
	sll (ix+1),c
	sll (ix+1),d
	sll (ix+1),e
	sll (ix+1),h
	sll (ix+1),l
	sll (ix+1)
	sll (ix+1),a
	srl (ix+1),b
	srl (ix+1),c
	srl (ix+1),d
	srl (ix+1),e
	srl (ix+1),h
	srl (ix+1),l
	srl (ix+1)
	srl (ix+1),a

	bit 0,(ix+1)
	bit 0,(ix+1)
	bit 0,(ix+1)
	bit 0,(ix+1)
	bit 0,(ix+1)
	bit 0,(ix+1)
	bit 0,(ix+1)
	bit 0,(ix+1)
	bit 1,(ix+1)
	bit 1,(ix+1)
	bit 1,(ix+1)
	bit 1,(ix+1)
	bit 1,(ix+1)
	bit 1,(ix+1)
	bit 1,(ix+1)
	bit 1,(ix+1)

	bit 2,(ix+1)
	bit 2,(ix+1)
	bit 2,(ix+1)
	bit 2,(ix+1)
	bit 2,(ix+1)
	bit 2,(ix+1)
	bit 2,(ix+1)
	bit 2,(ix+1)

	bit 3,(ix+1)
	bit 3,(ix+1)
	bit 3,(ix+1)
	bit 3,(ix+1)
	bit 3,(ix+1)
	bit 3,(ix+1)
	bit 3,(ix+1)
	bit 3,(ix+1)

	bit 4,(ix+1)
	bit 4,(ix+1)
	bit 4,(ix+1)
	bit 4,(ix+1)
	bit 4,(ix+1)
	bit 4,(ix+1)
	bit 4,(ix+1)
	bit 4,(ix+1)

	bit 5,(ix+1)
	bit 5,(ix+1)
	bit 5,(ix+1)
	bit 5,(ix+1)
	bit 5,(ix+1)
	bit 5,(ix+1)
	bit 5,(ix+1)
	bit 5,(ix+1)

	bit 6,(ix+1)
	bit 6,(ix+1)
	bit 6,(ix+1)
	bit 6,(ix+1)
	bit 6,(ix+1)
	bit 6,(ix+1)
	bit 6,(ix+1)
	bit 6,(ix+1)

	bit 7,(ix+1)
	bit 7,(ix+1)
	bit 7,(ix+1)
	bit 7,(ix+1)
	bit 7,(ix+1)
	bit 7,(ix+1)
	bit 7,(ix+1)
	bit 7,(ix+1)

	res 0,(ix+1),b
	res 0,(ix+1),c
	res 0,(ix+1),d
	res 0,(ix+1),e
	res 0,(ix+1),h
	res 0,(ix+1),l
	res 0,(ix+1)
	res 0,(ix+1),a

	res 1,(ix+1),b
	res 1,(ix+1),c
	res 1,(ix+1),d
	res 1,(ix+1),e
	res 1,(ix+1),h
	res 1,(ix+1),l
	res 1,(ix+1)
	res 1,(ix+1),a

	res 2,(ix+1),b
	res 2,(ix+1),c
	res 2,(ix+1),d
	res 2,(ix+1),e
	res 2,(ix+1),h
	res 2,(ix+1),l
	res 2,(ix+1)
	res 2,(ix+1),a

	res 3,(ix+1),b
	res 3,(ix+1),c
	res 3,(ix+1),d
	res 3,(ix+1),e
	res 3,(ix+1),h
	res 3,(ix+1),l
	res 3,(ix+1)
	res 3,(ix+1),a

	res 4,(ix+1),b
	res 4,(ix+1),c
	res 4,(ix+1),d
	res 4,(ix+1),e
	res 4,(ix+1),h
	res 4,(ix+1),l
	res 4,(ix+1)
	res 4,(ix+1),a

	res 5,(ix+1),b
	res 5,(ix+1),c
	res 5,(ix+1),d
	res 5,(ix+1),e
	res 5,(ix+1),h
	res 5,(ix+1),l
	res 5,(ix+1)
	res 5,(ix+1),a

	res 6,(ix+1),b
	res 6,(ix+1),c
	res 6,(ix+1),d
	res 6,(ix+1),e
	res 6,(ix+1),h
	res 6,(ix+1),l
	res 6,(ix+1)
	res 6,(ix+1),a

	res 7,(ix+1),b
	res 7,(ix+1),c
	res 7,(ix+1),d
	res 7,(ix+1),e
	res 7,(ix+1),h
	res 7,(ix+1),l
	res 7,(ix+1)
	res 7,(ix+1),a

	set 0,(ix+1),b
	set 0,(ix+1),c
	set 0,(ix+1),d
	set 0,(ix+1),e
	set 0,(ix+1),h
	set 0,(ix+1),l
	set 0,(ix+1)
	set 0,(ix+1),a

	set 1,(ix+1),b
	set 1,(ix+1),c
	set 1,(ix+1),d
	set 1,(ix+1),e
	set 1,(ix+1),h
	set 1,(ix+1),l
	set 1,(ix+1)
	set 1,(ix+1),a

	set 2,(ix+1),b
	set 2,(ix+1),c
	set 2,(ix+1),d
	set 2,(ix+1),e
	set 2,(ix+1),h
	set 2,(ix+1),l
	set 2,(ix+1)
	set 2,(ix+1),a

	set 3,(ix+1),b
	set 3,(ix+1),c
	set 3,(ix+1),d
	set 3,(ix+1),e
	set 3,(ix+1),h
	set 3,(ix+1),l
	set 3,(ix+1)
	set 3,(ix+1),a

	set 4,(ix+1),b
	set 4,(ix+1),c
	set 4,(ix+1),d
	set 4,(ix+1),e
	set 4,(ix+1),h
	set 4,(ix+1),l
	set 4,(ix+1)
	set 4,(ix+1),a

	set 5,(ix+1),b
	set 5,(ix+1),c
	set 5,(ix+1),d
	set 5,(ix+1),e
	set 5,(ix+1),h
	set 5,(ix+1),l
	set 5,(ix+1)
	set 5,(ix+1),a

	set 6,(ix+1),b
	set 6,(ix+1),c
	set 6,(ix+1),d
	set 6,(ix+1),e
	set 6,(ix+1),h
	set 6,(ix+1),l
	set 6,(ix+1)
	set 6,(ix+1),a

	set 7,(ix+1),b
	set 7,(ix+1),c
	set 7,(ix+1),d
	set 7,(ix+1),e
	set 7,(ix+1),h
	set 7,(ix+1),l
	set 7,(ix+1)
	set 7,(ix+1),a

iycb
	rlc (iy+1),b
	rlc (iy+1),c
	rlc (iy+1),d
	rlc (iy+1),e
	rlc (iy+1),h
	rlc (iy+1),l
	rlc (iy+1)
	rlc (iy+1),a
	rrc (iy+1),b
	rrc (iy+1),c
	rrc (iy+1),d
	rrc (iy+1),e
	rrc (iy+1),h
	rrc (iy+1),l
	rrc (iy+1)
	rrc (iy+1),a

	rl (iy+1),b
	rl (iy+1),c
	rl (iy+1),d
	rl (iy+1),e
	rl (iy+1),h
	rl (iy+1),l
	rl (iy+1)
	rl (iy+1),a
	rr (iy+1),b
	rr (iy+1),c
	rr (iy+1),d
	rr (iy+1),e
	rr (iy+1),h
	rr (iy+1),l
	rr (iy+1)
	rr (iy+1),a

	sla (iy+1),b
	sla (iy+1),c
	sla (iy+1),d
	sla (iy+1),e
	sla (iy+1),h
	sla (iy+1),l
	sla (iy+1)
	sla (iy+1),a
	sra (iy+1),b
	sra (iy+1),c
	sra (iy+1),d
	sra (iy+1),e
	sra (iy+1),h
	sra (iy+1),l
	sra (iy+1)
	sra (iy+1),a

	sll (iy+1),b
	sll (iy+1),c
	sll (iy+1),d
	sll (iy+1),e
	sll (iy+1),h
	sll (iy+1),l
	sll (iy+1)
	sll (iy+1),a
	srl (iy+1),b
	srl (iy+1),c
	srl (iy+1),d
	srl (iy+1),e
	srl (iy+1),h
	srl (iy+1),l
	srl (iy+1)
	srl (iy+1),a

	bit 0,(iy+1)
	bit 0,(iy+1)
	bit 0,(iy+1)
	bit 0,(iy+1)
	bit 0,(iy+1)
	bit 0,(iy+1)
	bit 0,(iy+1)
	bit 0,(iy+1)
	bit 1,(iy+1)
	bit 1,(iy+1)
	bit 1,(iy+1)
	bit 1,(iy+1)
	bit 1,(iy+1)
	bit 1,(iy+1)
	bit 1,(iy+1)
	bit 1,(iy+1)

	bit 2,(iy+1)
	bit 2,(iy+1)
	bit 2,(iy+1)
	bit 2,(iy+1)
	bit 2,(iy+1)
	bit 2,(iy+1)
	bit 2,(iy+1)
	bit 2,(iy+1)

	bit 3,(iy+1)
	bit 3,(iy+1)
	bit 3,(iy+1)
	bit 3,(iy+1)
	bit 3,(iy+1)
	bit 3,(iy+1)
	bit 3,(iy+1)
	bit 3,(iy+1)

	bit 4,(iy+1)
	bit 4,(iy+1)
	bit 4,(iy+1)
	bit 4,(iy+1)
	bit 4,(iy+1)
	bit 4,(iy+1)
	bit 4,(iy+1)
	bit 4,(iy+1)

	bit 5,(iy+1)
	bit 5,(iy+1)
	bit 5,(iy+1)
	bit 5,(iy+1)
	bit 5,(iy+1)
	bit 5,(iy+1)
	bit 5,(iy+1)
	bit 5,(iy+1)

	bit 6,(iy+1)
	bit 6,(iy+1)
	bit 6,(iy+1)
	bit 6,(iy+1)
	bit 6,(iy+1)
	bit 6,(iy+1)
	bit 6,(iy+1)
	bit 6,(iy+1)

	bit 7,(iy+1)
	bit 7,(iy+1)
	bit 7,(iy+1)
	bit 7,(iy+1)
	bit 7,(iy+1)
	bit 7,(iy+1)
	bit 7,(iy+1)
	bit 7,(iy+1)

	res 0,(iy+1),b
	res 0,(iy+1),c
	res 0,(iy+1),d
	res 0,(iy+1),e
	res 0,(iy+1),h
	res 0,(iy+1),l
	res 0,(iy+1)
	res 0,(iy+1),a

	res 1,(iy+1),b
	res 1,(iy+1),c
	res 1,(iy+1),d
	res 1,(iy+1),e
	res 1,(iy+1),h
	res 1,(iy+1),l
	res 1,(iy+1)
	res 1,(iy+1),a

	res 2,(iy+1),b
	res 2,(iy+1),c
	res 2,(iy+1),d
	res 2,(iy+1),e
	res 2,(iy+1),h
	res 2,(iy+1),l
	res 2,(iy+1)
	res 2,(iy+1),a

	res 3,(iy+1),b
	res 3,(iy+1),c
	res 3,(iy+1),d
	res 3,(iy+1),e
	res 3,(iy+1),h
	res 3,(iy+1),l
	res 3,(iy+1)
	res 3,(iy+1),a

	res 4,(iy+1),b
	res 4,(iy+1),c
	res 4,(iy+1),d
	res 4,(iy+1),e
	res 4,(iy+1),h
	res 4,(iy+1),l
	res 4,(iy+1)
	res 4,(iy+1),a

	res 5,(iy+1),b
	res 5,(iy+1),c
	res 5,(iy+1),d
	res 5,(iy+1),e
	res 5,(iy+1),h
	res 5,(iy+1),l
	res 5,(iy+1)
	res 5,(iy+1),a

	res 6,(iy+1),b
	res 6,(iy+1),c
	res 6,(iy+1),d
	res 6,(iy+1),e
	res 6,(iy+1),h
	res 6,(iy+1),l
	res 6,(iy+1)
	res 6,(iy+1),a

	res 7,(iy+1),b
	res 7,(iy+1),c
	res 7,(iy+1),d
	res 7,(iy+1),e
	res 7,(iy+1),h
	res 7,(iy+1),l
	res 7,(iy+1)
	res 7,(iy+1),a

	set 0,(iy+1),b
	set 0,(iy+1),c
	set 0,(iy+1),d
	set 0,(iy+1),e
	set 0,(iy+1),h
	set 0,(iy+1),l
	set 0,(iy+1)
	set 0,(iy+1),a

	set 1,(iy+1),b
	set 1,(iy+1),c
	set 1,(iy+1),d
	set 1,(iy+1),e
	set 1,(iy+1),h
	set 1,(iy+1),l
	set 1,(iy+1)
	set 1,(iy+1),a

	set 2,(iy+1),b
	set 2,(iy+1),c
	set 2,(iy+1),d
	set 2,(iy+1),e
	set 2,(iy+1),h
	set 2,(iy+1),l
	set 2,(iy+1)
	set 2,(iy+1),a

	set 3,(iy+1),b
	set 3,(iy+1),c
	set 3,(iy+1),d
	set 3,(iy+1),e
	set 3,(iy+1),h
	set 3,(iy+1),l
	set 3,(iy+1)
	set 3,(iy+1),a

	set 4,(iy+1),b
	set 4,(iy+1),c
	set 4,(iy+1),d
	set 4,(iy+1),e
	set 4,(iy+1),h
	set 4,(iy+1),l
	set 4,(iy+1)
	set 4,(iy+1),a

	set 5,(iy+1),b
	set 5,(iy+1),c
	set 5,(iy+1),d
	set 5,(iy+1),e
	set 5,(iy+1),h
	set 5,(iy+1),l
	set 5,(iy+1)
	set 5,(iy+1),a

	set 6,(iy+1),b
	set 6,(iy+1),c
	set 6,(iy+1),d
	set 6,(iy+1),e
	set 6,(iy+1),h
	set 6,(iy+1),l
	set 6,(iy+1)
	set 6,(iy+1),a

	set 7,(iy+1),b
	set 7,(iy+1),c
	set 7,(iy+1),d
	set 7,(iy+1),e
	set 7,(iy+1),h
	set 7,(iy+1),l
	set 7,(iy+1)
	set 7,(iy+1),a

