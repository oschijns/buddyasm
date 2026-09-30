#once

#include "std/mod.asm"
#include "std/reset_mem.asm"

; ------------------------------------------------------------------------------
; MARK: Debugging module
; ------------------------------------------------------------------------------
#bank header
debug:

    ; Specify where to find characters for '0'  and 'A' when printing hexadecimal values. 
    .CHAR_ZERO  = 0x30
    .CHAR_ALPHA = 0x41


#bank zpsav

    ; Array to display on the screen
    .array_size:    #res 1
    .array_addr:
    .array_addr_lo: #res 1
    .array_addr_hi: #res 1


#bank zptmp

    ; Store value picked from array
    .value: #res 1


#bank program

    ; Print the array to the screen as a sequence of hexadecimal values.
    ; array_size and array_addr must be set before calling this subroutine.
    .print_array:

        ; Initialize the PPU to start printing at the top of the screen
        ; TODO:...

        ldy #0
        ..loop:

            ; Pick a value from the array
            lda (.array_addr), y
            sta .value

            ; Print high nybble first
            #d repeat(4, asm { lsr a })
            jsr .print_hexadecimal

            ; Print low nybble second
            lda .value
            and #0xF
            jsr .print_hexadecimal

            cpy .array_size
            bne ..loop

        rts

    ; Given a value (A) in the range [0, 15] print it as an hexadecimal digit
    .print_hexadecimal:

        ; Based on the value, check if we need to add '0' or 'A' 
        clc
        cmp #0xA
        bge ..else
            adc .CHAR_ZERO
            jmp ..endif
        ..else:
            ..DIFF = .CHAR_ALPHA - 0xA
            adc ..DIFF
        ..endif:

        sta ppu_data

        rts


; Helper macro to print something on the screen
#fn debug_print_array(addr, size) =>
{
        copy(debug.array_size, size)
    set_addr(debug.array_addr, addr)
    asm { jsr debug.print_array }
}