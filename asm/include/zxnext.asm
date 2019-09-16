I2C_clock	                      equ 0x103B	;%0001 0000 0011 1011 ??	Sets and reads the I2C SCL line.
I2C_data	                      equ 0x113B	;%0001 0001 0011 1011 ??	Sets and reads the I2C SDA line
Layer_2_Access_Port	              equ 0x123B	;%0001 0010 0011 1011 ??	Enables Layer 2 and controls paging of layer 2 screen into lower memory.
UART_TX	                          equ 0x133B	;%0001 0011 0011 1011 ??	Sends byte to serial port. If read, tells if data in RX buffer
UART_RX	                          equ 0x143B	;%0001 0100 0011 1011 ??	Reads data from serial port, write sets the baudrate
Plus_3_Memory_Paging_Control	  equ 0x1FFD	;%0001 ---- ---- --0-	    Controls ROM paging and special paging options from the +2a/+3.
TBBlue_Select	                  equ 0x243B	;%0010 0100 0011 1011	    Selects active port for TBBlue/Next feature configuration.
TBBlue_Access	                  equ 0x253B	;%0010 0101 0011 1011	    Reads and/or writes the selected TBBlue control register.
Sprite_Status_Slot_Select	      equ 0x303B	;%0011 0000 0011 1011 ??	Sets active sprite-attribute index and pattern-slot index, reads sprite status.
Memory_Paging_Control	          equ 0x7FFD	;%01-- ---- ---- --0-	    Selects active RAM, ROM, and displayed screen.
Sound_Chip_Write	              equ 0xBFFD	;%10-- ---- ---- --0-	    Writes to the selected register of the selected sound chip.
Next_Memory_Bank_Select	          equ 0xDFFD	;%1101 1111 1111 1101	    Provides additional bank select bits for extended memory.
Kempston_Mouse_Buttons	          equ 0xFADF	;%---- ---0 --0- ----	    Reads buttons on Kempston Mouse.
Kempston_Mouse_X	              equ 0xFBDF	;%---- -0-1 --0- ----	    X coordinate of Kempston Mouse, 0-255.
Kempston_Mouse_Y	              equ 0xFFDF	;%---- -1-1 --0- ----	    Y coordinate of Kempston Mouse, 0-192.
Turbo_Sound_Next_Control	      equ 0xFFFD	;%11-- ---- ---- --0-	    Controls stereo channels and selects active sound chip and sound chip channel.
MB02_DMA_Port	                  equ 0x000B	;	                        Controls Z8410 DMA chip via MB02 standard.
Kempston_Joystick	              equ 0x001F	;%---- ---- 0001 1111	    Reads movement of joysticks using Kempston interface.
Kempston_Joystick_2	              equ 0x0037	;%---- ---- 0011 0111	    Reads movement of joystick using Kempston interface (second joystick variant).
Sprite_Attribute_Upload	          equ 0x0057	;%---- ---- 0101 0111	    Uploads sprite positions, visibility, colour type and effect flags.
Sprite_Pattern_Upload	          equ 0x005B	;%---- ---- 0101 1011 ??	Used to upload the pattern of the selected sprite.
Datagear_DMA_Port                 equ 0x006B	;---- ---- 0110 1011	    Controls Z8410 DMA chip via Datagear standard.
SpecDrum_DAC_Output               equ 0x00DF	;%---- ---- --01 1111	    Output to SpecDrum DAC.
ULA_Control_Port                  equ 0x00FE	;%---- ---- ---- ---0	    Controls border color and base Spectrum audio settings.
Timex_Sinclair_Video_Mode_Control equ 0x00FF	;%---- ---- 1111 1111	    Controls Timex Sinclair video modes and colours in hi-res mode.


; Next control registers
Turbo_Control	                    equ 0x07	; Yes   Yes   Sets accelerated clock speed, reads actual speed.
    ;Bit 1-0 equ turbo: %00 equ 3.5MHz, %01 equ 7MHz, %10 equ 14MHz (%00 after PoR or Hard-reset).
    TURBO_NORMAL    equ 0
    TURBO_7MHZ      equ 1
    TURBO_14MHZ     equ 2

Machine_ID	                        equ 0x00	; Yes   No    Identifies TBBlue board type. Should always be 10 on Next.
    Terasic_Altera_DE1          equ 1
    Terasic_Altera_DE2          equ 2
    FBLabs                      equ 5
    VTrucco                     equ 6
    WXEDA                       equ 7
    Emulator                    equ 8
    ZXUNO                       equ 9
    Spectrum_Next               equ 10
    Multicore                   equ 11
    ZX_Spectrum_Next_Anti_brick equ 250

