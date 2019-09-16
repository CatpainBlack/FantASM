; Low-level calls
; ***************************************************************************
; * DISK_FILEMAP ($85)                                                      *
; ***************************************************************************
; Obtain a map of card addresses describing the space occupied by the file.
; Can be called multiple times if buffer is filled, continuing from previous. ; Entry:
;       A=file handle (just opened, or following previous DISK_FILEMAP calls)
;       IX [HL from dot command]=buffer (must be located >= $4000)
;       DE=max entries (each 6 bytes: 4 byte address, 2 byte sector count)
; Exit (success):
;       Fc=0 ;       DE=max entries-number of entries returned
;       HL=address in buffer after last entry
;       A=card flags: bit 0=card id (0 or 1)
;                     bit 1=0 for byte addressing, 1 for block addressing
; Exit (failure):
;       Fc=1
;       A=error
;
; NOTES:
; Each entry may describe an area of the file between 2K and just under 32MB
; in size, depending upon the fragmentation and disk format. ;
; If the file has been accessed, the filepointer should be reset to the start
; using F_SEEK, and a single byte read (with F_READ) before making this call.
; This will ensure that the current sector information maintained by the OS
; is correctly pointing to the first sector of the file.
;
; The provided buffer address must be >=$4000 (ie dot commands will need to
; allocate space in the main RAM area using the BC_SPACES call in the 48K
; ROM, or page in an allocated bank).
;
; If you wish to check whether a file is unfragmented, there are 2 ways:
;   (1) for files < ~32MB in size, make a call to DISK_FILEMAP. If there is
;       only 1 entry (ie exit HL=entry HL+6), the file is unfragmented
;   (2) for files > ~32MB in size, you must manually check whether each
;       section of the file directly follows the previous one. The .DEFRAG
;       dot command contains appropriate code for this that you may wish to
;       use: please see the source in src/asm/dot_commands/defrag.asm on the
;  standard SD card distribution.
;
; Please see example application code, stream.asm, for full usage information
; (available separately or at the end of this document).
disk_filemap   equ 0x85 ; $85 (133) obtain file allocation map

; ***************************************************************************
; * DISK_STRMSTART $86
; ***************************************************************************
; Start reading from the card in streaming mode.
; Entry: IXDE [HLDE from dot command]=card address
;        BC=number of 512-byte blocks to stream
;        A=card flags
;        additionally, from NextZXOS v2.01, bit 7 may be set to indicate that
;        the user will perform the initial wait for data token
; Exit (success): Fc=0
;                 B=0 for SD/MMC protocol, 1 for IDE protocol
;                 C=8-bit data port
; Exit (failure): Fc=1, A=esx_edevicebusy
;
; NOTES:
;
; On the Next, this call always returns with B=0 (SD/MMC protocol) and C=$EB
; When streaming using the SD/MMC protocol, after every 512 bytes you must read
; a 2-byte CRC value (which can be discarded) and then wait for a $FE value
; indicating that the next block is ready to be read.
;
; On NextZXOS v2.01+, you may optionally set bit 7 of A to indicate that the
; call should return without waiting for the initial $FE data token, allowing
; other work to be done to cover the latency. In this case, the user must wait
; for the $FE token before any data is read from the stream.
;
; Please see example application code, stream.asm, for full usage information
; (available separately or at the end of this document).
disk_strmstart equ 0x86 ; $86 (134) start streaming operation

; ***************************************************************************
; * DISK_STRMEND ($87)                                                      *
; ***************************************************************************
; Stop current streaming operation.
; Entry: A=card flags
; Exit (success): Fc=0
; Exit (failure): Fc=1, A=esx_edevicebusy
;
; NOTES:
; This call must be made to terminate a streaming operation.
; Please see example application code, stream.asm, for full usage information
; (available separately or at the end of this document).
disk_strmend   equ 0x87 ; $87 (135) end streaming operation

