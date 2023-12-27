const ZERO_FLAG: u16 = 0x7; // z
const SUBTRACTION_FLAG: u16 = 0x6; // n
const HALF_CARRY_FLAG: u16 = 0x5; // h
const CARRY_FLAG: u16 = 0x4; // c

struct CPU {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
        }
    }

    
}