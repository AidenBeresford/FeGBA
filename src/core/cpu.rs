use crate::constants;
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
    spsr: usize, // spsr access in user/system is UB
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
            spsr: register_index::SPSR_UND,
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
    // ARM INSTRUCTIONS
    fn BX(&mut self, opcode: u32) {
        let rm: usize = (opcode & 0b0111) as usize;
        if self.pass_condition(opcode) {
            self.set_flag(Flag::T, (self.register[self.idx[rm]] & 1) != 0);
            self.register[15] = self.register[self.idx[rm]] & 0xFFFF_FFFE;
        }
    }
    
    fn MLA(&mut self, opcode: u32) {
        if self.pass_condition(opcode) {
            let rd = ((opcode >> 16) & 0xF) as usize;
            let rn = ((opcode >> 12) & 0xF) as usize;
            let rs = ((opcode >> 8) & 0xF) as usize;
            let rm = (opcode & 0xF) as usize;
            let s = (opcode >> 20) & 0x1;
    
            self.register[rd] = self.register[self.idx[rm]] * self.register[self.idx[rs]] + self.register[self.idx[rn]];
            if s == 1 {
                self.set_flag(Flag::N, (self.register[self.idx[rd]] >> 31) == 1);
                self.set_flag(Flag::Z, self.register[self.idx[rd]] == 0);
            }
        }
    }

    fn MOV(&mut self, opcode: u32) {
        if self.pass_condition(opcode) {
            let s = (opcode >> 20) & 0x1;
            let rd = (opcode >> 12) & 0xF;
            let shifter = opcode & 0xFFF;

            self.register[self.idx[rd as usize]] = shifter;
            if s == 1 && rd == self.register[15] {
                // CPSR = SPSR
                // Note: not sure if that's the correct SPSR
                self.register[register_index::CPSR] = self.register[register_index::SPSR_UND];
            }
            else if s == 1 {
                // N Flag = Rd[31]
                self.set_flag(Flag::N, rd >> 31 == 1);
                // Z Flag = if Rd == 0 then 1 else 0
                self.set_flag(Flag::Z, rd != 0);
                // C Flag = shifter_carry_out // I need the shifter_carry_out to be implemented
                // self.set_flag(Flag::C, shifter_carry_out);
                // V Flag = unaffected
            }
        }
    }

    fn ORR(&mut self, opcode: u32) {
        if self.pass_condition(opcode) {
            let s = (opcode >> 20) & 0x1;
            let rd = (opcode >> 12) & 0xF;
            let rn = (opcode >> 16) & 0xF;
            let shifter = opcode & 0xFFF;

            // Rd = Rn OR shifter_operand
            self.register[self.idx[rd as usize]] = shifter | rn;
            if s == 1 && rd == self.register[15] {
                // CPSR = SPSR
                self.register[register_index::CPSR] = self.register[register_index::SPSR_UND];
            }
            else if s == 1 {
                // N Flag = Rd[31]
                self.set_flag(Flag::N, rd >> 31 == 1);
                // Z Flag = if Rd == 0 then 1 else 0
                self.set_flag(Flag::Z, rd != 0);
                // C Flag = shifter_carry_out
                // self.set_flag(Flag::C, shifter_carry_out);
                // V Flag = unaffected
            }
        }
    }

    // HELPER FUNCTIONS
    fn set_flag(&mut self, flag: Flag, bit: bool) {
        let mask = flag.get_mask();
        if bit == true {
            self.register[register_index::CPSR] |= mask;
        } else {
            self.register[register_index::CPSR] &= !mask;
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let mask = flag.get_mask();
        (self.register[register_index::CPSR] & mask) != 0
    }

    fn pass_condition(&self, opcode: u32) -> bool {
        let condition = opcode & CONDITION_MASK;
        let n = self.get_flag(Flag::N);
        let z = self.get_flag(Flag::Z);
        let c = self.get_flag(Flag::C);
        let v = self.get_flag(Flag::V);
    
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
    T
}

impl Flag {
    fn get_mask(&self) -> u32 {
        match self {
            Flag::N => flag_masks::N,
            Flag::Z => flag_masks::Z,
            Flag::C => flag_masks::C,
            Flag::V => flag_masks::V,
            Flag::T => flag_masks::T,
        }
    }
}