; ***************************************************************************
; * M_DOSVERSION ($88)                                                      *
; ***************************************************************************
; Get API version/mode information.
; Entry:
;
; Exit:
;       For esxDOS <= 0.8.6
;               Fc=1, error
;               A=14 ("no such device")
;
;       For NextZXOS:
;               Fc=0, success
;               B='N',C='X' (NextZXOS signature)
;               DE=NextZXOS version in BCD format: D=major, E=minor version
;                  eg for NextZXOS v1.94, DE=$0194
;               HL=language code:
;                    English: L='e',H='n'
;                    Spanish: L='e',H='s'
;                    Further languages may be available in the future
;               A=0 if running in NextZXOS mode (and zero flag is set)
;               A<>0 if running in 48K mode (and zero flag is reset)    m_dosversion equ 0x88
; $88 (136) get NextZXOS version/mode information
m_dosversion equ 0x88 ; $88 (136) get NextZXOS version/mode information

; ***************************************************************************
; * M_GETSETDRV ($89)                                                       *
; ***************************************************************************
; Get or set the default drive.
; Entry:
;       A=0, get the default drive
;       A<>0, set the default drive to A
;             bits 7..3=drive letter (0=A...15=P)
;             bits 2..0=ignored (use 1 to ensure A<>0)
; Exit (success): ;       Fc=0
;       A=default drive, encoded as:
;             bits 7..3=drive letter (0=A...15=P)
;             bits 2..0=0 ; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTE:
; This call isn't often useful, as it is not necessary to provide a
; specific drive to calls which need a drive/filename.
; For such calls, you can instead provide:
;   A='*' use the default drive
;   A='$'   use the system drive (C:, where the NEXTZXOS and BIN dirs are)
; Any drive provided in such calls is also overridden by any drive letter
; that is specified in the filename (eg “D:/myfile.txt\0”).
;
; NOTE:
; When setting a drive, this call only affects the default drive seen by other
; esxDOS API calls. It does *not* change the default drive seen by +3DOS API
; calls, or the default LOAD/SAVE drives used by NextBASIC. This is because the
; RAM used to hold these defaults (RAM 7 and the system variables area) could
; potentially be being used for other purposes by programs using only the
; esxDOS API. ; If running in NextZXOS mode (not 48K mode) and you intend to use +3DOS API
; calls or return to NextBASIC, you can instead use the +3DOS DOS_SET_DRIVE
; call (which sets the default drive for +3DOS and esxDOS), and optionally
; change the LODDRV and SAVDRV system variables (affecting NextBASIC LOAD/SAVE).
m_getsetdrv  equ 0x89 ; $89 (137) get/set default drive

; ***************************************************************************
; * M_TAPEIN ($8b)                                                          *
; ***************************************************************************
; Tape input redirection control.
; Entry:
;       B=0, in_open:
;               Attach tap file with name at IX [HL from dot command],
;               drive in A
;       B=1, in_close:
;               Detach tap file
;       B=2, in_info:
;               Return attached filename to buffer at IX [HL from dot command],
;               and drive in A
;       B=3, in_setpos:
;               Set position of tape pointer to block DE (0=start)
;       B=4, in_getpos:
;               Get position of tape pointer, in blocks, to HL
;       B=5, in_pause:
;               Toggles pause delay when loading SCREEN$
;               On exit, A=1 if pause now enabled, A=0 if not
;       B=6, in_flags:
;               Set tape flags to A
;               bit 0: 1=pause delay at SCREEN$ (as set by in_pause)
;               bit 1: 1=simulate tape loading with border/sound
;               On exit, A=previous value of the tape flags
m_tapein     equ 0x8b ; $8b (139) tape redirection control (input)

; ***************************************************************************
; * M_TAPEOUT ($8c)                                                         *
; ***************************************************************************
; Tape output redirection control.
; Entry:
;       B=0, out_open:
;               Create/attach tap file with name at IX [HL from dot command]
;               for appending, drive A
;       B=1, out_close:
;               Detach tap file
;       B=2, out_info:
;               Return attached filename to buffer at IX [HL from dot command]
;               and drive in A
;       B=3, out_trunc:
;               Create/overwrite tap file with name at IX [HL from dot command],
;               drive A
m_tapeout    equ 0x8c ; $8c (140) tape redirection control (output)

