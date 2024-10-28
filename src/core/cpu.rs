use crate::core::bus::Memory;
use crate::core::bus::BusAccess;
use crate::constants::register_index;
use crate::constants::register_initial;
use crate::constants::flag_masks;
use crate::constants::CONDITION_MASK;
use crate::constants::condition_codes;

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

/// TODO: cpsr hould be replaced with the actual instance vairable 
fn set_flag(cpsr: &mut u32, flag: Flag, bit: bool) {
    let mask = flag.get_mask();
    if bit == true {
        *cpsr |= mask;
    } else {
        *cpsr &= !mask;
    }
}

/// TODO: cpsr should be replaced with the actual instance vairable 
fn get_flag(cpsr: u32, flag: Flag) -> bool {
    let mask = flag.get_mask();
    (cpsr & mask) != 0
}

/// TODO: cpsr should be replaced with the actual instance vairable 
fn pass_condition(opcode: u32, cpsr: u32) -> bool {
    let condition = opcode & CONDITION_MASK;
    let n = get_flag(cpsr, Flag::N);
    let z = get_flag(cpsr, Flag::Z);
    let c = get_flag(cpsr, Flag::C);
    let v = get_flag(cpsr, Flag::V);

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
