#once

#include "cpu.asm"
#include "aliases.asm"
#include "../common.asm"

#bankdef header   { #addr 0x0000, #size 0x0010, #outp 8 * 0x0000 }
#bankdef prg      { #addr 0x8000, #size 0x7ffa, #outp 8 * 0x0010 }
#bankdef vectors  { #addr 0xfffa, #size 0x0006, #outp 8 * 0x800a }

#bankdef zeropage { #addr 0x0000, #size 0x0100 }
#bankdef stack    { #addr 0x0100, #size 0x0100 }
#bankdef oam      { #addr 0x0200, #size 0x0100 }
#bankdef ram      { #addr 0x0300, #size 0x0500 }


#bank header

; magic number
#d "NES", 0x1a

#d8 2 ; 16KB PRG bank count
#d8 0 ; 8KB CHR bank count
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


; Setup the three main functions
#bank vectors
#fn bind(func) => le(func`16)

#d16 bind(nmi  )
#d16 bind(reset)
#d16 bind(irq  )


; MARK: CONSTANTS
PPU_CTRL    = 0x2000
PPU_MASK    = 0x2001
PPU_STATUS  = 0x2002
PPU_ADDR    = 0x2006
PPU_DATA    = 0x2007
APU_DMC     = 0x4010
APU_FRMCNTR = 0x4017

PPU_CTRL_NMI = 0b10000000

PPU_MASK_LEFTBKG = 0b00000010
PPU_MASK_LEFTSPR = 0b00000100
PPU_MASK_SHOWBKG = 0b00001000
PPU_MASK_SHOWSPR = 0b00010000

VRAM_PALETTE = 0x3f00


#bank zeropage

; General purpose variable
var:
    ; Timer to countdown
    .timer: #res 1

    ; Index of the palette to use
    .palette_index: #res 1

    ; 16-bits address to use
    .addr: #res 2


#bank prg

reset:
    ; disable IRQs and decimal mode
    disable interrupt
    clear decimal

    ; disable APU frame IRQ
    load  x with im(0x40)
    store x in   APU_FRMCNTR

    ; set up stack
    load x with im(0xff)
    transfer x to stack

    ; disable NMI
    ++ x
    store x in PPU_CTRL

    ; disable rendering
    store x in PPU_MASK

    ; disable DMC IRQs
    store x in APU_DMC

    ; wait for PPU to be ready
    .vblank_wait1:
        bit PPU_STATUS
        if (+) .vblank_wait1

    ; clear memory
    .clear_mem:
        load a with im(0x00)
        store a in 0x0000 [x]
        store a in 0x0100 [x]
        store a in 0x0200 [x]
        store a in 0x0300 [x]
        store a in 0x0400 [x]
        store a in 0x0500 [x]
        store a in 0x0600 [x]
        store a in 0x0700 [x]
        ++ x
        if not zero .clear_mem

    ; wait for PPU to be ready again
    .vblank_wait2:
        bit PPU_STATUS
        if positive .vblank_wait2

    ; load first palette color
    load a with 0x0d
    load_palette()

    ; enable rendering
    load  a with im(PPU_MASK_SHOWBKG | PPU_MASK_LEFTBKG)
    store a in PPU_MASK

    ; enable NMI
    load  a with im(PPU_CTRL_NMI)
    store a in PPU_CTRL

    ; wait for NMI
    .infinite:
        goto .infinite


; interrupt called at the end of every frame
nmi:
    ; increment timer
    ++          var.timer
    load a with var.timer
    cmp  a with im(8)
    if diff .end

    ; if timer reached 8...
        load  a with im(0)
        store a in var.timer

        ; update background color
        load x with var.palette_index
        load a with palette [x]
        load_palette()

        ; increment palette index
        ++ var.palette_index
        load a with var.palette_index
        cmp  a with im(palette.len)
        if diff .end

        ; if pallete index reached the end of the table...
            load  a with im(0)
            store a in   var.palette_index
    .end:
    return from interrupt

; interrupt called at every scanline
irq:
    return from interrupt


load_palette:
    ; store color from A in all palette slots
    load x with PPU_STATUS

    load  x with im(hi(VRAM_PALETTE))
    store x in   PPU_ADDR
    load  x with im(lo(VRAM_PALETTE))
    store x in   PPU_ADDR

    load y with im(0x20)
    .palette_loop:
        store a in PPU_DATA
        --y
        if not zero .palette_loop

    return

; Palettes available
palette:
    #d8 0x0d, 0x01, 0x12, 0x21, 0x31, 0x21, 0x12, 0x01, 0x0d ; blues
    #d8 0x0d, 0x06, 0x16, 0x26, 0x36, 0x26, 0x16, 0x06, 0x0d ; reds
    #d8 0x0d, 0x09, 0x19, 0x29, 0x39, 0x29, 0x19, 0x09, 0x0d ; greens

.len = $ - palette