; ***************************************************************************
; * M_GETHANDLE ($8d)                                                       *
; ***************************************************************************
; Get the file handle of the currently running dot command
; Entry:
;
; Exit:
;       A=handle
;       Fc=0
;
; NOTES:
; This call allows dot commands which are >8K to read further data direct
; from their own file (for loading into another memory area, or overlaying
; as required into the normal 8K dot command area currently in use).
; On entry to a dot command, the file is left open with the file pointer
; positioned directly after the first 8K.
; This call returns meaningless results if not called from a dot command.
m_gethandle  equ 0x8d ; $8d (141) get handle for current dot command

; ***************************************************************************
; * M_GETDATE ($8e)                                                         *
; ***************************************************************************
; Get the current date/time.
; Entry:
;
; Exit:
;       Fc=0 if RTC present and providing valid date/time, and:
;               BC=date, in MS-DOS format
;               DE=time, in MS-DOS format
;       Fc=1 if no RTC, or invalid date/time, and:
;               BC=0
;               DE=0
m_getdate    equ 0x8e ; $8e (142) get current date/time

; ***************************************************************************
; * M_EXECCMD ($8f)                                                         *
; ***************************************************************************
; Execute a dot command.
; Entry:
;       IX [HL from dot command]=address of commandline,
;                                excluding the leading "."
;                                terminated with $00 (or $0d, or ':')
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code (0 means user-defined error)
;       HL=address of user-defined error message within dot command
;
; NOTES:
; The dot command name can be fully-pathed if desired. If just a name is
; provided, it is opened from the C:/DOT directory.
;   eg: defm    "hexdump afile.txt",0
; runs c:/dot/hexdump
;       defm    "./mycommand.dot afile.txt",0
; runs mycommand.dot in current
;
; directory
; If A=0, the dot command has provided its own error message but this is not
; normally accessible. It can be read using the M_GETERR hook.
; This hook cannot be used from within another dot command.
m_execcmd    equ 0x8f ; $8f (143) execute a dot command

; ***************************************************************************
; * M_SETCAPS ($91)                                                         *
; ***************************************************************************
; Entry: A=capabilities to set:
;           bit 7=1, do not erase new file data in f_truncate/f_ftruncate
;                    (increases performance of these calls)
;           bits 0..6: reserved, must be zero
; Exit:  Fc=0, success
;        E=previous capabilities
;
; NOTE: This call is only available from NextZXOS v1.98M+.
;       Earlier versions will return with Fc=1 (error) and A=esx_enocmd
; NOTE: You should save the original value of the capabilities which is
;       returned in E. After completing the calls you need with your altered
;       capabilities, restore the original value by calling M_SETCAPS again
;       with the value that was previously returned in E.
;       This will ensure that other programs running after you have exited
;       will continue to see the original expected behaviour.
m_setcaps    equ 0x91 ; $91 (145) set additional capabilities

; ***************************************************************************
; * M_DRVAPI ($92)                                                          *
; ***************************************************************************
; Access API for installable drivers.
; Entry:
;       C=driver id (0=driver API)
;       B=call id
;       HL,DE=other parameters
; Exit (success):
;       Fc=0
;       other values depend on API call
; Exit (failure):
;       Fc=1
;       A=0, driver not found
;       else A=driver-specific error code (esxDOS error code for driver API)
; If C=0, the driver API is selected and calls are as follows:
; (Note that these are not really useful for user applications
; they are used
; by the .install/.uninstall dot commands).
;
; B=0, query the RTC
; (returns the same results as M_GETDATE)
;
; B=1, install a driver
;       D=number of relocations (0-255)
;       E=driver id, with bit 7=1 if should be called on an IM1 interrupt
;       HL=address of 512-byte driver code followed by D x 2-byte reloc offsets
; Possible error values are:
;       esx_eexist (18)         driver with same id already installed
;       esx_einuse (23)         no free driver slots available
;       esx_eloadingko (26)     bad relocation table
;
; B=2, uninstall a driver
;       E=driver id (bit 7 ignored)
;
; B=3, get paging value for driver banks
;       C=port (always $e3 on ZXNext)
;       A=paging value for DivMMC bank containing drivers (usually $82)
;
; B=4, get driver image
;       E=driver id (bit 7 ignored)
;       HL=address of 512-byte buffer
; NOTES:
; Any provided buffer addresses must be located >=$4000 since the lower 16K of
; memory is occupied with driver memory when this call is in operation.
m_drvapi     equ 0x92 ; $92 (146) access API for installable drivers

