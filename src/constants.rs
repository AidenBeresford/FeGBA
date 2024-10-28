pub mod flag_masks {
    pub const N: u32 = 1 << 31;
    pub const Z: u32 = 1 << 30;
    pub const C: u32 = 1 << 29;
    pub const V: u32 = 1 << 28;
    pub const Q: u32 = 1 << 27;
}

pub const CONDITION_MASK: u32 = 0xF << 28;

pub mod condition_codes {
    pub const EQ: u32 = 0b0000 << 28;
    pub const NQ: u32 = 0b0001 << 28;
    pub const CS_HS: u32 = 0b0010 << 28;
    pub const CC_LO: u32 = 0b0011 << 28;
    pub const MI: u32 = 0b0100 << 28;
    pub const PL: u32 = 0b0100 << 28;
    pub const VS: u32 = 0b0100 << 28;
    pub const VC: u32 = 0b0100 << 28;
    pub const HI: u32 = 0b0100 << 28;
    pub const LS: u32 = 0b0100 << 28;
    pub const GE: u32 = 0b0100 << 28;
    pub const LT: u32 = 0b0100 << 28;
    pub const GT: u32 = 0b0100 << 28;
    pub const LE: u32 = 0b0100 << 28;
    pub const AL: u32 = 0b0100 << 28;
}