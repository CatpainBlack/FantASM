DOS_VERSION        equ 0x0103 ;Get +3DOS issue and version numbers
DOS_OPEN           equ 0x0106 ;Create and/or open a file
DOS_CLOSE          equ 0x0109 ;Close a file
DOS_ABANDON        equ 0x010C ;Abandon a file
DOS_REF_HEAD       equ 0x010F ;Point at the header data for this file
DOS_READ           equ 0x0112 ;Read bytes into memory
DOS_WRITE          equ 0x0115 ;Write bytes from memory
DOS_BYTE_READ      equ 0x0118 ;Read a byte
DOS_BYTE_WRITE     equ 0x011B ;Write a byte
DOS_CATALOG        equ 0x011E ;Catalog disk directory
DOS_FREE_SPACE     equ 0x0121 ;Free space on disk
DOS_DELETE         equ 0x0124 ;Delete a file
DOS_RENAME         equ 0x0127 ;Rename a file
DOS_BOOT           equ 0x012A ;Boot an operating system or other program
DOS_SET_DRIVE      equ 0x012D ;Set/get default drive
DOS_SET_USER       equ 0x0130 ;Set/get default user number
DOS_GET_POSITION   equ 0x0133 ;Get file pointer for random access
DOS_SET_POSITION   equ 0x0136 ;Set file pointer for random access
DOS_GET_EOF        equ 0x0139 ;Get end of file position for random access
DOS_GET_1346       equ 0x013C ;Get memory usage in pages 1, 3, 4, 6
DOS_SET_1346       equ 0x013F ;Re-allocate memory usage in pages 1, 3, 4, 6
DOS_FLUSH          equ 0x0142 ;Bring disk up to date
DOS_SET_ACCESS     equ 0x0145 ;Change open file's access mode
DOS_SET_ATTRIBUTES equ 0x0148 ;Change a file's attributes
DOS_SET_MESSAGE    equ 0x014E ;Enable/disable error messages

IDE_VERSION        equ 0x00A0 ;Get IDEDOS version number
IDE_SWAP_OPEN      equ 0x00D9 ;Open a swap partition
IDE_SWAP_CLOSE     equ 0x00DC ;Close a swap partition
IDE_SWAP_OUT       equ 0x00DF ;Write block to swap partition
IDE_SWAP_IN        equ 0x00E2 ;Read block from swap partition
IDE_SWAP_EX        equ 0x00E5 ;Exchange block with swap partition
IDE_SWAP_POS       equ 0x00E8 ;Get current block number in swap partition
IDE_SWAP_MOVE      equ 0x00EB ;Set current block number in swap partition
IDE_SWAP_RESIZE    equ 0x00EE ;Change block size of swap partition
IDE_PARTITION_FIND equ 0x00B5 ;Find named partition
IDE_DOS_MAP        equ 0x00F1 ;Map drive to partition
IDE_DOS_UNMAP      equ 0x00F4 ;Unmap drive
IDE_DOS_MAPPING    equ 0x00F7 ;Get drive mapping
IDE_SNAPLOAD       equ 0x00FD ;Load a snapshot
IDE_PATH           equ 0x01b1 ;Create, delete, change or get directory
IDE_CAPACITY       equ 0x01b4 ;Get card capacity
IDE_GET_LFN        equ 0x01b7 ;Get long filename
IDE_BROWSER        equ 0x01ba ;File browser
IDE_MOUNT          equ 0x01d2 ;Unmount/remount SD cards