; ***************************************************************************
; * M_GETERR ($93)                                                          *
; ***************************************************************************
; Entry:
;       A=esxDOS error code, or 0=user defined error from dot command
;       if A=0, IX [HL from dot command]=error message address from dot command
;
;       B=0, generate BASIC error report (does not return)
;       B=1, return error message to 32-byte buffer at DE
;
; NOTES:
; Dot commands may use this call to fetch a standard esxDOS error message
; (with B=1), but must not use it to generate an error report (with B=0) as
; this would short-circuit the tidy-up code.
; User programs may use the call to generate any custom error message (and not
; just a custom message returned by a dot command). To do this, enter with
; A=0 and IX [HL from dot command]=address of custom message, >=$4000.
; Custom error messages must be terminated with bit 7 set on the final
; character.
m_geterr     equ 0x93 ; $93 (147) get or generate error message

; ***************************************************************************
; * M_P3DOS ($94)                                                           *
; ***************************************************************************
; Make a +3DOS/IDEDOS/NextZXOS API call.
; Entry:
;       DE=+3DOS/IDEDOS/NextZXOS call ID
;       C=RAM bank that needs to be paged (usually 7, but 0 for some calls)
;       B'C',D'E',H'L',AF,IX [HL from dot command] contain entry parameters
;       for call
; Exit:
;       exit values as described for +3DOS/IDEDOS/NextZXOS call ID
;       EXCEPT: any value to be returned in IX will instead be in H'L'
;       All registers except IX,IY may be changed.
;
; NOTES:
;
; B'C', D'E', H'L' contain the entry parameters that the +3DOS API call
; expects to be in BC, DE, HL.
;
; As with other esxDOS API calls, any IX entry parameter should instead be
; loaded into HL if making the call from within a dot command.
;
; Do not attempt to use this hook code unless you are running in NextZXOS mode
; (can be determined by using the M_DOSVERSION hook).
;
; Any parameters which are addresses of data (eg filenames etc) must lie between
; $4000...$BFE0.
;
; Any errors returned will be +3DOS/IDEDOS/NextZXOS error codes, not esxDOS
; error codes. Additionally, carry flag RESET indicates an error condition.
;
; No $DFFD paging should be in force.
;
; MMU2 ($4000-$5fff) must be the default (lower half of RAM bank 5), containing
; the system variables.
;
; The stack should be in normal configuration (not in TSTACK). For calls with
; C=7 (ie requiring RAM7 at the top and the stack below $bfe0), M_P3DOS will
; automatically switch the stack into TSTACK during the call, so there is no
; need for calling code to adjust stack location before invoking M_P3DOS.
;
; For calls requiring normal configuration (ROM2/5/2/0), RAM0 must already
; be paged. For other calls, any banks can be paged at $c000, and will be
; restored when the +3DOS call has completed.
m_p3dos      equ 0x94 ; $94 (148) execute +3DOS/IDEDOS/NextZXOS call

