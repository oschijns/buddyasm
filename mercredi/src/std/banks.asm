#once

; NES Memory layout:

; Start    End      Description                     Comment
;-------------------------------------------------------------------------------
; $0000    $00FF    RAM, zero-page
; $0100    $01FF    RAM, CPU stack
; $0200    $07FF    RAM, general-purpose
; $0800    $1FFF                                    mirror of $0000-$07FF
; $2000    $2007    PPU registers
; $2008    $3FFF                                    mirror of $2000-$2007
; $4000    $400F    APU registers
; $4010    $4017    DMC, joystick, APU registers
; $4020    $5FFF    Cartridge                       maybe mapper registers
; $6000    $7FFF    Cartridge RAM                   maybe battery-backed
; $8000    $FFFF    PRG ROM                         maybe bank switched
; $FFFA    $FFFB    NMI   vector
; $FFFC    $FFFD    Reset vector
; $FFFE    $FFFF    BRK   vector
;-------------------------------------------------------------------------------


; Banks definition
; #addr (bytes) define the address where to place the bank
; #size (bytes) define the size of the bank
; #outp (bits) define where to place the bank in the ROM file

; ROM banks
#bankdef header   { #addr 0x0000, #size 0x0010, #outp 8 * 0x0000 } ; header
#bankdef consts   { #addr 0x8000, #size 0x1000, #outp 8 * 0x0010 } ; PRG ROM
#bankdef program  { #addr 0x9000, #size 0x6ffa, #outp 8 * 0x1010 } ; PRG ROM
#bankdef vectors  { #addr 0xfffa, #size 0x0006, #outp 8 * 0x800a } ; vectors
#bankdef chr      { #addr 0x0000, #size 0x2000, #outp 8 * 0x8010 } ; CHR ROM

; Fill the CHR bank with the tileset
#bank chr
#d $incbin("../../output/tileset.chr")
#d $incbin("../../output/tileset.chr")

; Note:
; "consts" and "program" are split to allow intertwining
; raw arrays with opcodes when writing algorithms.


; RAM banks
#bankdef zptmp  { #addr 0x0000, #size 0x0080 } ; zero-page
#bankdef zpsav  { #addr 0x0080, #size 0x0080 } ; zero-page
#bankdef stack  { #addr 0x0100, #size 0x0100 } ; CPU stack
#bankdef oam    { #addr 0x0200, #size 0x0100 } ; RAM
#bankdef memtmp { #addr 0x0300, #size 0x0100 } ; RAM
#bankdef memsav { #addr 0x0400, #size 0x0400 } ; RAM

; Start of temporary partition of RAM
START_ZP  = 0x0000
START_MEM = 0x0300

; Note:
; zeropage and RAM are split into two banks to differentiate
; temporary memory (tmp) from dedicated memory (sav).



#bank header

; magic number
#d "NES", 0x1a

#d8 2 ; 16KB PRG bank count
#d8 1 ;  8KB CHR bank count
#d4 0 ; low nybble of mapper id
#d1 0
#d1 0 ; trainer presence
#d1 0 ; SRAM presence
#d1 0 ; mirroring
#d4 0 ; high nybble of mapper id
#d4 0
#d8 0
#d8 0
#d2 0
#d1 0 ; bus conflict presence
#d1 0 ; extra RAM presence
#d2 0
#d2 0 ; region