IDE_STREAM_OPEN   equ 0x0056 ;Open stream to a channel
IDE_STREAM_CLOSE  equ 0x0059 ;Close stream and attached channel
IDE_STREAM_IN     equ 0x005c ;Get byte from current stream
IDE_STREAM_OUT    equ 0x005f ;Write byte to current stream
IDE_STREAM_PTR    equ 0x0062 ;Get or set pointer information for current stream
IDE_BANK          equ 0x01bd ;Allocate or free 8K banks in ZX or DivMMC memory
IDE_BASIC         equ 0x01c0 ;Execute a BASIC command line
IDE_WINDOW_LINEIN equ 0x01c3 ;Input line from current window stream
IDE_WINDOW_STRING equ 0x01c6 ;Output string to current window stream
IDE_INTEGER_VAR   equ 0x01c9 ;Get or set NextBASIC integer variable
IDE_RTC           equ 0x01cc ;Query the real-time-clock module
IDE_DRIVER        equ 0x01cf ;Access the driver API
IDE_MODE          equ 0x01d5 ;Query NextBASIC display mode info, or change mode
IDE_TOKENISER     equ 0x01d8 ;Convert BASIC between plain text & tokenised forms

;The following API calls are present but generally for system use only and not useful for games/applications
DOS_INITIALISE      equ 0x0100 ;Initialise +3DOS
IDE_INTERFACE       equ 0x00A3 ;Initialise card interfaces
IDE_INIT            equ 0x00A6 ;Initialise IDEDOS
IDE_DRIVE           equ 0x00A9 ;Get unit handle
IDE_SECTOR_READ     equ 0x00AC ;Low-level sector read
IDE_SECTOR_WRITE    equ 0x00AF ;Low-level sector write
IDE_PARTITON_READ   equ 0x00C4 ;Read a partition entry
IDE_PARTITION_OPEN  equ 0x00CD ;Open a partition
IDE_PARTITION_CLOSE equ 0x00D0 ;Close a partition
IDE_PARTITIONS      equ 0x01a5 ;Get number of open partitions

; Error Codes
rc_ready   equ 0 ;Drive not ready
rc_wp      equ 1 ;Disk is write protected
rc_seek    equ 2 ;Seek fail
rc_crc     equ 3 ;CRC data error
rc_nodata  equ 4 ;No data
rc_mark    equ 5 ;Missing address mark
rc_unrecog equ 6 ;Unrecognised disk format
rc_unknown equ 7 ;Unknown disk error
rc_diskchg equ 8 ;Disk changed whilst +3DOS was using it
rc_unsuit  equ 9 ;Unsuitable media for drive

; Non-recoverable errors
rc_badname      equ 20 ;Bad filename
rc_badparam     equ 21 ;Bad parameter
rc_nodrive      equ 22 ;Drive not found
rc_nofile       equ 23 ;File not found
rc_exists       equ 24 ;File already exists
rc_eof          equ 25 ;End of file
rc_diskfull     equ 26 ;Disk full
rc_dirfull      equ 27 ;Directory full
rc_ro           equ 28 ;Read-only file
rc_number       equ 29 ;File number not open (or open with wrong access)
rc_denied       equ 30 ;Access denied
rc_norename     equ 31 ;Cannot rename between drives
rc_extent       equ 32 ;Extent missing
rc_uncached     equ 33 ;Uncached
rc_toobig       equ 34 ;File too big
rc_notboot      equ 35 ;Disk not bootable
rc_inuse        equ 36 ;Drive in use
rc_invpartition equ 56 ;Invalid partition
rc_partexist    equ 57 ;Partition already exists
rc_notimp       equ 58 ;Not implemented
rc_partopen     equ 59 ;Partition open
rc_nohandle     equ 60 ;Out of handles
rc_notswap      equ 61 ;Not a swap partition
rc_mapped       equ 62 ;Drive already mapped
rc_noxdpb       equ 63 ;No XDPB
rc_noswap       equ 64 ;No suitable swap partition
rc_invdevice    equ 65 ;Invalid device
rc_cmdphase     equ 67 ;Command phase error
rc_dataphase    equ 68 ;Data phase error
rc_notdir       equ 69 ;Not a directory
rc_fragmented   equ 74 ;File is fragmented, use .DEFRAG
