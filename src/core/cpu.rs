use crate::core::bus::Memory;
use crate::core::bus::BusAccess;

pub struct ARM7TDMI {
    // register indexes are as follows
    // low regs are 0-7
    // high regs user/system are 8-16
    // r13_irq, r14_irq, and spsr_irq are 17, 18, and 19
    // fiq registers are 20-26, spsr_irq is 27
    // r13_svc, r14_svc, and spsr_svc are 28, 29, and 30
    // r13_abt, r14_abt, and spsr_abt are 31, 32, and 33
    // r13_und, r14_und, and r15_und are 34, 35, and 36
    register: [u32; 37],
    mode_register: [usize; 17], // this array is of indexes for register
}

impl Default for ARM7TDMI {
    fn default() -> ARM7TDMI {
        let mut cpu = ARM7TDMI { 
            register: [0; 37],
            mode_register: [0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14, 15, 16],
        };

        cpu.register[13] = 0x0300_7F00;
        cpu.register[15] = 0x0800_0000;
        cpu.register[16] = 0b0000_0000_0000_0000_0000_0000_0001_0011;
        cpu.register[17] = 0x0300_7FA0;
        cpu.register[34] = 0x0300_7FE0;

        cpu
    }
}
