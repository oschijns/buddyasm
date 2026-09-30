#once

#include "../assets/output/mercredi.asm"

#bank program

SrcLo   = $00
SrcHi   = $01
Rows    = $02
Address = ppu_nametable_0 + 4 * PPU_NAMETABLE_TILE_MAP_WIDTH + 7
Width   = 10

AddrText = ppu_nametable_0 + 16 * PPU_NAMETABLE_TILE_MAP_WIDTH + 4

TextLine0:
#d "NOUS SOMMES MERCREDI"
    .len = $ - TextLine0

TextLine1:
#d "CA TOMBE BIEN, C'EST"
    .len = $ - TextLine1

TextLine2:
#d "LE JOUR OU ON SORT"
    .len = $ - TextLine2

TextLine3:
#d "LES POUBELLES !"
    .len = $ - TextLine3

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
    STA ppu_data

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
    STA ppu_data

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


; Write lines of text

; Write three lines of text to the nametable

; LINE 0
    LDA #hi(AddrText)
    STA ppu_address
    LDA #lo(AddrText)
    STA ppu_address

    LDY #0
WriteLine0:
    LDA TextLine0, y
    STA ppu_data
    iny
    cpy #TextLine0.len
    bne WriteLine0

; LINE 1
    LDA #hi(AddrText + PPU_NAMETABLE_TILE_MAP_WIDTH * 2)
    STA ppu_address
    LDA #lo(AddrText + PPU_NAMETABLE_TILE_MAP_WIDTH * 2)
    STA ppu_address

    LDY #0
WriteLine1:
    LDA TextLine1, y
    STA ppu_data
    iny
    cpy #TextLine1.len
    bne WriteLine1

; LINE 2
    LDA #hi(AddrText + PPU_NAMETABLE_TILE_MAP_WIDTH * 3)
    STA ppu_address
    LDA #lo(AddrText + PPU_NAMETABLE_TILE_MAP_WIDTH * 3)
    STA ppu_address

    LDY #0
WriteLine2:
    LDA TextLine2, y
    STA ppu_data
    iny
    cpy #TextLine2.len
    bne WriteLine2

; LINE 3
    LDA #hi(AddrText + PPU_NAMETABLE_TILE_MAP_WIDTH * 4)
    STA ppu_address
    LDA #lo(AddrText + PPU_NAMETABLE_TILE_MAP_WIDTH * 4)
    STA ppu_address

    LDY #0
WriteLine3:
    LDA TextLine3, y
    STA ppu_data
    iny
    cpy #TextLine3.len
    bne WriteLine3


; Reset scroll

    ; Reset scroll
    LDA #0
    STA ppu_address
    STA ppu_address

    RTS