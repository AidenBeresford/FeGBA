use crate::core::bus::Memory;
use crate::core::bus::BusAccess;
use crate::constants::register_index;
use crate::constants::register_initial;

pub struct ARM7TDMI {
    // register indexes are as follows
    // low regs are 0-7
    // high general-purpose regs user/system are 8-12
    // fiq registers are 20-26
    register: [u32; 37],
    mode_register: [usize; 17], // this array is of indexes for register
}

impl Default for ARM7TDMI {
    fn default() -> ARM7TDMI {
        let mut cpu = ARM7TDMI { 
            register: [0; 37],
            mode_register: [0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 
            register_index::SP_USR, 
            register_index::LR_USR, 
            register_index::PC, 
            register_index::CPSR],
        };

        cpu.register[13] = register_initial::SP_USR;
        cpu.register[15] = register_initial::PC;
        cpu.register[16] = register_initial::CPSR;
        cpu.register[17] = register_initial::SP_IRQ;
        cpu.register[34] = register_initial::SP_UND;

        cpu
    }
}
