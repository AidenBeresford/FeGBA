use crate::core::bus::Memory;
use crate::core::addressing_modes::addressing_mode_1::addressing_mode_1;
use crate::core::addressing_modes::addressing_mode_2::addressing_mode_2;
use crate::core::addressing_modes::addressing_mode_3::addressing_mode_3;
use crate::core::addressing_modes::addressing_mode_4::addressing_mode_4;
use crate::core::disassembler::{disassemble, InstructionSet, Instruction};
use crate::constants::register_index;
use crate::constants::register_initial;
use crate::constants::flag_masks;
use crate::constants::CONDITION_MASK;
use crate::constants::condition_codes;

pub struct ARM7TDMI {
    pub register: [u32; 37],
    pub idx: [usize; 17],
    pub spsr: usize,
    pub pipeline: [u32; 2],
    pub halted: bool,
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
            pipeline: [0; 2],
            halted: false,
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
    pub fn step(&mut self, memory: &mut Memory) {
        if self.halted {
            return;
        }

        let pc = self.register[register_index::PC];
        let is_thumb = self.get_flag(Flag::T);

        if is_thumb {
            let opcode = memory.read_halfword(pc) as u16;
            self.execute_thumb(opcode, memory);
            self.register[register_index::PC] = self.register[register_index::PC].wrapping_add(2);
        } else {
            let opcode = memory.read_word(pc);
            self.execute_arm(opcode, memory);
            self.register[register_index::PC] = self.register[register_index::PC].wrapping_add(4);
        }
    }

    fn execute_arm(&mut self, opcode: u32, memory: &mut Memory) {
        let instruction = disassemble(InstructionSet::ARM(opcode));
        
        match instruction {
            Instruction::BranchAndBranchExchange => self.bx(opcode),
            Instruction::BlockDataTransfer => self.block_data_transfer(opcode, memory),
            Instruction::BranchAndBranchWithLink => self.branch(opcode),
            Instruction::SoftwareInterruptA => self.swi(opcode),
            Instruction::SingleDataTransfer => self.single_data_transfer(opcode, memory),
            Instruction::SingleDataSwap => self.swp(opcode, memory),
            Instruction::MultiplyAndMultiplyLong => self.multiply(opcode),
            Instruction::HalfwordDataTransferR => self.halfword_transfer_reg(opcode, memory),
            Instruction::HalfwordDataTransferI => self.halfword_transfer_imm(opcode, memory),
            Instruction::PSRTransferMRS => self.mrs(opcode),
            Instruction::PSRTransferMSR => self.msr(opcode),
            Instruction::DataProcessing => self.data_processing(opcode),
            _ => {
                println!("Undefined ARM instruction: {:#010x}", opcode);
            }
        }
    }

    fn execute_thumb(&mut self, opcode: u16, memory: &mut Memory) {
        let instruction = disassemble(InstructionSet::THUMB(opcode));
        
        match instruction {
            Instruction::MoveShiftedRegister => self.thumb_move_shifted_register(opcode),
            Instruction::AddSubtract => self.thumb_add_subtract(opcode),
            Instruction::MoveCompareAddSubImmediate => self.thumb_move_compare_add_sub_imm(opcode),
            Instruction::ALUOperations => self.thumb_alu_operations(opcode),
            Instruction::HiRegisterOperation => self.thumb_hi_register_ops(opcode),
            Instruction::PCRelativeLoad => self.thumb_pc_relative_load(opcode, memory),
            Instruction::LoadStoreRegisterOffset => self.thumb_load_store_reg_offset(opcode, memory),
            Instruction::LoadStoreSignExtended => self.thumb_load_store_sign_extended(opcode, memory),
            Instruction::LoadStoreImmediateOffset => self.thumb_load_store_imm_offset(opcode, memory),
            Instruction::LoadStoreHalfword => self.thumb_load_store_halfword(opcode, memory),
            Instruction::SPRelativeLoadStore => self.thumb_sp_relative_load_store(opcode, memory),
            Instruction::LoadAddress => self.thumb_load_address(opcode),
            Instruction::AddOffsetToSP => self.thumb_add_offset_to_sp(opcode),
            Instruction::PushPopRegister => self.thumb_push_pop(opcode, memory),
            Instruction::MultipleLoadStore => self.thumb_multiple_load_store(opcode, memory),
            Instruction::ConditionalBranch => self.thumb_conditional_branch(opcode),
            Instruction::UnconditionalBranch => self.thumb_unconditional_branch(opcode),
            Instruction::LongBranchWithLink => self.thumb_long_branch_with_link(opcode),
            Instruction::SoftwareInterruptT => self.swi_thumb(opcode),
            _ => {
                println!("Undefined THUMB instruction: {:#06x}", opcode);
            }
        }
    }

