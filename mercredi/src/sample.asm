#once

#include "../assets/output/mercredi.asm"

#bank program

SrcLo   = $00
SrcHi   = $01
Rows    = $02
Address = $2000 + 4 * 32 + 7
Width   = 10

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

    LDA #10
    STA Rows

NextRow:

    ; --------------------------------
    ; Write 10 tiles
    ; --------------------------------

    LDX #Width

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

    LDX #(32 - Width)

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

    ; Write the artwork attribute data to the nametable attribute table
    LDA #hi(ppu_nametable_0 + PPU_NAMETABLE_SIZE_TILE_MAP)
    STA ppu_address
    LDA #lo(ppu_nametable_0 + PPU_NAMETABLE_SIZE_TILE_MAP)
    STA ppu_address

    ; Source address
    LDA #lo(artwork.attributes)
    STA SrcLo
    LDA #hi(artwork.attributes)
    STA SrcHi

    LDA #artwork.attributes.len
    STA Rows

WriteAttribute:
    LDY #0
    LDA (SrcLo), y
    STA ppu_data

    ; Advance source pointer
    INC SrcLo
    BNE NoAttributeCarry

    INC SrcHi

NoAttributeCarry:
    DEC Rows
    BNE WriteAttribute

    ; Reset scroll
    LDA #0
    STA ppu_address
    STA ppu_address

    RTS