; ***************************************************************************
; * M_ERRH ($95)                                                            *
; ***************************************************************************
; Install error handler for dot command.
; Entry: HL=address of error handler within dot command
; (0 to change back to standard handler)
;
; NOTES:
; Can only be used from within a dot command.
; If any BASIC error occurs during a call to ROM3 (using RST $10 or RST $18)
; then your error handler will be entered with:
;        DE=address that would have been returned to if the error had not
;           occurred
;        A=BASIC error code-1 (eg 8=9 STOP statement)
m_errh       equ 0x95 ; $95 (149) register dot command error handler

; ***************************************************************************
; * F_OPEN ($9a)                                                            *
; ***************************************************************************
; Open a file.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=filespec, null-terminated
;       B=access modes, a combination of:
;         any/all of:
; esx_mode_read           $01 request read access
; esx_mode_write          $02 request write access
; esx_mode_use_header     $40 read/write +3DOS header
;         plus one of:
; esx_mode_open_exist     $00 only open existing file
; esx_mode_open_creat     $08 open existing or create file
; esx_mode_creat_noexist  $04 create new file, error if exists
; esx_mode_creat_trunc    $0c create new file, delete existing
;
;       DE=8-byte buffer with/for +3DOS header data (if specified in mode)
;       (NB: filetype will be set to $ff if headerless file was opened)
; Exit (success):
;       Fc=0
;       A=file handle
; Exit (failure):
;       Fc=1
;       A=error cod
f_open      equ 0x9a ; $9a (154) open file

; ***************************************************************************
; * F_CLOSE ($9b)                                                           *
; ***************************************************************************
; Close a file or directory.
; Entry:
;       A=file handle or directory handle
; Exit (success):
;       Fc=0
;       A=0
; Exit (failure):
;       Fc=1
;       A=error code
f_close     equ 0x9b ; $9b (155) close file

; ***************************************************************************
; * F_SYNC ($9c)                                                            *
; ***************************************************************************
; Sync file changes to disk.
; Entry:
;       A=file handle
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_sync      equ 0x9c ; $9c (156) sync file changes to disk

; ***************************************************************************
; * F_READ ($9d)                                                            *
; ***************************************************************************
; Read bytes from file.
; Entry:
;       A=file handle
;       IX [HL from dot command]=address
;       BC=bytes to read
; Exit (success):
;       Fc=0
;       BC=bytes actually read (also in DE)
;       HL=address following bytes read
; Exit (failure):
;       Fc=1
;       BC=bytes actually read
;       A=error code
;
; NOTES:
; EOF is not an error, check BC to determine if all bytes requested were read.
f_read      equ 0x9d ; $9d (157) read file

; ***************************************************************************
; * F_WRITE ($9e)                                                           *
; ***************************************************************************
; Write bytes to file.
; Entry:
;       A=file handle
;       IX [HL from dot command]=address
;       BC=bytes to write
; Exit (success):
;       Fc=0
;       BC=bytes actually written
; Exit (failure):
;       Fc=1
;       BC=bytes actually written
f_write     equ 0x9e ; $9e (158) write file

; ***************************************************************************
; * F_SEEK ($9f)                                                            *
; ***************************************************************************
; Seek to position in file.
; Entry:
;       A=file handle
;       BCDE=bytes to seek
;       IXL [L from dot command]=seek mode:
; esx_seek_set $00 set the fileposition to BCDE
; esx_seek_fwd $01 add BCDE to the fileposition
; esx_seek_bwd $02 subtract BCDE from the fileposition
; Exit (success):
;       Fc=0
;       BCDE=current position
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTES:
; Attempts to seek past beginning/end of file leave BCDE=position=0/filesize
; respectively, with no error.
f_seek      equ 0x9f ; $9f (159) set file position

; ***************************************************************************
; * F_FGETPOS ($a0)                                                         *
; ***************************************************************************
; Get current file position.
; Entry:
;       A=file handle
; Exit (success):
;       Fc=0
;       BCDE=current position
; Exit (failure):
;       Fc=1
;       A=error code
f_fgetpos   equ 0xa0 ; $a0 (160) get file position

