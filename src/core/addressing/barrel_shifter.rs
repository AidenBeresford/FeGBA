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


pub fn shifter_carry_out(&self, opcode: u32) -> u32 {
    let rm = opcode & 0xF;
    let rm_val = self.register[rm] as usize;
    let c_flag = get_flag(C);
    match decode_operand(opcode) {
        ShifterEncoding::Immediate => {
            let rotate_imm = (opcode >> 8) & 0xF;
            if rotate_imm == 0 {
                c_flag
            } else {
                (rm_val >> ((rotate_imm * 2) - 1)) & 1
            }
        },
        ShifterEncoding::Register => c_flag, // no shift, carry from CPSR

        ShifterEncoding::LSLImmediate => {
            let shift_amount = (opcode >> 7) & 0x1F;
            if shift_amount == 0 {
                c_flag
            } else {
                (rm_val >> (32 - shift_amount)) & 1
            }
        },
        ShifterEncoding::LSLRegister => {
            let shift_amount = (opcode >> 8) & 0xF;
            if shift_amount == 0 {
                c_flag
            } else {
                (rm_val >> (32 - shift_amount)) & 1
            }
        },

        ShifterEncoding::LSRImmediate => {
            let shift_amount = (opcode >> 7) & 0x1F;
            if shift_amount == 0 {
                (rm_val >> 31) & 1
            } else {
                (rm_val >> (shift_amount - 1)) & 1
            }
        },
        ShifterEncoding::LSRRegister => {
            let shift_amount = (opcode >> 8) & 0xF;
            if shift_amount == 0 {
                c_flag
            } else if shift_amount < 32 {
                (rm_val >> (shift_amount - 1)) & 1
            } else {
                0
            }
        },

        ShifterEncoding::ASRImmediate => {
            let shift_amount = (opcode >> 7) & 0x1F;
            if shift_amount == 0 {
                (rm_val >> 31) & 1
            } else {
                (rm_val >> (shift_amount - 1)) & 1
            }
        },
        ShifterEncoding::ASRRegister => {
            let shift_amount = (opcode >> 8) & 0xF;
            if shift_amount == 0 {
                c_flag
            } else if shift_amount < 32 {
                (rm_val >> (shift_amount - 1)) & 1
            } else {
                (rm_val >> 31) & 1
            }
        },

        ShifterEncoding::RORImmediate => {
            let rotate_amount = (opcode >> 7) & 0x1F;
            if rotate_amount == 0 {
                c_flag
            } else {
                (rm_val >> (rotate_amount - 1)) & 1
            }
        },
        ShifterEncoding::RORRegister => {
            let rotate_amount = (opcode >> 8) & 0xF;
            if rotate_amount == 0 {
                c_flag
            } else {
                let rotate = rotate_amount % 32;
                (rm_val >> (rotate - 1)) & 1
            }
        },

        ShifterEncoding::RRXImmediate => rm_val & 1,

        _ => 0, // Undefined case
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
