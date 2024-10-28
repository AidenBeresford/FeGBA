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