; ***************************************************************************
; * F_FSTAT ($a1)                                                           *
; ***************************************************************************
; Get file information/status.
; Entry:
;       A=file handle
;       IX [HL from dot command]=11-byte buffer address
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTES:
; The following details are returned in the 11-byte buffer:
;   +0(1) '*'
;   +1(1) $81
;   +2(1) file attributes (MS-DOS format)
;   +3(2) timestamp (MS-DOS format)
;   +5(2) datestamp (MS-DOS format)
;   +7(4)   file size in bytes
f_fstat     equ 0xa1 ; $a1 (161) get open file information

; ***************************************************************************
; * F_FTRUNCATE ($a2)                                                       *
; ***************************************************************************
; Truncate/extend file.
; Entry:
;       A=file handle
;       BCDE=new filesize
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTES:
; The M_SETCAPS ($91) hook can be used to modify the behaviour of this call
; so that is doesn't zeroise additional file sections (improving performance).
; Sets the filesize to precisely BCDE bytes.
; If BCDE<current filesize, the file is trunctated.
; If BCDE>current filesize, the file is extended. The extended part is erased
; with zeroes.
; The file position is unaffected. Therefore, if truncating, make sure to
; set the file position within the file before further writes (otherwise it
; will be extended again).
; +3DOS headers are included as part of the filesize. Truncating such files is
; not recommended.
f_ftruncate equ 0xa2 ; $a2 (162) truncate/extend open file

; ***************************************************************************
; * F_OPENDIR ($a3)                                                         *
; ***************************************************************************
; Open directory.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=directory, null-terminated
;       B=access mode
;         add together any or all of:
;           esx_mode_use_lfn        $10 return long filenames
;           esx_mode_use_wildcards  $20 only entries matching wildcard
; passed to F_READDIR are returned
; esx_mode_use_header     $40 read/write +3DOS headers
; Exit (success):
;       A=dir handle
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTES:
; Access modes determine how entries are formatted by F_READDIR
f_opendir   equ 0xa3 ; $a3 (163) open directory for reading

; ***************************************************************************
; * F_READDIR ($a4)                                                         *
; ***************************************************************************
; Read next directory entry.
; Entry:
;       A=handle
;       IX [HL from dot command]=buffer
;       Additionally, if directory was opened with esx_mode_use_wildcards:
;       DE=wildcard string (null-terminated)
; Exit (success):
;       A=number of entries returned (0 or 1)
;         If 0, there are no more entries
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; Buffer format:
;  1 byte file attributes (MSDOS format)
;  ? bytes file/directory name, null-terminated
;  2 bytes timestamp (MSDOS format)
;  2 bytes datestamp (MSDOS format)
;  4 bytes  file size
;
; NOTES:
;
; If the directory was opened with the esx_mode_use_lfn bit, long filenames
; (up to 260 bytes plus terminator) are returned
; otherwise short filenames
; (up to 12 bytes plus terminator) are returned.
;
; If opened with the esx_mode_use_header bit, after the normal entry follows the
; 8-byte +3DOS header (for headerless files, type=$ff, other bytes=zero).
;
; If opened with the esx_mode_use_wildcards bit, then only the next filename
; matching the wildcard string provided in DE is returned.
f_readdir   equ 0xa4 ; $a4 (164) read directory entry

; ***************************************************************************
; * F_TELLDIR ($a5)                                                         *
; ***************************************************************************
; Get current directory position.
; Entry:
;       A=handle
; Exit (success):
;       BCDE=current offset in directory
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_telldir   equ 0xa5 ; $a5 (165) get directory position

; ***************************************************************************
; * F_SEEKDIR ($a6)                                                         *
; ***************************************************************************
; Set current directory position.
; Entry:
;       A=handle
;       BCDE=offset in directory to seek to (as returned by F_TELLDIR)
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_seekdir   equ 0xa6 ; $a6 (166) set directory position

; ***************************************************************************
; * F_REWINDDIR ($a7)                                                       *
; ***************************************************************************
; Rewind directory position to the start of the directory.
; Entry:
;       A=handle
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_rewinddir equ 0xa7 ; $a7 (167) rewind to start of directory

