pub mod register_index {
    pub const SP_USR: usize = 13;
    pub const LR_USR: usize = 14;
    pub const PC: usize = 15;
    pub const CPSR: usize = 16;

    pub const SP_IRQ: usize = 17;
    pub const LR_IRQ: usize = 18;
    pub const SPSR_IRQ: usize = 19;

    pub const SP_FIQ: usize = 25;
    pub const LR_FIQ: usize = 26;
    pub const SPSR_FIQ: usize = 27;

    pub const SP_SVC: usize = 28;
    pub const LR_SVC: usize = 29;
    pub const SPSR_SVC: usize = 30;

    pub const SP_ABT: usize = 31;
    pub const LR_ABT: usize = 32;
    pub const SPSR_ABT: usize = 33;

    pub const SP_UND: usize = 34;
    pub const LR_UND: usize = 35;
    pub const SPSR_UND: usize = 36;
}

pub mod register_initial {
    pub const SP_USR: u32 = 0x0300_7F00;
    pub const PC: u32 = 0x0800_0000;
    pub const CPSR: u32 = 0b0001_0011;
    pub const SP_IRQ: u32 = 0x0300_7FA0;
    pub const SP_UND: u32 = 0x0300_7FE0;
}

pub mod flag_masks {
    pub const N: u32 = 1 << 31;
    pub const Z: u32 = 1 << 30;
    pub const C: u32 = 1 << 29;
    pub const V: u32 = 1 << 28;
}

pub mod condition_codes {
    pub const EQ: u32 = 0b0000 << 28;
    pub const NQ: u32 = 0b0001 << 28;
    pub const CS_HS: u32 = 0b0010 << 28;
    pub const CC_LO: u32 = 0b0011 << 28;
    pub const MI: u32 = 0b0100 << 28;
    pub const PL: u32 = 0b0101 << 28;
    pub const VS: u32 = 0b0110 << 28;
    pub const VC: u32 = 0b0111 << 28;
    pub const HI: u32 = 0b1000 << 28;
    pub const LS: u32 = 0b1001 << 28;
    pub const GE: u32 = 0b1010 << 28;
    pub const LT: u32 = 0b1011 << 28;
    pub const GT: u32 = 0b1100 << 28;
    pub const LE: u32 = 0b1101 << 28;
    pub const AL: u32 = 0b1110 << 28;
}

pub const CONDITION_MASK: u32 = 0xF << 28;