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

// TODO: implement this function that matches ShifterEncoding and returns the carryout
// pub fn shifter_carry_out(opcode: u32) -> u32 {}

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