; ***************************************************************************
; * F_GETCWD ($a8)                                                          *
; ***************************************************************************
; Get current working directory (or working directory for any filespec)
; Entry:
;       A=drive, to obtain current working directory for that drive
;   or: A=$ff, to obtain working directory for a supplied filespec in DE
;       DE=filespec (only if A=$ff)
;       IX [HL from dot command]=buffer for null-terminated path
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTE:
; If obtaining a path for a supplied filespec, the filename part (after the
; final /, \ or :) is ignored so need not be provided, or can be the name of a
; non-existent file/dir.
; NOTE: ; IX [HL from dot command] and DE may both address the same memory, if desired.
f_getcwd    equ 0xa8 ; $a8 (168) get current working directory

; ***************************************************************************
; * F_CHDIR ($a9)                                                           *
; ***************************************************************************
; Change directory.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=path, null-terminated
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTE: This hook changes the directory for the drive specified in A (or in
;       the path) but does not change the current drive. If this is required
;       you must also use the M_GETSETDRV hook.
f_chdir     equ 0xa9 ; $a9 (169) change directory

; ***************************************************************************
; * F_MKDIR ($aa)                                                           *
; ***************************************************************************
; Create directory.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=path, null-terminated
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_mkdir     equ 0xaa ; $aa (170) make directory

; ***************************************************************************
; * F_RMDIR ($ab)                                                           *
; ***************************************************************************
; Remove directory.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=path, null-terminated
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_rmdir     equ 0xab ; $ab (171) remove directory

; ***************************************************************************
; * F_STAT ($ac)                                                            *
; ***************************************************************************
; Get unopened file information/status.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=filespec, null-terminated
;       DE=11-byte buffer address
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTES:
; The following details are returned in the 11-byte buffer:
;   +0(1) drive specifier
;   +1(1) $81
;   +2(1) file attributes (MS-DOS format)
;   +3(2) timestamp (MS-DOS format)
;   +5(2) datestamp (MS-DOS format)
;   +7(4)   file size in bytes
f_stat      equ 0xac ; $ac (172) get unopen file information

; ***************************************************************************
; * F_UNLINK ($ad)                                                          *
; ***************************************************************************
; Delete file.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=filespec, null-terminated
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_unlink    equ 0xad ; $ad (173) delete file

; ***************************************************************************
; * F_TRUNCATE ($ae)                                                        *
; ***************************************************************************
; Truncate/extend unopened file.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=source filespec, null-terminated
;       BCDE=new filesize
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
;
; NOTES:
; The M_SETCAPS ($91) hook can be used to modify the behaviour of this call
; so that is doesn't zeroise additional file sections (improving performance).
; Sets the filesize to precisely BCDE bytes.
; If BCDE<current filesize, the file is trunctated.
; If BCDE>current filesize, the file is extended. The extended part is erased
; with zeroes.
; +3DOS headers are included as part of the filesize. Truncating such files is
; not recommended.
f_truncate  equ 0xae ; $ae (174) truncate/extend unopen file

; ***************************************************************************
; * F_CHMOD ($af)                                                           *
; ***************************************************************************
; Modify file attributes.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=filespec, null-terminated
;       B=attribute values bitmap
;       C=bitmap of attributes to change (1=change, 0=do not change)
;
;       Bitmasks for B and C are any combination of:
;           A_WRITE %00000001
;           A_READ %10000000
;           A_RDWR %10000001
;           A_HIDDEN %00000010
;           A_SYSTEM %00000100
;           A_ARCH %00100000
;
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_chmod     equ 0xaf ; $af (175) change file attributes

; ***************************************************************************
; * F_RENAME ($b0)                                                          *
; ***************************************************************************
; Rename or move a file.
; Entry:
;       A=drive specifier (overridden if filespec includes a drive)
;       IX [HL from dot command]=source filespec, null-terminated
;       DE=destination filespec, null-terminated
; Exit (success):
;       Fc=0
; Exit (failure):
;       Fc=1
;       A=error code
f_rename    equ 0xb0 ; $b0 (176) rename/move file

