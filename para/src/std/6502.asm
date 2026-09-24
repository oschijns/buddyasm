#once

; Status flags:
; - N : negative
; - Z : zero
; - C : carry
; - I : interrupt disable
; - D : decimal
; - V : overflow


; Cycle times indicated on the right column
; +  : +1 if page crossed
; ++ : +1 if page crossed & +1 if branch taken


#subruledef cpu6502_reladdr
{
    {addr: u16} =>
    {
        reladdr = addr - $ - 2
        $assert(reladdr <=  0x7f)
        $assert(reladdr >= !0x7f)
        reladdr`8
    }
}


#ruledef cpu6502
{
    ; Add memory to accumulator with carry
    ; N Z C I D V
    ; # # # . . #
    adc #{imm:   i8 }           => 0x69 @ imm       ; 2
    adc <{zaddr: u8 }           => 0x65 @ zaddr     ; 3
    adc <{zaddr: u8 },  x       => 0x75 @ zaddr     ; 4
    adc  {zaddr: u8 }           => 0x65 @ zaddr     ; 3
    adc  {zaddr: u8 },  x       => 0x75 @ zaddr     ; 4
    adc  {addr:  u16}           => 0x6d @ $le(addr) ; 4
    adc  {addr:  u16},  x       => 0x7d @ $le(addr) ; 4+
    adc  {addr:  u16},  y       => 0x79 @ $le(addr) ; 4+
    adc ({zaddr: u8 },  x)      => 0x61 @ zaddr     ; 6
    adc ({zaddr: u8 }), y       => 0x71 @ zaddr     ; 5+

    ; AND memory with accumulator
    ; N Z C I D V
    ; # # . . . .
    and #{imm:   i8 }           => 0x29 @ imm       ; 2
    and <{zaddr: u8 }           => 0x25 @ zaddr     ; 3
    and <{zaddr: u8 },  x       => 0x35 @ zaddr     ; 4
    and  {zaddr: u8 }           => 0x25 @ zaddr     ; 3
    and  {zaddr: u8 },  x       => 0x35 @ zaddr     ; 4
    and  {addr:  u16}           => 0x2d @ $le(addr) ; 4
    and  {addr:  u16},  x       => 0x3d @ $le(addr) ; 4+
    and  {addr:  u16},  y       => 0x39 @ $le(addr) ; 4+
    and ({zaddr: u8 },  x)      => 0x21 @ zaddr     ; 6
    and ({zaddr: u8 }), y       => 0x31 @ zaddr     ; 5+

    ; Shift left one bit
    ; N Z C I D V
    ; # # # . . .
    asl  a                      => 0x0a             ; 2
    asl <{zaddr: u8 }           => 0x06 @ zaddr     ; 5
    asl <{zaddr: u8 }, x        => 0x16 @ zaddr     ; 6
    asl  {zaddr: u8 }           => 0x06 @ zaddr     ; 5
    asl  {zaddr: u8 }, x        => 0x16 @ zaddr     ; 6
    asl  {addr:  u16}           => 0x0e @ $le(addr) ; 6
    asl  {addr:  u16}, x        => 0x1e @ $le(addr) ; 7

    ; Branch on carry clear
    ; N Z C I D V
    ; . . . . . .
    bcc {addr: cpu6502_reladdr} => 0x90 @ addr      ; 2++

    ; Branch on carry set
    ; N Z C I D V
    ; . . . . . .
    bcs {addr: cpu6502_reladdr} => 0xb0 @ addr      ; 2++

    ; Branch on result zero
    ; N Z C I D V
    ; . . . . . .
    beq {addr: cpu6502_reladdr} => 0xf0 @ addr      ; 2++

    ; Test bit in memory with accumulator
    ; N Z C I D V
    ; # # . . . #
    bit <{zaddr: u8 }           => 0x24 @ zaddr     ; 3
    bit  {zaddr: u8 }           => 0x24 @ zaddr     ; 3
    bit  {addr:  u16}           => 0x2c @ $le(addr) ; 4

    ; Branch on result minus
    ; N Z C I D V
    ; . . . . . .
    bmi {addr: cpu6502_reladdr} => 0x30 @ addr      ; 2++

    ; Branch on result not zero
    ; N Z C I D V
    ; . . . . . .
    bne {addr: cpu6502_reladdr} => 0xd0 @ addr      ; 2++

    ; Branch on result plus
    ; N Z C I D V
    ; . . . . . .
    bpl {addr: cpu6502_reladdr} => 0x10 @ addr      ; 2++

    ; Force break
    ; N Z C I D V
    ; . . . 1 . .
    brk                         => 0x00             ; 7

    ; Branch on overflow clear
    ; N Z C I D V
    ; . . . . . .
    bvc {addr: cpu6502_reladdr} => 0x50 @ addr      ; 2++

    ; Branch on overflow set
    ; N Z C I D V
    ; . . . . . .
    bvs {addr: cpu6502_reladdr} => 0x70 @ addr      ; 2++

    ; Clear carry flag
    ; N Z C I D V
    ; . . 0 . . .
    clc                         => 0x18             ; 2

    ; Clear decimal mode
    ; N Z C I D V
    ; . . . . 0 .
    cld                         => 0xd8             ; 2

    ; Clear interrupt disable status
    ; N Z C I D V
    ; . . . 0 . .
    cli                         => 0x58             ; 2

    ;CLear overflow flag
    ; N Z C I D V
    ; . . . . . 0
    clv                         => 0xb8             ; 2

    ; Compare memory and accumulator
    ; N Z C I D V
    ; # # # . . .
    cmp #{imm:   i8 }           => 0xc9 @ imm       ; 2
    cmp <{zaddr: u8 }           => 0xc5 @ zaddr     ; 3
    cmp <{zaddr: u8 },  x       => 0xd5 @ zaddr     ; 4
    cmp  {zaddr: u8 }           => 0xc5 @ zaddr     ; 3
    cmp  {zaddr: u8 },  x       => 0xd5 @ zaddr     ; 4
    cmp  {addr:  u16}           => 0xcd @ $le(addr) ; 4
    cmp  {addr:  u16},  x       => 0xdd @ $le(addr) ; 4+
    cmp  {addr:  u16},  y       => 0xd9 @ $le(addr) ; 4+
    cmp ({zaddr: u8 },  x)      => 0xc1 @ zaddr     ; 6
    cmp ({zaddr: u8 }), y       => 0xd1 @ zaddr     ; 5+

    ; Compare memory and index X
    ; N Z C I D V
    ; # # # . . .
    cpx #{imm:   i8 }           => 0xe0 @ imm       ; 2
    cpx <{zaddr: u8 }           => 0xe4 @ zaddr     ; 3
    cpx  {zaddr: u8 }           => 0xe4 @ zaddr     ; 3
    cpx  {addr:  u16}           => 0xec @ $le(addr) ; 4

    ; Compare memory and index Y
    ; N Z C I D V
    ; # # # . . .
    cpy #{imm:   i8 }           => 0xc0 @ imm       ; 2
    cpy <{zaddr: u8 }           => 0xc4 @ zaddr     ; 3
    cpy  {zaddr: u8 }           => 0xc4 @ zaddr     ; 3
    cpy  {addr:  u16}           => 0xcc @ $le(addr) ; 4

    ; Decrement memory by one
    ; N Z C I D V
    ; # # . . . .
    dec <{zaddr: u8 }           => 0xc6 @ zaddr     ; 5
    dec <{zaddr: u8 }, x        => 0xd6 @ zaddr     ; 6
    dec  {zaddr: u8 }           => 0xc6 @ zaddr     ; 5
    dec  {zaddr: u8 }, x        => 0xd6 @ zaddr     ; 6
    dec  {addr:  u16}           => 0xce @ $le(addr) ; 6
    dec  {addr:  u16}, x        => 0xde @ $le(addr) ; 7

    ; Decrement index X by one
    ; N Z C I D V
    ; # # . . . .
    dex                         => 0xca             ; 2

    ; Decrement index Y by one
    ; N Z C I D V
    ; # # . . . .
    dey                         => 0x88             ; 2

    ; XOR memory with accumulator
    ; N Z C I D V
    ; # # . . . .
    eor #{imm:   i8 }           => 0x49 @ imm       ; 2
    eor <{zaddr: u8 }           => 0x45 @ zaddr     ; 3
    eor <{zaddr: u8 },  x       => 0x55 @ zaddr     ; 4
    eor  {zaddr: u8 }           => 0x45 @ zaddr     ; 3
    eor  {zaddr: u8 },  x       => 0x55 @ zaddr     ; 4
    eor  {addr:  u16}           => 0x4d @ $le(addr) ; 4
    eor  {addr:  u16},  x       => 0x5d @ $le(addr) ; 4+
    eor  {addr:  u16},  y       => 0x59 @ $le(addr) ; 4+
    eor ({zaddr: u8 },  x)      => 0x41 @ zaddr     ; 6
    eor ({zaddr: u8 }), y       => 0x51 @ zaddr     ; 5+

    ; Increment memory by one
    ; N Z C I D V
    ; # # . . . .
    inc <{zaddr: u8 }           => 0xe6 @ zaddr     ; 5
    inc <{zaddr: u8 }, x        => 0xf6 @ zaddr     ; 6
    inc  {zaddr: u8 }           => 0xe6 @ zaddr     ; 5
    inc  {zaddr: u8 }, x        => 0xf6 @ zaddr     ; 6
    inc  {addr:  u16}           => 0xee @ $le(addr) ; 6
    inc  {addr:  u16}, x        => 0xfe @ $le(addr) ; 7

    ; Increment index X by one
    ; N Z C I D V
    ; # # . . . .
    inx                         => 0xe8             ; 2

    ; Increment index Y by one
    ; N Z C I D V
    ; # # . . . .
    iny                         => 0xc8             ; 2

    ; Jump to new location
    ; N Z C I D V
    ; . . . . . .
    jmp  {addr: u16}            => 0x4c @ $le(addr) ; 3
    jmp ({addr: u16})           => 0x6c @ $le(addr) ; 5

    ; Jump to new location saving return address
    ; N Z C I D V
    ; . . . . . .
    jsr {addr: u16}             => 0x20 @ $le(addr) ; 6

    ; Load accumulator with memory
    ; N Z C I D V
    ; # # . . . .
    lda #{imm:   i8 }           => 0xa9 @ imm       ; 2
    lda <{zaddr: u8 }           => 0xa5 @ zaddr     ; 3
    lda <{zaddr: u8 },  x       => 0xb5 @ zaddr     ; 4
    lda  {zaddr: u8 }           => 0xa5 @ zaddr     ; 3
    lda  {zaddr: u8 },  x       => 0xb5 @ zaddr     ; 4
    lda  {addr:  u16}           => 0xad @ $le(addr) ; 4
    lda  {addr:  u16},  x       => 0xbd @ $le(addr) ; 4+
    lda  {addr:  u16},  y       => 0xb9 @ $le(addr) ; 4+
    lda ({zaddr: u8 },  x)      => 0xa1 @ zaddr     ; 6
    lda ({zaddr: u8 }), y       => 0xb1 @ zaddr     ; 5+

    ; Load index X with memory
    ; N Z C I D V
    ; # # . . . .
    ldx #{imm:   i8 }           => 0xa2 @ imm       ; 2
    ldx <{zaddr: u8 }           => 0xa6 @ zaddr     ; 3
    ldx <{zaddr: u8 }, y        => 0xb6 @ zaddr     ; 4
    ldx  {zaddr: u8 }           => 0xa6 @ zaddr     ; 3
    ldx  {zaddr: u8 }, y        => 0xb6 @ zaddr     ; 4
    ldx  {addr:  u16}           => 0xae @ $le(addr) ; 4
    ldx  {addr:  u16}, y        => 0xbe @ $le(addr) ; 4+

    ; Load index Y with memory
    ; N Z C I D V
    ; # # . . . .
    ldy #{imm:   i8 }           => 0xa0 @ imm       ; 2
    ldy <{zaddr: u8 }           => 0xa4 @ zaddr     ; 3
    ldy <{zaddr: u8 }, x        => 0xb4 @ zaddr     ; 4
    ldy  {zaddr: u8 }           => 0xa4 @ zaddr     ; 3
    ldy  {zaddr: u8 }, x        => 0xb4 @ zaddr     ; 4
    ldy  {addr:  u16}           => 0xac @ $le(addr) ; 4
    ldy  {addr:  u16}, x        => 0xbc @ $le(addr) ; 4+

    ; Shift right one bit
    ; N Z C I D V
    ; 0 # # . . .
    lsr  a                      => 0x4a             ; 2
    lsr <{zaddr: u8 }           => 0x46 @ zaddr     ; 5
    lsr <{zaddr: u8 }, x        => 0x56 @ zaddr     ; 6
    lsr  {zaddr: u8 }           => 0x46 @ zaddr     ; 5
    lsr  {zaddr: u8 }, x        => 0x56 @ zaddr     ; 6
    lsr  {addr:  u16}           => 0x4e @ $le(addr) ; 6
    lsr  {addr:  u16}, x        => 0x5e @ $le(addr) ; 7

    ; No operation
    ; N Z C I D V
    ; . . . . . .
    nop                         => 0xea             ; 2

    ; OR memory with accumulator
    ; N Z C I D V
    ; # # . . . .
    ora #{imm:   i8 }           => 0x09 @ imm       ; 2
    ora <{zaddr: u8 }           => 0x05 @ zaddr     ; 3
    ora <{zaddr: u8 },  x       => 0x15 @ zaddr     ; 4
    ora  {zaddr: u8 }           => 0x05 @ zaddr     ; 3
    ora  {zaddr: u8 },  x       => 0x15 @ zaddr     ; 4
    ora  {addr:  u16}           => 0x0d @ $le(addr) ; 4
    ora  {addr:  u16},  x       => 0x1d @ $le(addr) ; 4+
    ora  {addr:  u16},  y       => 0x19 @ $le(addr) ; 4+
    ora ({zaddr: u8 },  x)      => 0x01 @ zaddr     ; 6
    ora ({zaddr: u8 }), y       => 0x11 @ zaddr     ; 5+

    ; Push accumulator on stack
    ; N Z C I D V
    ; . . . . . .
    pha                         => 0x48             ; 3

    ; Push processor status on stack
    ; N Z C I D V
    ; . . . . . .
    php                         => 0x08             ; 3

    ; Pull accumulator from stack
    ; N Z C I D V
    ; # # . . . .
    pla                         => 0x68             ; 4

    ; Pull processor status from stack
    ; N Z C I D V
    ; # # # # # #
    plp                         => 0x28             ; 4

    ; Rotate one bit left
    ; N Z C I D V
    ; # # # . . .
    rol  a                      => 0x2a             ; 2
    rol <{zaddr: u8 }           => 0x26 @ zaddr     ; 5
    rol <{zaddr: u8 }, x        => 0x36 @ zaddr     ; 6
    rol  {zaddr: u8 }           => 0x26 @ zaddr     ; 5
    rol  {zaddr: u8 }, x        => 0x36 @ zaddr     ; 6
    rol  {addr:  u16}           => 0x2e @ $le(addr) ; 6
    rol  {addr:  u16}, x        => 0x3e @ $le(addr) ; 7

    ; Rotate one bit right
    ; N Z C I D V
    ; # # # . . .
    ror  a                      => 0x6a             ; 2
    ror <{zaddr: u8 }           => 0x66 @ zaddr     ; 5
    ror <{zaddr: u8 }, x        => 0x76 @ zaddr     ; 6
    ror  {zaddr: u8 }           => 0x66 @ zaddr     ; 5
    ror  {zaddr: u8 }, x        => 0x76 @ zaddr     ; 6
    ror  {addr:  u16}           => 0x6e @ $le(addr) ; 6
    ror  {addr:  u16}, x        => 0x7e @ $le(addr) ; 7

    ; Return from interrupt
    ; N Z C I D V
    ; # # # # # #
    rti                         => 0x40             ; 6

    ; Return from subroutine
    ; N Z C I D V
    ; . . . . . .
    rts                         => 0x60             ; 6

    ; Subtrack memory from accumulator with borrow
    ; N Z C I D V
    ; # # # . . #
    sbc #{imm:   i8 }           => 0xe9 @ imm       ; 2
    sbc <{zaddr: u8 }           => 0xe5 @ zaddr     ; 3
    sbc <{zaddr: u8 },  x       => 0xf5 @ zaddr     ; 4
    sbc  {zaddr: u8 }           => 0xe5 @ zaddr     ; 3
    sbc  {zaddr: u8 },  x       => 0xf5 @ zaddr     ; 4
    sbc  {addr:  u16}           => 0xed @ $le(addr) ; 4
    sbc  {addr:  u16},  x       => 0xfd @ $le(addr) ; 4+
    sbc  {addr:  u16},  y       => 0xf9 @ $le(addr) ; 4+
    sbc ({zaddr: u8 },  x)      => 0xe1 @ zaddr     ; 6
    sbc ({zaddr: u8 }), y       => 0xf1 @ zaddr     ; 5+

    ; Set carry flag
    ; N Z C I D V
    ; . . 1 . . .
    sec                         => 0x38             ; 2

    ; Set decimal mode
    ; N Z C I D V
    ; . . . . 1 .
    sed                         => 0xf8             ; 2

    ; Set interrupt disable status
    ; N Z C I D V
    ; . . . 1 . .
    sei                         => 0x78             ; 2

    ; Store accumulator in memory
    ; N Z C I D V
    ; . . . . . .
    sta <{zaddr: u8 }           => 0x85 @ zaddr     ; 3
    sta <{zaddr: u8 },  x       => 0x95 @ zaddr     ; 4
    sta  {zaddr: u8 }           => 0x85 @ zaddr     ; 3
    sta  {zaddr: u8 },  x       => 0x95 @ zaddr     ; 4
    sta  {addr:  u16}           => 0x8d @ $le(addr) ; 4
    sta  {addr:  u16},  x       => 0x9d @ $le(addr) ; 5
    sta  {addr:  u16},  y       => 0x99 @ $le(addr) ; 5
    sta ({zaddr: u8 },  x)      => 0x81 @ zaddr     ; 6
    sta ({zaddr: u8 }), y       => 0x91 @ zaddr     ; 6

    ; Store index X in memory
    ; N Z C I D V
    ; . . . . . .
    stx <{zaddr: u8 }           => 0x86 @ zaddr     ; 3
    stx <{zaddr: u8 }, y        => 0x96 @ zaddr     ; 4
    stx  {zaddr: u8 }           => 0x86 @ zaddr     ; 3
    stx  {zaddr: u8 }, y        => 0x96 @ zaddr     ; 4
    stx  {addr:  u16}           => 0x8e @ $le(addr) ; 4

    ; Store index Y in memory
    ; N Z C I D V
    ; . . . . . .
    sty <{zaddr: u8 }           => 0x84 @ zaddr     ; 3
    sty <{zaddr: u8 }, x        => 0x94 @ zaddr     ; 4
    sty  {zaddr: u8 }           => 0x84 @ zaddr     ; 3
    sty  {zaddr: u8 }, x        => 0x94 @ zaddr     ; 4
    sty  {addr:  u16}           => 0x8c @ $le(addr) ; 4

    ; Transfer accumulator to index X
    ; N Z C I D V
    ; # # . . . .
    tax                         => 0xaa             ; 2

    ; Transfer accumulator to index Y
    ; N Z C I D V
    ; # # . . . .
    tay                         => 0xa8             ; 2

    ; Transfer stack pointer to index X
    ; N Z C I D V
    ; # # . . . .
    tsx                         => 0xba             ; 2

    ; Transfer index X to accumulator
    ; N Z C I D V
    ; # # . . . .
    txa                         => 0x8a             ; 2

    ; Transfer index X to stack pointer
    ; N Z C I D V
    ; . . . . . .
    txs                         => 0x9a             ; 2

    ; Transfer index Y to accumulator
    ; N Z C I D V
    ; # # . . . .
    tya                         => 0x98             ; 2
}

; Extra aliases
#ruledef cpu6502_aliases
{
    blt {addr: u16} => asm { bcc {addr} }
    bge {addr: u16} => asm { bcs {addr} }
}