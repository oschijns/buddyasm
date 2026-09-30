#once

#include "6502.asm"
#include "banks.asm"
#include "constants.asm"


; low or high byte of a 16-bits word
#fn lo(word) => word[ 7:0]
#fn hi(word) => word[15:8]

; maximum or minimum between two values
#fn min(a, b) => a < b ? a : b
#fn max(a, b) => a > b ? a : b


; Setup the vectors with the three main functions
#fn setup_vectors(nmi, start, irq) => $le(nmi`16) @ $le(start`16) @ $le(irq`16)


; Repeat a sequence of expressions N times
; Example usage: `#d repeat(3, asm {nop})`
#fn repeat(n, expr) =>
{
    n > 0 
        ? expr @ repeat(n - 1, expr) 
        : 0`0
}


; Simply copy a value from one memory location to an other (using A)
#fn copy(to, from) => asm
{
    lda {from}
    sta {to}
}

; Return the negative value as a signed 8bit number (crash?)
#fn negative(const) => (-const)`8


; Swap the two nybbles of a byte (0x1A becomes 0xA1)
; Assumes the byte to swap is already loaded in register A
#fn swap_nybbles() => asm
{
    asl a
    adc #0b1000_0000
    rol a

    asl a
    adc #0b1000_0000
    rol a
}


; Set an address at the specified destination
#fn set_addr(dest, addr) => asm
{
    lda # lo({addr})
    sta <    {dest} + 0
    lda # hi({addr})
    sta <    {dest} + 1
}


; Case of a switch-like statement
#fn switch_case(case, label) => asm
{
    cmp {case}
    beq {label}
}


; Store registers states in the persistant area of the zeropage.
; We could use main memory instead and distinguish NMI from IRQ 
; but at the cost of more RAM and more cycles per loads and stores.
#bank zpsav

; Storage for registers when executing interrupts
interrupt:
    .a: #res 1
    .x: #res 1
    .y: #res 1

; Store registers state to execute interrupt
#fn interrupt_store() => asm
{
    sta < interrupt.a
    stx < interrupt.x
    sty < interrupt.y
}

; Restore registers state to exit interrupt
#fn interrupt_load() => asm
{
    lda < interrupt.a
    ldx < interrupt.x
    ldy < interrupt.y
}