    // ARM INSTRUCTIONS
    fn data_processing(&mut self, opcode: u32) {
        if !self.pass_condition(opcode) {
            return;
        }

        let alu_op = (opcode >> 21) & 0xF;
        let s_bit = ((opcode >> 20) & 1) == 1;
        let rn = ((opcode >> 16) & 0xF) as usize;
        let rd = ((opcode >> 12) & 0xF) as usize;
        
        let mut operand2 = 0u32;
        let carry_out = addressing_mode_1(self, opcode, &mut operand2);
        
        let rn_val = self.register[self.idx[rn]];

        match alu_op {
            0x0 => self.and(rd, rn_val, operand2, s_bit, carry_out), // AND
            0x1 => self.eor(rd, rn_val, operand2, s_bit, carry_out), // EOR
            0x2 => self.sub(rd, rn_val, operand2, s_bit), // SUB
            0x3 => self.rsb(rd, rn_val, operand2, s_bit), // RSB
            0x4 => self.add(rd, rn_val, operand2, s_bit), // ADD
            0x5 => self.adc(rd, rn_val, operand2, s_bit), // ADC
            0x6 => self.sbc(rd, rn_val, operand2, s_bit), // SBC
            0x7 => self.rsc(rd, rn_val, operand2, s_bit), // RSC
            0x8 => self.tst(rn_val, operand2, carry_out), // TST
            0x9 => self.teq(rn_val, operand2, carry_out), // TEQ
            0xA => self.cmp(rn_val, operand2), // CMP
            0xB => self.cmn(rn_val, operand2), // CMN
            0xC => self.orr(rd, rn_val, operand2, s_bit, carry_out), // ORR
            0xD => self.mov(rd, operand2, s_bit, carry_out), // MOV
            0xE => self.bic(rd, rn_val, operand2, s_bit, carry_out), // BIC
            0xF => self.mvn(rd, operand2, s_bit, carry_out), // MVN
            _ => {}
        }
    }

