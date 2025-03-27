use crate::core::cpu::ARM7TDMI;

enum ShifterEncoding {
    Immediate,
    Register,
    LSLImmediate,
    LSLRegister,
    LSRImmediate,
    LSRRegister,
    ASRImmediate,
    ASRRegister,
    RORImmediate,
    RORRegister,
    RRXImmediate,
    Undefined,
}


pub fn addressing_mode_1(cpu: &ARM7TDMI, opcode: u32, operand: &mut u32) -> bool {
    let rm = opcode & 0xF;
    let rm_val = cpu.register[cpu.idx[rm as usize]];
    let c_flag = cpu.get_flag(C);
    match decode_operand(opcode) {
        ShifterEncoding::Immediate => {
            let rotate_imm: u8 = (opcode >> 8) & 0xF;
            let immed_8: u8 = opcode & 0xFF;
            let rotate_amt: u8 = 2*rotate_imm;

            operand = (immed_8 << (32-rotate_amt)) | (immed_8 >> rotate_amt);
            
            if rotate_amt == 0 {
                c_flag
            } else {
                (operand>>31) == 1
            }
        },
        
        ShifterEncoding::Register => {
            operand = rm_val;
            c_flag
        },
        // no shift, carry from CPSR

        ShifterEncoding::LSLImmediate => {
            let shift_imm: u8 = (opcode >> 7) & 0x1F;
            
            if shift_imm == 0 {
                operand = rm_val;
                c_flag
            } else {
                operand = rm_val << shift_imm;
                ((rm_val >> (32-shift_imm)) & 1) == 1 
            }
        },

        ShifterEncoding::LSLRegister => {
            let rs = (opcode >> 8) & 0xF;
            let rs_val = cpu.register[cpu.idx[rs as usize]] & 0xFF;
            
            if rs_val == 0 {
                operand = rm_val;
                c_flag
            } else if rs_val < 32 {
                operand = rm_val << rs_val;
                ((rm_val >> (32-rs_val)) & 1) == 1
            } else if rs_val == 32 {
                operand = 0;
                (rs_val & 1) == 1
            } else {
                operand = 0;
                false
            }
            
        },

        ShifterEncoding::LSRImmediate => {
            let shift_imm = (opcode >> 7) & 0x1F;
            
            if shift_imm == 0 {
                operand = 0;
                (rm_val >> 31) == 1
            } else {
                operand = rm_val >> shift_imm;
                ((rm >> (shift_imm-1)) & 1) == 1
            }
        },

        ShifterEncoding::LSRRegister => {
            let rs = (opcode >> 8) & 0xF;
            let rs_val = cpu.register[cpu.idx[rs as usize]] & 0xFF;
            
            if rs_val == 0 {
                operand = rm_val;
                c_flag
            } else if rs_val < 32 {
                operand = rm_val >> rs_val;
                ((rm_val >> (rs_val-1)) & 1) == 1
            } else if rs_val == 32 {
                operand = 0;
                (rm_val >> 31) == 1
            } else {
                operand = 0;
                false
            }
        },

        ShifterEncoding::ASRImmediate => {
            let shift_imm = (opcode >> 7) & 0x1F;
            
            if shift_imm == 0 {
                operand = 0xFFFF_FFFF * (rm_val >> 31);
                (rm_val >> 31) == 1
            } else {
                operand = (rm_val as i32) >> shift_imm;
                ((rm_val >> (shift_imm - 1)) & 1) == 1
            }
        },

        ShifterEncoding::ASRRegister => {
            let rs = (opcode >> 8) & 0xF;
            let rs_val = cpu.register[cpu.idx[rs as usize]] & 0xFF;
            
            if rs_val == 0 {
                operand = rm_val;
                c_flag
            } else if rs_val < 32 {
                operand = (rm_val as i32) >> rs_val;
                ((rm_val >> (rs_val - 1)) & 1) == 1
            } else {
                operand = 0xFFFF_FFFF * (rm_val >> 31);
                (rm_val >> 31) == 1
            }
        },

        ShifterEncoding::RORImmediate => {
            let shift_imm = (opcode >> 7) & 0x1F;

            if shift_imm == 0 {
                operand = ((c_flag as u32) << 31) | (rm_val >> 1);
                (rm_val & 1) == 1
            } else {
                operand = (rm_val << (32-shift_imm)) | (rm_val >> shift_imm);
                ((rm_val >> (shift_imm-1)) & 1) == 1
            }
        },

        ShifterEncoding::RORRegister => { 
            let rs = (opcode >> 8) & 0xF;
            let rs_val = cpu.register[cpu.idx[rs as usize]] & 0xFF;
            let rs_small = rs_val & 0x1F

            if rs_val == 0 {
                operand = rm_val;
                c_flag
            } else if rs_small == 0 {
                operand = rm_val;
                (rm_val >> 31) == 1
            } else {
                operand = (rm_val << (32-rs_small)) | (rm_val >> rs_small);
                ((rm_val >> (rs_small-1)) & 1) == 1
            }
        },

        ShifterEncoding::RRXImmediate => {
            operand = ((c_flag as u32) << 31) | (rm_val >> 1);
            (rm_val & 1) == 1
        },

        _ => {
            panic!("UNDEFINED ADDRESSING MODE 1 CASE!");
            false
        }, // Undefined case
    }
}



fn decode_operand(opcode: u32) -> ShifterEncoding {
    if immediate(opcode) {
        return ShifterEncoding::Immediate;
    } else if register(opcode) {
        return ShifterEncoding::Register;
    } else if lsl_immediate(opcode) {
        return ShifterEncoding::LSLImmediate;
    } else if lsl_register(opcode) {
        return ShifterEncoding::LSLRegister;
    } else if lsr_immediate(opcode) {
        return ShifterEncoding::LSRImmediate;
    } else if lsr_register(opcode) {
        return ShifterEncoding::LSRRegister; 
    } else if asr_immediate(opcode) {
        return ShifterEncoding::ASRImmediate;
    } else if asr_register(opcode) {
        return ShifterEncoding::ASRRegister;
    } else if rrx_immediate(opcode) { 
        return ShifterEncoding::RRXImmediate;
    } else if ror_register(opcode) {
        return ShifterEncoding::RORRegister;
    } else if ror_immediate(opcode) {
        return ShifterEncoding::RORImmediate;
    } else {
        return ShifterEncoding::Undefined;
    }
}

fn immediate(opcode: u32) -> bool {
    (opcode & (1<<25)) != 0
}

fn register(opcode: u32) -> bool {
    (opcode & 0x0000_0FF0) == 0
}

fn lsl_immediate(opcode: u32) -> bool {
    (opcode & 0b0111_0000) == 0
}

fn lsl_register(opcode: u32) -> bool {
    (opcode & 0b1111_0000) == 0b0001_0000
}

fn lsr_immediate(opcode: u32) -> bool {
    (opcode & 0b0111_0000) == 0b0010_0000
}

fn lsr_register(opcode: u32) -> bool {
    (opcode & 0b1111_0000) == 0b0011_0000
}

fn asr_immediate(opcode: u32) -> bool {
    (opcode & 0b0111_0000) == 0b0100_0000
}

fn asr_register(opcode: u32) -> bool {
    (opcode & 0b1111_0000) == 0b0101_0000
}

fn ror_immediate(opcode: u32) -> bool {
    (opcode & 0b0111_0000) == 0b0110_0000
}

fn ror_register(opcode: u32) -> bool {
    (opcode & 0b1111_0000) == 0b0111_0000
}

fn rrx_immediate(opcode: u32) -> bool {
    (opcode & 0x0000_0FF0) == 0b0110_0000
}