Next_Version	                    equ 0x01	; Yes   No    Identifies FPGA image version.
Next_Version_sub_minor	            equ 0x0E	; Yes   No    Identifies FPGA image version (sub minor number).

Next_Reset	                        equ 0x02	; Yes   Yes   Identifies type of last reset. Can be written to force reset.
    SOFT        equ 0
    HARD        equ 1
    POWER_ON    equ 2 ; Read only

Machine_Type	                    equ 0x03	; Yes   Yes   Identifies timing and machine type.
    ;%000: Config mode
    ;%001: ZX 48k
    ;%010: ZX 128k/+2 (Grey)
    ;%011: ZX +2A-B/+3e/Next Native
    ;%100: Pentagon.

ROM_mapping	                        equ 0x04	; No    Yes   In config mode, allows RAM to be mapped to ROM area.

Peripheral1	                        equ 0x05	; Yes   Yes   Sets joystick mode, video frequency and Scandoubler.
Peripheral2	                        equ 0x06	; Yes   Yes   Enables Acceleration, Lightpen, DivMMC, Multiface, Mouse and AY audio.
Peripheral3	                        equ 0x08	; Yes   Yes   ABC/ACB Stereo, Internal Speaker, SpecDrum, Timex Video Modes, Turbo Sound Next, RAM contention and [un]lock 128k paging.
Peripheral4	                        equ 0x09	; Yes   Yes   Sets scanlines, AY mono output, Sprite-id lockstep, disables Kempston and divMMC ports.

Anti_brick	                        equ 0x10	; Yes   Yes   Used within the Anti-brick system.
Video_Timing	                    equ 0x11	; Yes   Yes   Sets video output timing variant.

Global_Transparency	                equ 0x14	; Yes   Yes   Sets the "transparent" colour for Layer 2, ULA and LoRes pixel data.

L2_RAM_Page                         equ 0x12	; Yes   Yes   Sets the bank number where Layer 2 video memory begins.
L2_Shadow_Page	                    equ 0x13	; Yes   Yes   Sets the bank number where the Layer 2 shadow screen begins.
L2_XOffset	                        equ 0x16	; Yes   Yes   Sets the pixel offset used for drawing Layer 2 graphics on the screen.
L2_YOffset	                        equ 0x17	; Yes   Yes   Sets the Y offset used when drawing Layer 2 graphics on the screen.

Sprite_Control	                    equ 0x15	; Yes   Yes   Enables/disables Sprites and Lores Layer, and chooses priority of sprites and Layer 2.

Clip_Window_L2	                    equ 0x18	; Yes   Yes   Sets and reads clip-window for Layer 2.
Clip_Window_Sprites	                equ 0x19	; Yes   Yes   Sets and reads clip-window for Sprites
Clip_Window_ULA_LoRes	            equ 0x1A	; Yes   Yes   Sets and reads clip-window for ULA/LoRes layer.
Clip_Window_Control	                equ 0x1C	; Yes   Yes   Controls (resets) the clip-window registers indices.

Raster_Line_MSB	                    equ 0x1E	; Yes   No    Holds the MSB (only, as bit 0) of the raster line currently being drawn.
Raster_Line_LSB	                    equ 0x1F	; Yes   No    Holds the eight LSBs of the raster line currently being drawn.
Raster_Interrupt_Control	        equ 0x22	; Yes   Yes   Controls the timing of raster interrupts and the ULA frame interrupt.
Raster_Interrupt_Value	            equ 0x23	; Yes   Yes   Holds the eight LSBs of the line on which a raster interrupt should occur.

Keymap_High_Address	                equ 0x28	; No    Yes   High address of Keymap (MSB in bit 0)
Keymap_Low_Address	                equ 0x29	; No    Yes   Low eight LSBs of Keymap address.
Keymap_High_Data	                equ 0x2A	; No    Yes   High data to Keymap (MSB of data in bit 0)
Keymap_Low_Data	                    equ 0x2B	; No    Yes   Low eight LSBs of Keymap data.

SoundDrive_port_0xDF_mirror	        equ 0x2D	; No    Yes   SpecDrum port 0xDF / DAC A+C mirror.

ULA_LoRes_Layer_X_Offset	        equ 0x32	; Yes   Yes   Pixel offset to use when drawing ULA or LoRes Layer.
ULA_LoRes_Layer_Y_Offset	        equ 0x33	; Yes   Yes   Pixel offset to use when drawing ULA or LoRes Layer.

