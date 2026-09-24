#once

;;; Compile this program with:
;;; 
;;; customasm nes_colors.asm -o nes_colors.nes
;;; 
;;; ...then run it on your favorite NES emulator!


#include "std/mod.asm"
#include "debug.asm"


; Setup the three main entry points of the program
#bank vectors
#d setup_vectors(program_nmi, program_start, program_irq)


#bank program

program_start:
    ; disable IRQs and decimal mode
    sei
    cld

    ; disable APU frame IRQ
    ldx #0x40
    stx apu_frame_counter

    ; set up stack
    ldx #0xff
    txs

    ; disable NMI
    inx
    stx ppu_control

    ; disable rendering
    stx ppu_mask

    ; disable DMC IRQs
    stx apu_dmc_control

    ; wait for PPU to be ready
    .vblank_wait1:
        bit ppu_status
        bpl .vblank_wait1

    ; clear memory
    .clear_mem:
        lda #0x00
        sta 0x0000, x
        sta 0x0100, x
        sta 0x0200, x
        sta 0x0300, x
        sta 0x0400, x
        sta 0x0500, x
        sta 0x0600, x
        sta 0x0700, x
        inx
        bne .clear_mem

    ; wait for PPU to be ready again
    .vblank_wait2:
        bit ppu_status
        bpl .vblank_wait2

    ; load default palette
    jsr load_palette

    ;<fill>
    ; PPU address = $2000
        lda #0x20
        sta ppu_address
        lda #0x00
        sta ppu_address

        ldy #0

        .fill_rows:
            ldx #0

        .fill_row:
            txa
            sta ppu_data
            inx
            cpx #32
            bne .fill_row

            iny
            cpy #30
            bne .fill_rows
    ;</fill>

    ; enable rendering
    lda #PPU_MASK_RENDER_BG | PPU_MASK_LEFTMOST_BG
    sta ppu_mask

    ; enable NMI
    lda #PPU_CTRL_NMI
    sta ppu_control

    ; wait for NMI
    .infinite:
        jmp .infinite


; interrupt called at the end of every frame
program_nmi:
    #d interrupt_store()
    #d interrupt_load()
    rti


; interrupt called at every scanline
program_irq:
    #d interrupt_store()
    #d interrupt_load()
    rti


#bank zpsav

; General purpose variable
var:
    ; Timer to countdown
    .timer: #res 1

    ; Index of the palette to use
    .palette_index: #res 1

    ; 16-bits address to use
    .addr: #res 2


#bank program

load_palette:
    ; store color from A in all palette slots
    ldx ppu_status

    ldx # hi(ppu_palette)
    stx ppu_address
    ldx # lo(ppu_palette)
    stx ppu_address

    ldy #0x20
    .palette_loop:
        lda palette, y
        sta ppu_data
        dey
        bne .palette_loop

    rts

; Palettes available
palette:
    #d8 0x0d, 0x01, 0x12, 0x21 ; blues
    #d8 0x0d, 0x06, 0x16, 0x26 ; reds
    #d8 0x0d, 0x09, 0x19, 0x29 ; greens
    #d8 0x0d, 0x01, 0x12, 0x21 ; blues

    #d8 0x0d, 0x06, 0x16, 0x26 ; reds
    #d8 0x0d, 0x09, 0x19, 0x29 ; greens
    #d8 0x0d, 0x01, 0x12, 0x21 ; blues
    #d8 0x0d, 0x06, 0x16, 0x26 ; reds

.len = $ - palette
