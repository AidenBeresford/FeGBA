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
    pub const I: u32 = 1 << 7;
    pub const F: u32 = 1 << 6;
    pub const T: u32 = 1 << 5;
}

pub mod condition_codes {
    pub const EQ: u32 = 0b0000 << 28;
    pub const NE: u32 = 0b0001 << 28;
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

// CPU Mode constants
pub mod cpu_mode {
    pub const USR: u32 = 0b10000;
    pub const FIQ: u32 = 0b10001;
    pub const IRQ: u32 = 0b10010;
    pub const SVC: u32 = 0b10011;
    pub const ABT: u32 = 0b10111;
    pub const UND: u32 = 0b11011;
    pub const SYS: u32 = 0b11111;
}

// Memory Map Constants
pub mod memory {
    pub const BIOS_START: u32 = 0x0000_0000;
    pub const BIOS_END: u32 = 0x0000_3FFF;
    pub const BIOS_SIZE: usize = 0x4000;

    pub const EWRAM_START: u32 = 0x0200_0000;
    pub const EWRAM_END: u32 = 0x0203_FFFF;
    pub const EWRAM_SIZE: usize = 0x40000;

    pub const IWRAM_START: u32 = 0x0300_0000;
    pub const IWRAM_END: u32 = 0x0300_7FFF;
    pub const IWRAM_SIZE: usize = 0x8000;

    pub const IO_START: u32 = 0x0400_0000;
    pub const IO_END: u32 = 0x0400_03FF;
    pub const IO_SIZE: usize = 0x400;

    pub const PALETTE_START: u32 = 0x0500_0000;
    pub const PALETTE_END: u32 = 0x0500_03FF;
    pub const PALETTE_SIZE: usize = 0x400;

    pub const VRAM_START: u32 = 0x0600_0000;
    pub const VRAM_END: u32 = 0x0601_7FFF;
    pub const VRAM_SIZE: usize = 0x18000;

    pub const OAM_START: u32 = 0x0700_0000;
    pub const OAM_END: u32 = 0x0700_03FF;
    pub const OAM_SIZE: usize = 0x400;

    pub const ROM_START: u32 = 0x0800_0000;
    pub const ROM_END: u32 = 0x09FF_FFFF;
    pub const ROM_SIZE: usize = 0x2000000;

    pub const SRAM_START: u32 = 0x0E00_0000;
    pub const SRAM_END: u32 = 0x0E00_FFFF;
    pub const SRAM_SIZE: usize = 0x10000;
}