; ***************************************************************************
; * F_GETFREE ($b1)                                                         *
; ***************************************************************************
; Gets free space on drive.
; Entry:
;       A=drive specifier
; Exit (success):
;       Fc=0
;       BCDE=number of 512-byte blocks free on drive
; Exit (failure):
;       Fc=1 ;       A=error code
f_getfree   equ 0xb1 ; $b1 (177) get free space

; File Modes
    esx_mode_read          equ 0x01 ;request read access
    esx_mode_write         equ 0x02 ;request write access
    esx_mode_use_header    equ 0x40 ;read/write +3DOS header
    esx_mode_use_lfn       equ 0x10 ;return long filenames
    esx_mode_use_wildcards equ 0x20 ;only entries matching wildcard passed to F_READDIR are returned
    esx_mode_open_exist    equ 0x00 ;only open existing file
    esx_mode_open_creat    equ 0x08 ;open existing or create file
    esx_mode_creat_noexist equ 0x04 ;create new file, error if exists
    esx_mode_creat_trunc   equ 0x0c ;create new file, delete existing

; Seek Mode
    esx_seek_set equ 0x00 ;set the fileposition to BCDE
    esx_seek_fwd equ 0x01 ;add BCDE to the fileposition
    esx_seek_bwd equ 0x02 ;subtract BCDE from the fileposition

; File attributes
    A_WRITE  equ %00000001
    A_READ   equ %10000000
    A_RDWR   equ %10000001
    A_HIDDEN equ %00000010
    A_SYSTEM equ %00000100
    A_ARCH   equ %00100000

; Error codes
    esx_ok              equ  0 ;Unknown error
    esx_eok             equ  1 ;OK
    esx_nonsense        equ  2 ;Nonsense in esxDOS
    esx_estend          equ  3 ;Statement end error
    esx_ewrtype         equ  4 ;Wrong file type
    esx_enoent          equ  5 ;No such file or dir
    esx_eio             equ  6 ;I/O error
    esx_einval          equ  7 ;Invalid filename
    esx_eacces          equ  8 ;Access denied
    esx_enospc          equ  9 ;Drive full
    esx_enxio           equ 10 ;Invalid i/o request
    esx_enodrv          equ 11 ;No such drive
    esx_enfile          equ 12 ;Too many files open
    esx_ebadf           equ 13 ;Bad file number
    esx_enodev          equ 14 ;No such device
    esx_eoverflow       equ 15 ;File pointer overflow
    esx_eisdir          equ 16 ;Is a directory
    esx_enotdir         equ 17 ;Not a directory
    esx_eexist          equ 18 ;Already exists
    esx_epath           equ 19 ;Invalid path
    esx_esys            equ 20 ;Missing system
    esx_enametoolong    equ 21 ;Path too long
    esx_enocmd          equ 22 ;No such command
    esx_einuse          equ 23 ;In use
    esx_erdonly         equ 24 ;Read only
    esx_everify         equ 25 ;Verify failed
    esx_eloadingko      equ 26 ;Sys file load error
    esx_edirinuse       equ 27 ;Directory in use
    esx_emapramactive   equ 28 ;MAPRAM is active
    esx_edrivebusy      equ 29 ;Drive busy
    esx_efsunknown      equ 30 ;Unknown filesystem
    esx_edevicebusy     equ 31 ;Device busy

; Macros
    macro   esxcall hook
        rst 8
        db  hook
    endm

    MACRO   dosversion
        rst 8
        db  m_dosversion
    ENDM

    MACRO   getdrive
        xor a
        rst 8
        db  m_getsetdrv
    ENDM

    MACRO   setdrive drv
        ld  a,drv
        rst 8
        db  m_getsetdrv
    ENDM

    MACRO gethandle
        rst 8
        db  m_gethandle
    ENDM
