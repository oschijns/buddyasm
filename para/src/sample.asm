#once

#include "../output/para.asm"

#bank program

SrcLo   = $00
SrcHi   = $01
Rows    = $02
Address = $2000 + 6 * 32 + 10

DrawPicture:

    ; PPU address = $2000
    LDA #hi(Address)
    STA ppu_address
    LDA #lo(Address)
    STA ppu_address

    ; Source address
    LDA #lo(artwork.indexes)
    STA SrcLo
    LDA #hi(artwork.indexes)
    STA SrcHi

    LDA #12
    STA Rows

NextRow:

    ; --------------------------------
    ; Write 12 tiles
    ; --------------------------------

    LDX #12

WriteTile:

    LDY #0
    LDA (SrcLo), y
    STA $2007

    ; Advance source pointer
    INC SrcLo
    BNE NoCarry

    INC SrcHi

NoCarry:

    DEX
    BNE WriteTile

    ; --------------------------------
    ; Move to next nametable row
    ;
    ; 32 tiles per row
    ; We already wrote 12
    ; Therefore skip 20.
    ; --------------------------------

    LDX #20

SkipTiles:

    LDA #0
    STA $2007

    DEX
    BNE SkipTiles

    ; --------------------------------
    ; Next row
    ; --------------------------------

    DEC Rows
    BNE NextRow

    ; Reset scroll
    LDA #0
    STA ppu_address
    STA ppu_address

    RTS