Sprite_port_mirror_Index	        equ 0x34	; Yes   Yes   Selects sprite index 0..127 to be affected by writes to other Sprite ports (and mirrors).
Sprite_port_mirror_Attribute_0	    equ 0x35	; No    Yes   Nextreg port-mirror to write directly into "byte 1" of Sprite Attribute Upload ($xx57).
Sprite_port_mirror_Attribute_1	    equ 0x36	; No    Yes   Nextreg port-mirror to write directly into "byte 2" of Sprite Attribute Upload ($xx57).
Sprite_port_mirror_Attribute_2	    equ 0x37	; No    Yes   Nextreg port-mirror to write directly into "byte 3" of Sprite Attribute Upload ($xx57).
Sprite_port_mirror_Attribute_3	    equ 0x38	; No    Yes   Nextreg port-mirror to write directly into "byte 4" of Sprite Attribute Upload ($xx57).
Sprite_port_mirror_Attribute_4	    equ 0x39	; No    Yes   Nextreg port-mirror to write directly into "byte 5" of Sprite Attribute Upload ($xx57).

Palette_Index	                    equ 0x40	; Yes   Yes   Chooses an palette element (index) to manipulate with
Palette_Value	                    equ 0x41	; Yes   Yes   Use to set/read 8-bit colours of the ULANext palette.

Enhanced_ULA_Ink_Color_Mask	        equ 0x42	; Yes   Yes   Specifies mask to extract ink colour from attribute cell value in ULANext mode.
Enhanced_ULA_Control	            equ 0x43	; Yes   Yes   Enables or disables Enhanced ULA interpretation of attribute values and toggles active palette.
Enhanced_ULA_Palette_Extension	    equ 0x44	; Yes   Yes   Use to set 9-bit (2-byte) colours of the Enhanced ULA palette, or to read second byte of colour.
Transparency_colour_fallback	    equ 0x4A	; Yes   Yes   8-bit colour to be used when all layers contain transparent pixel.
Sprites_Transparency_Index	        equ 0x4B	; Yes   Yes   Index into sprite palette (of "transparent" colour).

MMU0                        	    equ 0x50	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 0 (see Memory map).
MMU1                        	    equ 0x51	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 1 (see Memory map).
MMU2                        	    equ 0x52	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 2 (see Memory map).
MMU3                        	    equ 0x53	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 3 (see Memory map).
MMU4                        	    equ 0x54	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 4 (see Memory map).
MMU5                        	    equ 0x55	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 5 (see Memory map).
MMU6                        	    equ 0x56	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 6 (see Memory map).
MMU7                        	    equ 0x57	; Yes   Yes   Selects the 8k-bank stored in 8k-slot 7 (see Memory map).

Copper_Data	                        equ 0x60	; No    Yes   Used to upload code to the Copper.
Copper_Control_Low_Byte	            equ 0x61	; No    Yes   Holds low byte of Copper control bits.
Copper_Control_High_Byte	        equ 0x62	; No    Yes   Holds high byte of Copper control flags.

ULA_Control	                        equ 0x68	; Yes   Yes   Disables ULA completely and controls its mixing with Tilemap layer.
Sprite_port_mirror_Attribute_0_Inc	equ 0x75	; No    Yes   Same as Sprite port-mirror Attribute 0 Register ($35) (write first byte of sprite-attributes), plus increments Sprite port-mirror Index Register ($34)
Sprite_port_mirror_Attribute_1_Inc	equ 0x76	; No    Yes   Same as Sprite port-mirror Attribute 1 Register ($36) (write second byte of sprite-attributes), plus increments Sprite port-mirror Index Register ($34)
Sprite_port_mirror_Attribute_2_Inc	equ 0x77	; No    Yes   Same as Sprite port-mirror Attribute 2 Register ($37) (write third byte of sprite-attributes), plus increments Sprite port-mirror Index Register ($34)
Sprite_port_mirror_Attribute_3_Inc	equ 0x78	; No    Yes   Same as Sprite port-mirror Attribute 3 Register ($38) (write fourth byte of sprite-attributes), plus increments Sprite port-mirror Index Register ($34)
Sprite_port_mirror_Attribute_4_Inc	equ 0x79	; No    Yes   The same as Sprite port-mirror Attribute 4 Register ($39) (write fifth byte of sprite-attributes), plus increments Sprite port-mirror Index Register ($34)
Debug_LED_Control	                equ 0xFF	; No    Yes   Turns debug LEDs on and off on TBBlue implementations that have them.

