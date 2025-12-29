#once


#include "cpu.asm"


#subruledef reg_x
{
    x => 0`0
}

#subruledef reg_y
{
    y => 0`0
}


#ruledef
{
    ; MARK: loading
    load A with im({imm:   i8 })              => asm { lda #{imm  }      }
    load A with zp({zaddr: u8 })              => asm { lda <{zaddr}      }
    load A with zp({zaddr: u8 })[{x: reg_x}]  => asm { lda <{zaddr},  x  }
    load A with    {addr:  u16}               => asm { lda  {addr }      }
    load A with    {addr:  u16}[{x: reg_x}]   => asm { lda  {addr },  x  }
    load A with    {addr:  u16}[{y: reg_y}]   => asm { lda  {addr },  y  }
    load A with  *({zaddr: u8 }[{x: reg_x}])  => asm { lda ({zaddr},  x) }
    load A with  *({zaddr: u8 })[{y: reg_y}]  => asm { lda ({zaddr}), y  }

    load X with im({imm:   i8 })             => asm { ldx #{imm  }    }
    load X with zp({zaddr: u8 })             => asm { ldx <{zaddr}    }
    load X with zp({zaddr: u8 })[{y: reg_y}] => asm { ldx <{zaddr}, y }
    load X with    {addr:  u16}              => asm { ldx  {addr }    }
    load X with    {addr:  u16}[{y: reg_y}]  => asm { ldx  {addr }, y }

    load Y with im({imm:   i8 })             => asm { ldy #{imm  }    }
    load Y with zp({zaddr: u8 })             => asm { ldy <{zaddr}    }
    load Y with zp({zaddr: u8 })[{x: reg_x}] => asm { ldy <{zaddr}, x }
    load Y with    {addr:  u16}              => asm { ldy  {addr }    }
    load Y with    {addr:  u16}[{x: reg_x}]  => asm { ldy  {addr }, x }

    ; MARK: storing
    store A in zp({zaddr: u8 })              => asm { sta <{zaddr}      } 
    store A in zp({zaddr: u8 })[{x: reg_x}]  => asm { sta <{zaddr},  x  } 
    store A in    {addr:  u16}               => asm { sta  {addr }      }
    store A in    {addr:  u16}[{x: reg_x}]   => asm { sta  {addr },  x  }
    store A in    {addr:  u16}[{y: reg_y}]   => asm { sta  {addr },  y  }
    store A in  *({zaddr: u8 }[{x: reg_x}])  => asm { sta ({zaddr},  x) } 
    store A in  *({zaddr: u8 })[{y: reg_y}]  => asm { sta ({zaddr}), y  } 

    store X in zp({zaddr: u8 })             => asm { stx <{zaddr}    }
    store X in zp({zaddr: u8 })[{y: reg_y}] => asm { stx <{zaddr}, y }
    store X in    {addr:  u16}              => asm { stx  {addr }    }

    store Y in zp({zaddr: u8 })             => asm { sty <{zaddr}    }
    store Y in zp({zaddr: u8 })[{x: reg_x}] => asm { sty <{zaddr}, x }
    store Y in    {addr:  u16}              => asm { sty  {addr }    }

    ; MARK: branching
    if     carry    {addr: cpu6502_reladdr} => asm { bcs {addr} }
    if not carry    {addr: cpu6502_reladdr} => asm { bcc {addr} }
    if     zero     {addr: cpu6502_reladdr} => asm { beq {addr} }
    if not zero     {addr: cpu6502_reladdr} => asm { bne {addr} }
    if     overflow {addr: cpu6502_reladdr} => asm { bvs {addr} }
    if not overflow {addr: cpu6502_reladdr} => asm { bvc {addr} }
    if     positive {addr: cpu6502_reladdr} => asm { bpl {addr} }
    if     negative {addr: cpu6502_reladdr} => asm { bmi {addr} }
    if     (+)      {addr: cpu6502_reladdr} => asm { bpl {addr} }
    if     (-)      {addr: cpu6502_reladdr} => asm { bmi {addr} }
    if     eq       {addr: cpu6502_reladdr} => asm { beq {addr} }
    if     diff     {addr: cpu6502_reladdr} => asm { bne {addr} }
    if     lt       {addr: cpu6502_reladdr} => asm { bcc {addr} }
    if     ge       {addr: cpu6502_reladdr} => asm { bcs {addr} }
    if     (<)      {addr: cpu6502_reladdr} => asm { bcc {addr} }
    if     (>~)     {addr: cpu6502_reladdr} => asm { bcs {addr} }

    ; MARK: addition
    A + im({imm:   i8 })              => asm { adc #{imm  }      }
    A + zp({zaddr: u8 })              => asm { adc <{zaddr}      }
    A + zp({zaddr: u8 })[{x: reg_x}]  => asm { adc <{zaddr},  x  }
    A +    {addr:  u16}               => asm { adc  {addr }      }
    A +    {addr:  u16}[{x: reg_x}]   => asm { adc  {addr },  x  }
    A +    {addr:  u16}[{y: reg_y}]   => asm { adc  {addr },  y  }
    A +  *({zaddr: u8 }[{x: reg_x}])  => asm { adc ({zaddr},  x) }
    A +  *({zaddr: u8 })[{y: reg_y}]  => asm { adc ({zaddr}), y  }

    ; MARK: subtraction
    A - im({imm:   i8 })              => asm { sbc #{imm  }      }
    A - zp({zaddr: u8 })              => asm { sbc <{zaddr}      }
    A - zp({zaddr: u8 })[{x: reg_x}]  => asm { sbc <{zaddr},  x  }
    A -    {addr:  u16}               => asm { sbc  {addr }      }
    A -    {addr:  u16}[{x: reg_x}]   => asm { sbc  {addr },  x  }
    A -    {addr:  u16}[{y: reg_y}]   => asm { sbc  {addr },  y  }
    A -  *({zaddr: u8 }[{x: reg_x}])  => asm { sbc ({zaddr},  x) }
    A -  *({zaddr: u8 })[{y: reg_y}]  => asm { sbc ({zaddr}), y  }

    ; MARK: logical AND
    A and im({imm:   i8 })             => asm { and #{imm  }      }
    A and zp({zaddr: u8 })             => asm { and <{zaddr}      }
    A and zp({zaddr: u8 })[{x: reg_x}] => asm { and <{zaddr},  x  }
    A and    {addr:  u16}              => asm { and  {addr }      }
    A and    {addr:  u16}[{x: reg_x}]  => asm { and  {addr },  x  }
    A and    {addr:  u16}[{y: reg_y}]  => asm { and  {addr },  y  }
    A and  *({zaddr: u8 }[{x: reg_x}]) => asm { and ({zaddr},  x) }
    A and  *({zaddr: u8 })[{y: reg_y}] => asm { and ({zaddr}), y  }

    ; MARK: logical OR
    A or im({imm:   i8 })             => asm { ora #{imm  }      }
    A or zp({zaddr: u8 })             => asm { ora <{zaddr}      }
    A or zp({zaddr: u8 })[{x: reg_x}] => asm { ora <{zaddr},  x  }
    A or    {zaddr: u8 }              => asm { ora  {zaddr}      }
    A or    {zaddr: u8 }[{x: reg_x}]  => asm { ora  {zaddr},  x  }
    A or    {addr:  u16}              => asm { ora  {addr }      }
    A or    {addr:  u16}[{x: reg_x}]  => asm { ora  {addr },  x  }
    A or    {addr:  u16}[{y: reg_y}]  => asm { ora  {addr },  y  }
    A or  *({zaddr: u8 }[{x: reg_x}]) => asm { ora ({zaddr},  x) }
    A or  *({zaddr: u8 })[{y: reg_y}] => asm { ora ({zaddr}), y  }

    ; MARK: logical XOR
    A xor im({imm:   i8 })             => asm { eor #{imm  }      }
    A xor zp({zaddr: u8 })             => asm { eor <{zaddr}      }
    A xor zp({zaddr: u8 })[{x: reg_x}] => asm { eor <{zaddr},  x  }
    A xor    {zaddr: u8 }              => asm { eor  {zaddr}      }
    A xor    {zaddr: u8 }[{x: reg_x}]  => asm { eor  {zaddr},  x  }
    A xor    {addr:  u16}              => asm { eor  {addr }      }
    A xor    {addr:  u16}[{x: reg_x}]  => asm { eor  {addr },  x  }
    A xor    {addr:  u16}[{y: reg_y}]  => asm { eor  {addr },  y  }
    A xor  *({zaddr: u8 }[{x: reg_x}]) => asm { eor ({zaddr},  x) }
    A xor  *({zaddr: u8 })[{y: reg_y}] => asm { eor ({zaddr}), y  }

    ; MARK: bit shit
    sh_l A                            => asm { asl  a               }
    sh_l zp({zaddr: u8 })             => asm { asl <{zaddr}    }
    sh_l zp({zaddr: u8 })[{x: reg_x}] => asm { asl <{zaddr}, x }
    sh_l    {addr:  u16}              => asm { asl  {addr }    }
    sh_l    {addr:  u16}[{x: reg_x}]  => asm { asl  {addr }, x }

    sh_r A                            => asm { lsr  a               }
    sh_r zp({zaddr: u8 })             => asm { lsr <{zaddr}    }
    sh_r zp({zaddr: u8 })[{x: reg_x}] => asm { lsr <{zaddr}, x }
    sh_r    {addr:  u16}              => asm { lsr  {addr }    }
    sh_r    {addr:  u16}[{x: reg_x}]  => asm { lsr  {addr }, x }

    ; MARK: bit rotation
    rot_l  A                          => asm { rol  a               }
    rot_l zp({zaddr: u8})             => asm { rol <{zaddr}    }
    rot_l zp({zaddr: u8})[{x: reg_x}] => asm { rol <{zaddr}, x }
    rot_l    {addr:  u16}             => asm { rol  {addr }    }
    rot_l    {addr:  u16}[{x: reg_x}] => asm { rol  {addr }, x }

    rot_r  A                          => asm { ror  a               }
    rot_r zp({zaddr: u8})             => asm { ror <{zaddr}    }
    rot_r zp({zaddr: u8})[{x: reg_x}] => asm { ror <{zaddr}, x }
    rot_r    {addr:  u16}             => asm { ror  {addr }    }
    rot_r    {addr:  u16}[{x: reg_x}] => asm { ror  {addr }, x }

    ; MARK: increment
    ++{x: reg_x} => asm { inx }
    ++{y: reg_y} => asm { iny }

    ++zp({zaddr: u8})             => asm { inc <{zaddr}    }
    ++zp({zaddr: u8})[{x: reg_x}] => asm { inc <{zaddr}, x }
    ++{addr: u16}                 => asm { inc  {addr }    }
    ++{addr: u16}[{x: reg_x}]     => asm { inc  {addr }, x }

    ; MARK: decrement
    --{x: reg_x} => asm { dex }
    --{y: reg_y} => asm { dey }

    --zp({zaddr: u8})             => asm { dec <{zaddr}    }
    --zp({zaddr: u8})[{x: reg_x}] => asm { dec <{zaddr}, x }
    --{addr: u16}                 => asm { dec  {addr }    }
    --{addr: u16}[{x: reg_x}]     => asm { dec  {addr }, x }

    ; MARK: comparison
    cmp A with im({imm:   i8 })             => asm { cmp #{imm  }      }
    cmp A with zp({zaddr: u8 })             => asm { cmp <{zaddr}      }
    cmp A with zp({zaddr: u8 })[{x: reg_x}] => asm { cmp <{zaddr},  x  }
    cmp A with    {addr:  u16}              => asm { cmp  {addr }      }
    cmp A with    {addr:  u16}[{x: reg_x}]  => asm { cmp  {addr },  x  }
    cmp A with    {addr:  u16}[{y: reg_y}]  => asm { cmp  {addr },  y  }
    cmp A with   ({zaddr: u8 }[{x: reg_x}]) => asm { cmp ({zaddr},  x) }
    cmp A with   ({zaddr: u8 })[{y: reg_y}] => asm { cmp ({zaddr}), y  }

    cmp X im({imm:   i8 }) => asm { cpx #{imm  } }
    cmp X zp({zaddr: u8 }) => asm { cpx <{zaddr} }
    cmp X    {addr:  u16}  => asm { cpx  {addr } }

    cmp Y im({imm:   i8 }) => asm { cpy #{imm  } }
    cmp Y zp({zaddr: u8 }) => asm { cpy <{zaddr} }
    cmp Y    {addr:  u16}  => asm { cpy  {addr } }

    ; status register
    set     carry     => asm { sec }
    clear   carry     => asm { clc }
    set     decimal   => asm { sed }
    clear   decimal   => asm { cld }
    disable interrupt => asm { sei }
    enable  interrupt => asm { cli }
    clear   overflow  => asm { clv }

    ; transfer operations
    transfer A     to X     => asm { tax }
    transfer X     to A     => asm { txa }
    transfer A     to Y     => asm { tay }
    transfer Y     to A     => asm { tya }
    transfer Stack to X     => asm { tsx }
    transfer X     to Stack => asm { txs }

    ; stack operations
    push A      => asm { pha }
    pop  A      => asm { pla }
    push Status => asm { php }
    pop  Status => asm { plp }

    ; jumps and returns
    goto   {addr: u16}    => asm { jmp  {addr}  }
    goto *({addr: u16})   => asm { jmp ({addr}) }
    {addr: u16}()         => asm { jsr  {addr}  }
    force       interrupt => asm { brk }
    return from interrupt => asm { rti }
    return                => asm { rts }
}