    fn and(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool, carry: bool) {
        let result = rn_val & operand2;
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
            }
        }
    }

    fn eor(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool, carry: bool) {
        let result = rn_val ^ operand2;
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
            }
        }
    }

    fn sub(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool) {
        let (result, carry) = rn_val.overflowing_sub(operand2);
        let overflow = ((rn_val ^ operand2) & (rn_val ^ result) & 0x8000_0000) != 0;
        
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !carry);
                self.set_flag(Flag::V, overflow);
            }
        }
    }

    fn rsb(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool) {
        let (result, carry) = operand2.overflowing_sub(rn_val);
        let overflow = ((operand2 ^ rn_val) & (operand2 ^ result) & 0x8000_0000) != 0;
        
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !carry);
                self.set_flag(Flag::V, overflow);
            }
        }
    }

    fn add(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool) {
        let (result, carry) = rn_val.overflowing_add(operand2);
        let overflow = (!((rn_val ^ operand2) & 0x8000_0000) & ((rn_val ^ result) & 0x8000_0000)) != 0;
        
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
                self.set_flag(Flag::V, overflow);
            }
        }
    }

    fn adc(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool) {
        let c = if self.get_flag(Flag::C) { 1 } else { 0 };
        let (temp, carry1) = rn_val.overflowing_add(operand2);
        let (result, carry2) = temp.overflowing_add(c);
        let overflow = (!((rn_val ^ operand2) & 0x8000_0000) & ((rn_val ^ result) & 0x8000_0000)) != 0;
        
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry1 || carry2);
                self.set_flag(Flag::V, overflow);
            }
        }
    }

    fn sbc(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool) {
        let c = if self.get_flag(Flag::C) { 0 } else { 1 };
        let (temp, carry1) = rn_val.overflowing_sub(operand2);
        let (result, carry2) = temp.overflowing_sub(c);
        let overflow = ((rn_val ^ operand2) & (rn_val ^ result) & 0x8000_0000) != 0;
        
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !(carry1 || carry2));
                self.set_flag(Flag::V, overflow);
            }
        }
    }

    fn rsc(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool) {
        let c = if self.get_flag(Flag::C) { 0 } else { 1 };
        let (temp, carry1) = operand2.overflowing_sub(rn_val);
        let (result, carry2) = temp.overflowing_sub(c);
        let overflow = ((operand2 ^ rn_val) & (operand2 ^ result) & 0x8000_0000) != 0;
        
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !(carry1 || carry2));
                self.set_flag(Flag::V, overflow);
            }
        }
    }

    fn tst(&mut self, rn_val: u32, operand2: u32, carry: bool) {
        let result = rn_val & operand2;
        self.set_flag(Flag::N, (result >> 31) == 1);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, carry);
    }

    fn teq(&mut self, rn_val: u32, operand2: u32, carry: bool) {
        let result = rn_val ^ operand2;
        self.set_flag(Flag::N, (result >> 31) == 1);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, carry);
    }

    fn cmp(&mut self, rn_val: u32, operand2: u32) {
        let (result, carry) = rn_val.overflowing_sub(operand2);
        let overflow = ((rn_val ^ operand2) & (rn_val ^ result) & 0x8000_0000) != 0;
        
        self.set_flag(Flag::N, (result >> 31) == 1);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, !carry);
        self.set_flag(Flag::V, overflow);
    }

    fn cmn(&mut self, rn_val: u32, operand2: u32) {
        let (result, carry) = rn_val.overflowing_add(operand2);
        let overflow = (!((rn_val ^ operand2) & 0x8000_0000) & ((rn_val ^ result) & 0x8000_0000)) != 0;
        
        self.set_flag(Flag::N, (result >> 31) == 1);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, carry);
        self.set_flag(Flag::V, overflow);
    }

    fn orr(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool, carry: bool) {
        let result = rn_val | operand2;
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
            }
        }
    }

    fn mov(&mut self, rd: usize, operand2: u32, s_bit: bool, carry: bool) {
        self.register[self.idx[rd]] = operand2;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (operand2 >> 31) == 1);
                self.set_flag(Flag::Z, operand2 == 0);
                self.set_flag(Flag::C, carry);
            }
        }
    }

    fn bic(&mut self, rd: usize, rn_val: u32, operand2: u32, s_bit: bool, carry: bool) {
        let result = rn_val & !operand2;
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
            }
        }
    }

    fn mvn(&mut self, rd: usize, operand2: u32, s_bit: bool, carry: bool) {
        let result = !operand2;
        self.register[self.idx[rd]] = result;
        
        if s_bit {
            if rd == 15 {
                self.register[register_index::CPSR] = self.register[self.spsr];
            } else {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
            }
        }
    }

    fn multiply(&mut self, opcode: u32) {
        if !self.pass_condition(opcode) {
            return;
        }

        let accumulate = ((opcode >> 21) & 1) == 1;
        let set_flags = ((opcode >> 20) & 1) == 1;
        let long = ((opcode >> 23) & 1) == 1;
        let signed = ((opcode >> 22) & 1) == 1;

        if long {
            let rdhi = ((opcode >> 16) & 0xF) as usize;
            let rdlo = ((opcode >> 12) & 0xF) as usize;
            let rs = ((opcode >> 8) & 0xF) as usize;
            let rm = (opcode & 0xF) as usize;

            let rm_val = self.register[self.idx[rm]];
            let rs_val = self.register[self.idx[rs]];

            let result = if signed {
                (rm_val as i32 as i64).wrapping_mul(rs_val as i32 as i64) as u64
            } else {
                (rm_val as u64).wrapping_mul(rs_val as u64)
            };

            let mut final_result = result;
            if accumulate {
                let acc = ((self.register[self.idx[rdhi]] as u64) << 32) | (self.register[self.idx[rdlo]] as u64);
                final_result = result.wrapping_add(acc);
            }

            self.register[self.idx[rdlo]] = (final_result & 0xFFFF_FFFF) as u32;
            self.register[self.idx[rdhi]] = (final_result >> 32) as u32;

            if set_flags {
                self.set_flag(Flag::N, (final_result >> 63) == 1);
                self.set_flag(Flag::Z, final_result == 0);
            }
        } else {
            let rd = ((opcode >> 16) & 0xF) as usize;
            let rn = ((opcode >> 12) & 0xF) as usize;
            let rs = ((opcode >> 8) & 0xF) as usize;
            let rm = (opcode & 0xF) as usize;

            let rm_val = self.register[self.idx[rm]];
            let rs_val = self.register[self.idx[rs]];

            let mut result = rm_val.wrapping_mul(rs_val);
            
            if accumulate {
                let rn_val = self.register[self.idx[rn]];
                result = result.wrapping_add(rn_val);
            }

            self.register[self.idx[rd]] = result;

            if set_flags {
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            }
        }
    }

    fn single_data_transfer(&mut self, opcode: u32, memory: &mut Memory) {
        if !self.pass_condition(opcode) {
            return;
        }

        let load = ((opcode >> 20) & 1) == 1;
        let byte = ((opcode >> 22) & 1) == 1;
        let rd = ((opcode >> 12) & 0xF) as usize;

        let mut address = 0u32;
        addressing_mode_2(self, opcode, &mut address);

        if load {
            if byte {
                self.register[self.idx[rd]] = memory.read_byte(address) as u32;
            } else {
                self.register[self.idx[rd]] = memory.read_word(address);
            }
        } else {
            let value = self.register[self.idx[rd]];
            if byte {
                memory.write_byte(address, (value & 0xFF) as u8);
            } else {
                memory.write_word(address, value);
            }
        }
    }

    fn halfword_transfer_reg(&mut self, opcode: u32, memory: &mut Memory) {
        if !self.pass_condition(opcode) {
            return;
        }

        let load = ((opcode >> 20) & 1) == 1;
        let sh = (opcode >> 5) & 0x3;
        let rd = ((opcode >> 12) & 0xF) as usize;

        let mut address = 0u32;
        addressing_mode_3(self, opcode, &mut address);

        if load {
            match sh {
                0x1 => { // LDRH
                    self.register[self.idx[rd]] = memory.read_halfword(address) as u32;
                },
                0x2 => { // LDRSB
                    let val = memory.read_byte(address) as i8;
                    self.register[self.idx[rd]] = val as i32 as u32;
                },
                0x3 => { // LDRSH
                    let val = memory.read_halfword(address) as i16;
                    self.register[self.idx[rd]] = val as i32 as u32;
                },
                _ => {}
            }
        } else {
            if sh == 0x1 { // STRH
                let value = self.register[self.idx[rd]];
                memory.write_halfword(address, (value & 0xFFFF) as u16);
            }
        }
    }

    fn halfword_transfer_imm(&mut self, opcode: u32, memory: &mut Memory) {
        self.halfword_transfer_reg(opcode, memory);
    }

    fn block_data_transfer(&mut self, opcode: u32, memory: &mut Memory) {
        if !self.pass_condition(opcode) {
            return;
        }

        let load = ((opcode >> 20) & 1) == 1;
        let register_list = (opcode & 0xFFFF) as u16;

        let mut start_addr = 0u32;
        let mut end_addr = 0u32;
        addressing_mode_4(self, opcode, &mut start_addr, &mut end_addr);

        let mut address = start_addr;
        
        for i in 0..16 {
            if (register_list & (1 << i)) != 0 {
                if load {
                    self.register[self.idx[i]] = memory.read_word(address);
                } else {
                    memory.write_word(address, self.register[self.idx[i]]);
                }
                address = address.wrapping_add(4);
            }
        }
    }

    fn branch(&mut self, opcode: u32) {
        if !self.pass_condition(opcode) {
            return;
        }

        let link = ((opcode >> 24) & 1) == 1;
        let mut offset = (opcode & 0x00FF_FFFF) << 2;
        
        if (offset & 0x0200_0000) != 0 {
            offset |= 0xFC00_0000;
        }

        if link {
            self.register[self.idx[14]] = self.register[register_index::PC].wrapping_sub(4);
        }

        self.register[register_index::PC] = self.register[register_index::PC].wrapping_add(offset);
    }

    fn bx(&mut self, opcode: u32) {
        if !self.pass_condition(opcode) {
            return;
        }

        let rm = (opcode & 0xF) as usize;
        let addr = self.register[self.idx[rm]];
        
        self.set_flag(Flag::T, (addr & 1) == 1);
        self.register[register_index::PC] = addr & !1;
    }

    fn swp(&mut self, opcode: u32, memory: &mut Memory) {
        if !self.pass_condition(opcode) {
            return;
        }

        let byte = ((opcode >> 22) & 1) == 1;
        let rn = ((opcode >> 16) & 0xF) as usize;
        let rd = ((opcode >> 12) & 0xF) as usize;
        let rm = (opcode & 0xF) as usize;

        let address = self.register[self.idx[rn]];
        let rm_val = self.register[self.idx[rm]];

        if byte {
            let temp = memory.read_byte(address);
            memory.write_byte(address, (rm_val & 0xFF) as u8);
            self.register[self.idx[rd]] = temp as u32;
        } else {
            let temp = memory.read_word(address);
            memory.write_word(address, rm_val);
            self.register[self.idx[rd]] = temp;
        }
    }

    fn swi(&mut self, _opcode: u32) {
        // Software interrupt - would trigger exception handling
        // For now, just a placeholder
    }

    fn mrs(&mut self, opcode: u32) {
        if !self.pass_condition(opcode) {
            return;
        }

        let use_spsr = ((opcode >> 22) & 1) == 1;
        let rd = ((opcode >> 12) & 0xF) as usize;

        if use_spsr {
            self.register[self.idx[rd]] = self.register[self.spsr];
        } else {
            self.register[self.idx[rd]] = self.register[register_index::CPSR];
        }
    }

    fn msr(&mut self, opcode: u32) {
        if !self.pass_condition(opcode) {
            return;
        }

        let use_spsr = ((opcode >> 22) & 1) == 1;
        let immediate = ((opcode >> 25) & 1) == 1;

        let value = if immediate {
            let rotate_imm = ((opcode >> 8) & 0xF) * 2;
            let immed_8 = opcode & 0xFF;
            immed_8.rotate_right(rotate_imm)
        } else {
            let rm = (opcode & 0xF) as usize;
            self.register[self.idx[rm]]
        };

        let mask = if ((opcode >> 19) & 1) == 1 { 0xFF000000 } else { 0 }
                 | if ((opcode >> 18) & 1) == 1 { 0x00FF0000 } else { 0 }
                 | if ((opcode >> 17) & 1) == 1 { 0x0000FF00 } else { 0 }
                 | if ((opcode >> 16) & 1) == 1 { 0x000000FF } else { 0 };

        if use_spsr {
            self.register[self.spsr] = (self.register[self.spsr] & !mask) | (value & mask);
        } else {
            self.register[register_index::CPSR] = (self.register[register_index::CPSR] & !mask) | (value & mask);
        }
    }

    // THUMB INSTRUCTIONS
    fn thumb_move_shifted_register(&mut self, opcode: u16) {
        let op = (opcode >> 11) & 0x3;
        let offset = (opcode >> 6) & 0x1F;
        let rs = ((opcode >> 3) & 0x7) as usize;
        let rd = (opcode & 0x7) as usize;

        let rs_val = self.register[self.idx[rs]];
        let result = match op {
            0 => rs_val << offset,
            1 => if offset == 0 { 0 } else { rs_val >> offset },
            2 => if offset == 0 { 
                if (rs_val >> 31) == 1 { 0xFFFF_FFFF } else { 0 }
            } else { 
                ((rs_val as i32) >> offset) as u32 
            },
            _ => rs_val,
        };

        self.register[self.idx[rd]] = result;
        self.set_flag(Flag::N, (result >> 31) == 1);
        self.set_flag(Flag::Z, result == 0);
    }

    fn thumb_add_subtract(&mut self, opcode: u16) {
        let op = (opcode >> 9) & 1;
        let immediate = ((opcode >> 10) & 1) == 1;
        let rn_offset = ((opcode >> 6) & 0x7) as usize;
        let rs = ((opcode >> 3) & 0x7) as usize;
        let rd = (opcode & 0x7) as usize;

        let rs_val = self.register[self.idx[rs]];
        let operand = if immediate { rn_offset as u32 } else { self.register[self.idx[rn_offset]] };

        let (result, carry) = if op == 0 {
            rs_val.overflowing_add(operand)
        } else {
            rs_val.overflowing_sub(operand)
        };

        let overflow = if op == 0 {
            (!((rs_val ^ operand) & 0x8000_0000) & ((rs_val ^ result) & 0x8000_0000)) != 0
        } else {
            ((rs_val ^ operand) & (rs_val ^ result) & 0x8000_0000) != 0
        };

        self.register[self.idx[rd]] = result;
        self.set_flag(Flag::N, (result >> 31) == 1);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, if op == 0 { carry } else { !carry });
        self.set_flag(Flag::V, overflow);
    }

    fn thumb_move_compare_add_sub_imm(&mut self, opcode: u16) {
        let op = (opcode >> 11) & 0x3;
        let rd = ((opcode >> 8) & 0x7) as usize;
        let offset = (opcode & 0xFF) as u32;

        match op {
            0 => { // MOV
                self.register[self.idx[rd]] = offset;
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::Z, offset == 0);
            },
            1 => { // CMP
                let rd_val = self.register[self.idx[rd]];
                let (result, carry) = rd_val.overflowing_sub(offset);
                let overflow = ((rd_val ^ offset) & (rd_val ^ result) & 0x8000_0000) != 0;
                
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !carry);
                self.set_flag(Flag::V, overflow);
            },
            2 => { // ADD
                let rd_val = self.register[self.idx[rd]];
                let (result, carry) = rd_val.overflowing_add(offset);
                let overflow = (!((rd_val ^ offset) & 0x8000_0000) & ((rd_val ^ result) & 0x8000_0000)) != 0;
                
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
                self.set_flag(Flag::V, overflow);
            },
            3 => { // SUB
                let rd_val = self.register[self.idx[rd]];
                let (result, carry) = rd_val.overflowing_sub(offset);
                let overflow = ((rd_val ^ offset) & (rd_val ^ result) & 0x8000_0000) != 0;
                
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !carry);
                self.set_flag(Flag::V, overflow);
            },
            _ => {}
        }
    }

    fn thumb_alu_operations(&mut self, opcode: u16) {
        let op = (opcode >> 6) & 0xF;
        let rs = ((opcode >> 3) & 0x7) as usize;
        let rd = (opcode & 0x7) as usize;

        let rs_val = self.register[self.idx[rs]];
        let rd_val = self.register[self.idx[rd]];

        match op {
            0x0 => { // AND
                let result = rd_val & rs_val;
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0x1 => { // EOR
                let result = rd_val ^ rs_val;
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0x2 => { // LSL
                let shift = rs_val & 0xFF;
                let result = if shift < 32 { rd_val << shift } else { 0 };
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0x3 => { // LSR
                let shift = rs_val & 0xFF;
                let result = if shift < 32 { rd_val >> shift } else { 0 };
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0x4 => { // ASR
                let shift = rs_val & 0xFF;
                let result = if shift < 32 { 
                    ((rd_val as i32) >> shift) as u32 
                } else { 
                    if (rd_val >> 31) == 1 { 0xFFFF_FFFF } else { 0 }
                };
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0x5 => { // ADC
                let c = if self.get_flag(Flag::C) { 1 } else { 0 };
                let (temp, carry1) = rd_val.overflowing_add(rs_val);
                let (result, carry2) = temp.overflowing_add(c);
                
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry1 || carry2);
            },
            0x6 => { // SBC
                let c = if self.get_flag(Flag::C) { 0 } else { 1 };
                let (temp, carry1) = rd_val.overflowing_sub(rs_val);
                let (result, carry2) = temp.overflowing_sub(c);
                
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !(carry1 || carry2));
            },
            0x7 => { // ROR
                let shift = rs_val & 0xFF;
                let result = rd_val.rotate_right(shift);
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0x8 => { // TST
                let result = rd_val & rs_val;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0x9 => { // NEG
                let (result, carry) = 0u32.overflowing_sub(rs_val);
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !carry);
            },
            0xA => { // CMP
                let (result, carry) = rd_val.overflowing_sub(rs_val);
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !carry);
            },
            0xB => { // CMN
                let (result, carry) = rd_val.overflowing_add(rs_val);
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, carry);
            },
            0xC => { // ORR
                let result = rd_val | rs_val;
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0xD => { // MUL
                let result = rd_val.wrapping_mul(rs_val);
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0xE => { // BIC
                let result = rd_val & !rs_val;
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            0xF => { // MVN
                let result = !rs_val;
                self.register[self.idx[rd]] = result;
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
            },
            _ => {}
        }
    }

    fn thumb_hi_register_ops(&mut self, opcode: u16) {
        let op = (opcode >> 8) & 0x3;
        let h1 = ((opcode >> 7) & 1) == 1;
        let h2 = ((opcode >> 6) & 1) == 1;
        let rs = (((opcode >> 3) & 0x7) + if h2 { 8 } else { 0 }) as usize;
        let rd = ((opcode & 0x7) + if h1 { 8 } else { 0 }) as usize;

        match op {
            0 => { // ADD
                self.register[self.idx[rd]] = self.register[self.idx[rd]].wrapping_add(self.register[self.idx[rs]]);
            },
            1 => { // CMP
                let rd_val = self.register[self.idx[rd]];
                let rs_val = self.register[self.idx[rs]];
                let (result, carry) = rd_val.overflowing_sub(rs_val);
                
                self.set_flag(Flag::N, (result >> 31) == 1);
                self.set_flag(Flag::Z, result == 0);
                self.set_flag(Flag::C, !carry);
            },
            2 => { // MOV
                self.register[self.idx[rd]] = self.register[self.idx[rs]];
            },
            3 => { // BX
                let addr = self.register[self.idx[rs]];
                self.set_flag(Flag::T, (addr & 1) == 1);
                self.register[register_index::PC] = addr & !1;
            },
            _ => {}
        }
    }

    fn thumb_pc_relative_load(&mut self, opcode: u16, memory: &Memory) {
        let rd = ((opcode >> 8) & 0x7) as usize;
        let offset = ((opcode & 0xFF) << 2) as u32;
        let pc = self.register[register_index::PC] & !2;
        
        let address = pc.wrapping_add(offset);
        self.register[self.idx[rd]] = memory.read_word(address);
    }

    fn thumb_load_store_reg_offset(&mut self, opcode: u16, memory: &mut Memory) {
        let load = ((opcode >> 11) & 1) == 1;
        let byte = ((opcode >> 10) & 1) == 1;
        let ro = ((opcode >> 6) & 0x7) as usize;
        let rb = ((opcode >> 3) & 0x7) as usize;
        let rd = (opcode & 0x7) as usize;

        let address = self.register[self.idx[rb]].wrapping_add(self.register[self.idx[ro]]);

        if load {
            if byte {
                self.register[self.idx[rd]] = memory.read_byte(address) as u32;
            } else {
                self.register[self.idx[rd]] = memory.read_word(address);
            }
        } else {
            if byte {
                memory.write_byte(address, (self.register[self.idx[rd]] & 0xFF) as u8);
            } else {
                memory.write_word(address, self.register[self.idx[rd]]);
            }
        }
    }

    fn thumb_load_store_sign_extended(&mut self, opcode: u16, memory: &Memory) {
        let h = ((opcode >> 11) & 1) == 1;
        let sign_extend = ((opcode >> 10) & 1) == 1;
        let ro = ((opcode >> 6) & 0x7) as usize;
        let rb = ((opcode >> 3) & 0x7) as usize;
        let rd = (opcode & 0x7) as usize;

        let address = self.register[self.idx[rb]].wrapping_add(self.register[self.idx[ro]]);

        if sign_extend {
            if h {
                let val = memory.read_halfword(address) as i16;
                self.register[self.idx[rd]] = val as i32 as u32;
            } else {
                let val = memory.read_byte(address) as i8;
                self.register[self.idx[rd]] = val as i32 as u32;
            }
        } else {
            self.register[self.idx[rd]] = memory.read_halfword(address) as u32;
        }
    }

    fn thumb_load_store_imm_offset(&mut self, opcode: u16, memory: &mut Memory) {
        let byte = ((opcode >> 12) & 1) == 1;
        let load = ((opcode >> 11) & 1) == 1;
        let offset = ((opcode >> 6) & 0x1F) as u32;
        let rb = ((opcode >> 3) & 0x7) as usize;
        let rd = (opcode & 0x7) as usize;

        let address = self.register[self.idx[rb]].wrapping_add(if byte { offset } else { offset << 2 });

        if load {
            if byte {
                self.register[self.idx[rd]] = memory.read_byte(address) as u32;
            } else {
                self.register[self.idx[rd]] = memory.read_word(address);
            }
        } else {
            if byte {
                memory.write_byte(address, (self.register[self.idx[rd]] & 0xFF) as u8);
            } else {
                memory.write_word(address, self.register[self.idx[rd]]);
            }
        }
    }

    fn thumb_load_store_halfword(&mut self, opcode: u16, memory: &mut Memory) {
        let load = ((opcode >> 11) & 1) == 1;
        let offset = (((opcode >> 6) & 0x1F) << 1) as u32;
        let rb = ((opcode >> 3) & 0x7) as usize;
        let rd = (opcode & 0x7) as usize;

        let address = self.register[self.idx[rb]].wrapping_add(offset);

        if load {
            self.register[self.idx[rd]] = memory.read_halfword(address) as u32;
        } else {
            memory.write_halfword(address, (self.register[self.idx[rd]] & 0xFFFF) as u16);
        }
    }

    fn thumb_sp_relative_load_store(&mut self, opcode: u16, memory: &mut Memory) {
        let load = ((opcode >> 11) & 1) == 1;
        let rd = ((opcode >> 8) & 0x7) as usize;
        let offset = ((opcode & 0xFF) << 2) as u32;

        let address = self.register[self.idx[13]].wrapping_add(offset);

        if load {
            self.register[self.idx[rd]] = memory.read_word(address);
        } else {
            memory.write_word(address, self.register[self.idx[rd]]);
        }
    }

    fn thumb_load_address(&mut self, opcode: u16) {
        let sp = ((opcode >> 11) & 1) == 1;
        let rd = ((opcode >> 8) & 0x7) as usize;
        let offset = ((opcode & 0xFF) << 2) as u32;

        if sp {
            self.register[self.idx[rd]] = self.register[self.idx[13]].wrapping_add(offset);
        } else {
            let pc = self.register[register_index::PC] & !2;
            self.register[self.idx[rd]] = pc.wrapping_add(offset);
        }
    }

    fn thumb_add_offset_to_sp(&mut self, opcode: u16) {
        let sign = ((opcode >> 7) & 1) == 1;
        let offset = ((opcode & 0x7F) << 2) as u32;

        if sign {
            self.register[self.idx[13]] = self.register[self.idx[13]].wrapping_sub(offset);
        } else {
            self.register[self.idx[13]] = self.register[self.idx[13]].wrapping_add(offset);
        }
    }

    fn thumb_push_pop(&mut self, opcode: u16, memory: &mut Memory) {
        let load = ((opcode >> 11) & 1) == 1;
        let r_bit = ((opcode >> 8) & 1) == 1;
        let register_list = (opcode & 0xFF) as u8;

        let mut sp = self.register[self.idx[13]];

        if load {
            for i in 0..8 {
                if (register_list & (1 << i)) != 0 {
                    self.register[self.idx[i]] = memory.read_word(sp);
                    sp = sp.wrapping_add(4);
                }
            }
            if r_bit {
                self.register[register_index::PC] = memory.read_word(sp);
                sp = sp.wrapping_add(4);
            }
        } else {
            if r_bit {
                sp = sp.wrapping_sub(4);
                memory.write_word(sp, self.register[self.idx[14]]);
            }
            for i in (0..8).rev() {
                if (register_list & (1 << i)) != 0 {
                    sp = sp.wrapping_sub(4);
                    memory.write_word(sp, self.register[self.idx[i]]);
                }
            }
        }

        self.register[self.idx[13]] = sp;
    }

    fn thumb_multiple_load_store(&mut self, opcode: u16, memory: &mut Memory) {
        let load = ((opcode >> 11) & 1) == 1;
        let rb = ((opcode >> 8) & 0x7) as usize;
        let register_list = (opcode & 0xFF) as u8;

        let mut address = self.register[self.idx[rb]];

        if load {
            for i in 0..8 {
                if (register_list & (1 << i)) != 0 {
                    self.register[self.idx[i]] = memory.read_word(address);
                    address = address.wrapping_add(4);
                }
            }
        } else {
            for i in 0..8 {
                if (register_list & (1 << i)) != 0 {
                    memory.write_word(address, self.register[self.idx[i]]);
                    address = address.wrapping_add(4);
                }
            }
        }

        self.register[self.idx[rb]] = address;
    }

    fn thumb_conditional_branch(&mut self, opcode: u16) {
        let cond = (opcode >> 8) & 0xF;
        let mut offset = (opcode & 0xFF) as i8 as i32;
        offset = (offset << 1) as i32;

        let should_branch = match cond {
            0x0 => self.get_flag(Flag::Z),
            0x1 => !self.get_flag(Flag::Z),
            0x2 => self.get_flag(Flag::C),
            0x3 => !self.get_flag(Flag::C),
            0x4 => self.get_flag(Flag::N),
            0x5 => !self.get_flag(Flag::N),
            0x6 => self.get_flag(Flag::V),
            0x7 => !self.get_flag(Flag::V),
            0x8 => self.get_flag(Flag::C) && !self.get_flag(Flag::Z),
            0x9 => !self.get_flag(Flag::C) || self.get_flag(Flag::Z),
            0xA => self.get_flag(Flag::N) == self.get_flag(Flag::V),
            0xB => self.get_flag(Flag::N) != self.get_flag(Flag::V),
            0xC => !self.get_flag(Flag::Z) && (self.get_flag(Flag::N) == self.get_flag(Flag::V)),
            0xD => self.get_flag(Flag::Z) || (self.get_flag(Flag::N) != self.get_flag(Flag::V)),
            _ => false,
        };

        if should_branch {
            self.register[register_index::PC] = (self.register[register_index::PC] as i32).wrapping_add(offset) as u32;
        }
    }

    fn thumb_unconditional_branch(&mut self, opcode: u16) {
        let mut offset = (opcode & 0x7FF) as i32;
        if (offset & 0x400) != 0 {
            offset |= !0x7FF;
        }
        offset <<= 1;

        self.register[register_index::PC] = (self.register[register_index::PC] as i32).wrapping_add(offset) as u32;
    }

    fn thumb_long_branch_with_link(&mut self, opcode: u16) {
        let h_bit = ((opcode >> 11) & 1) == 1;
        let offset = (opcode & 0x7FF) as u32;

        if h_bit {
            let temp = self.register[self.idx[14]];
            self.register[self.idx[14]] = (self.register[register_index::PC].wrapping_sub(2)) | 1;
            self.register[register_index::PC] = temp.wrapping_add(offset << 1);
        } else {
            let mut offset_11 = offset;
            if (offset_11 & 0x400) != 0 {
                offset_11 |= 0xFFFF_F800;
            }
            self.register[self.idx[14]] = self.register[register_index::PC].wrapping_add(offset_11 << 12);
        }
    }

    fn swi_thumb(&mut self, _opcode: u16) {
        // Software interrupt
    }

    // HELPER FUNCTIONS
    pub fn set_flag(&mut self, flag: Flag, bit: bool) {
        let mask = flag.get_mask();
        if bit {
            self.register[register_index::CPSR] |= mask;
        } else {
            self.register[register_index::CPSR] &= !mask;
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let mask = flag.get_mask();
        (self.register[register_index::CPSR] & mask) != 0
    }

    pub fn pass_condition(&self, opcode: u32) -> bool {
        let condition = opcode & CONDITION_MASK;
        let n = self.get_flag(Flag::N);
        let z = self.get_flag(Flag::Z);
        let c = self.get_flag(Flag::C);
        let v = self.get_flag(Flag::V);
    
        match condition {
            condition_codes::EQ => z,
            condition_codes::NE => !z,
            condition_codes::CS_HS => c,
            condition_codes::CC_LO => !c,
            condition_codes::MI => n,
            condition_codes::PL => !n,
            condition_codes::VS => v,
            condition_codes::VC => !v,
            condition_codes::HI => c && !z,
            condition_codes::LS => !c || z,
            condition_codes::GE => n == v,
            condition_codes::LT => n != v,
            condition_codes::GT => !z && (n == v),
            condition_codes::LE => z || (n != v),
            condition_codes::AL => true,
            _ => false,
        }
    }

    pub fn dump_registers(&self) {
        println!("=== ARM7TDMI Register Dump ===");
        for i in 0..16 {
            println!("R{:02}: {:#010x}", i, self.register[self.idx[i]]);
        }
        println!("CPSR: {:#010x}", self.register[register_index::CPSR]);
        println!("Flags: N={} Z={} C={} V={} T={}",
            self.get_flag(Flag::N) as u8,
            self.get_flag(Flag::Z) as u8,
            self.get_flag(Flag::C) as u8,
            self.get_flag(Flag::V) as u8,
            self.get_flag(Flag::T) as u8);
    }
}

pub enum Flag {
    N,
    Z,
    C,
    V,
    T
}

impl Flag {
    pub fn get_mask(&self) -> u32 {
        match self {
            Flag::N => flag_masks::N,
            Flag::Z => flag_masks::Z,
            Flag::C => flag_masks::C,
            Flag::V => flag_masks::V,
            Flag::T => flag_masks::T,
        }
    }
}
