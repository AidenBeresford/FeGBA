use crate::core::bus::Memory;
use crate::core::bus::BusAccess;
use crate::constants::register_index;
use crate::constants::register_initial;
use crate::constants::flag_masks;
use crate::constants::CONDITION_MASK;
use crate::constants::condition_codes;

pub struct ARM7TDMI {
    // register indexes are as follows
    // low regs are 0-7 inclusive
    // usr/sys high general-purpose regs are 8-12 inclusive
    // fiq high general-purpose regs are 20-24 inclusive
    register: [u32; 37],
    idx: [usize; 17], // this array is of indexes for register
}

impl Default for ARM7TDMI {
    fn default() -> ARM7TDMI {
        let mut cpu = ARM7TDMI { 
            register: [0; 37],
            idx: [0, 1, 2, 3, 4, 5, 6, 7,
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

impl ARM7TDMI {
    fn set_flag(&mut self, flag: Flag, bit: bool) {
        let mask = flag.get_mask();
        if bit == true {
            self.register[16] |= mask;
        } else {
            self.register[16] &= !mask;
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let mask = flag.get_mask();
        (self.register[16] & mask) != 0
    }

    fn pass_condition(&self, opcode: u32) -> bool {
        let condition = opcode & CONDITION_MASK;
        let n = get_flag(self.register[16], Flag::N);
        let z = get_flag(self.register[16], Flag::Z);
        let c = get_flag(self.register[16], Flag::C);
        let v = get_flag(self.register[16], Flag::V);
    
        match condition {
            condition_codes::EQ => z,
            condition_codes::NQ => !z,
            condition_codes::CS_HS => c,
            condition_codes::CC_LO => !c,
            condition_codes::MI => n,
            condition_codes::PL => !n,
            condition_codes::VS => v,
            condition_codes::VC => !v,
            condition_codes::HI => c && !z,
            condition_codes::LS => !c && z,
            condition_codes::GE => n == v,
            condition_codes::LT => n != v,
            condition_codes::GT => !z && (n == v),
            condition_codes::LE => z && (n != v),
            condition_codes::AL => true,
            _ => false,
        }
    }
}

enum Flag {
    N,
    Z,
    C,
    V,
    Q
}

impl Flag {
    fn get_mask(&self) -> u32 {
        match self {
            Flag::N => flag_masks::N,
            Flag::Z => flag_masks::Z,
            Flag::C => flag_masks::C,
            Flag::V => flag_masks::V,
            Flag::Q => flag_masks::Q,
        }
    }
}
