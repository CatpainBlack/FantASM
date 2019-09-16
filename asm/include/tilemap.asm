TM_Control  equ 0x6B	; Controls Tilemap mode.
    TM_DISABLED equ 0x00 ;bit 7: 0=disable tilemap
    TM_ENABLED  equ 0x80 ;       1=enable tilemap
    TM_40X32    equ 0x00 ;bit 6: 0=40x32
    TM_80X32    equ 0x40 ;       1=80x32
    TM_ATTR     equ 0x00 ;bit 5: 0=attributes in tilemap
    TM_NO_ATTR  equ 0x20 ;       1=no attributes in tilemap
    TM_PAL1     equ 0x00 ;bit 4: 0=primary palette
    TM_PAL2     equ 0x10 ;       1=secondary palette
                            ;bits 3-2: reserved (0)
    TM_256      equ 0x00 ;bit 1: 0=256 tile mode
    TM_512      equ 0x02 ;       1=512 tile mode
    TM_OVER_ULA equ 0x01 ;bit 0: 1=tilemap over ULA

TM_Data         equ 0x6E	; Base address of the 40x32 or 80x32 tile map (similar to text-mode of other computers).
TM_Definitions	equ 0x6F	; Base address of the tiles' graphics.
TM_Attribute    equ 0x6C	; Default tile attribute for 8-bit only maps.
                            ;7-4 	Palette Offset
    TM_MIRROR_X equ 0x04  ; 3 X mirror
    TM_MIRROR_Y equ 0x02  ; 2 Y mirror
    TM_ROTATE   equ 0x01  ; 1 Rotate
                            ;0 	If in 512-tile-mode: bit 8 of tile-id
                            ;else draw priority: 1 = ULA over tilemap, 0 = tilemap over ULA

TM_Clip             equ 0x1B	; Sets and reads clip-window for Tilemap.
    ; bits 7-0: Coord. of the clip window
    ;  The values are 0,159,0,255 after a Reset
    ;  Reads do not advance the clip position
    MACRO tm_clip x1,x2,y1,y2
        nextreg TM_Clip,x1 ;  1st write = X1 position
        nextreg TM_Clip,x2 ;  2nd write = X2 position
        nextreg TM_Clip,y1 ;  3rd write = Y1 position
        nextreg TM_Clip,y2 ;  4rd write = Y2 position
    ENDM


TM_Offset_X_MSB	            equ 0x2F	; Sets the pixel offset (two high bits) used for drawing Tilemap graphics on the screen.
TM_Offset_X_LSB	            equ 0x30	; Sets the pixel offset (eight low bits) used for drawing Tilemap graphics on the screen.
TM_Offset_Y	                equ 0x31	; Sets the pixel offset used for drawing Tilemap graphics on the screen.
TM_Transparency_Index	    equ 0x4C	; Index into Tilemap palette (of "transparent" colour).
