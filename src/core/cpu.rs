use crate::core::bus::Memory;
use crate::core::bus::BusAccess;
use crate::constants::flag_masks;
use crate::constants::CONDITION_MASK;
use crate::constants::condition_codes;

pub struct ARM7TDMI {
    rlow: [u32; 8], // first 8 registers; accessible by THUMB
    pub r08: [u32; 2], // these arrays are for banking registers
    r09: [u32; 2],
    r10: [u32; 2],
    r11: [u32; 2],
    r12: [u32; 2],
    sp: [u32; 6],
    lr: [u32; 6],
    pc: u32,
    cpsr: u32,
    spsr: [u32; 5], // if anyone is willing to help cull these down please be my guest
}

impl Default for ARM7TDMI {
    fn default() -> ARM7TDMI {
        ARM7TDMI {
            rlow: [0; 8],
            r08: [0; 2],
            r09: [0; 2],
            r10: [0; 2],
            r11: [0; 2],
            r12: [0; 2],
            sp: [0x0300_7F00, 0x0300_7FA0, 0, 0, 0, 0x0300_7FE0],
            lr: [0; 6],
            pc: 0x0800_0000,
            cpsr: 0b0000_0000_0000_0000_0000_0000_0001_0011,
            spsr: [0; 5],
        }
    }
}

impl BusAccess for ARM7TDMI {
    fn lbyte(&mut self, memory: &Memory, addr: usize) {
        self.r08[0] = memory.rbyte(addr) as u32;
    }

    fn sbyte(&self, memory: &mut Memory, addr: usize) {
        memory.wbyte(addr, self.r08[0] as u8)
    }

}

impl ARM7TDMI {
    pub fn wreg(&mut self, reg: usize, data: u32) {
        self.rlow[reg] = data;
    }
    
    pub fn rreg(&self, reg: usize) -> u32 {
        self.rlow[reg]
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
