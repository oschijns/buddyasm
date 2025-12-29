#once

; low byte of a 16-bits word
#fn lo(word) => word[ 7:0]

; high byte of a 16-bits word
#fn hi(word) => word[15:8]

; Repeat a sequence of expressions N times
; Example usage: `#d repeat(3, asm {nop})`
#fn repeat(n, expr) => {
    n > 0 
        ? expr @ repeat(n - 1, insts) 
        : 0`0
}

