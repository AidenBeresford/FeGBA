use crate::core::cpu::ARM7TDMI;

enum ModeEncoding {
    ImmediateOffset,
    RegisterOffset,
    ScaledOffset,
    ImmediatePre,
    RegisterPre,
    ScaledPre,
    ImmediatePost,
    RegisterPost,
    ScaledPost,
    Undefined
}


pub fn addressing_mode_2(cpu: &mut ARM7TDMI, opcode: u32, operand: &mut u32) { 
    let rn = (opcode & 0x00F0_0000) >> 16;
    let rn_val = cpu.register[cpu.idx[rn as usize]];
    match decode_operand(opcode) {
        ModeEncoding::ImmediateOffset => {
            let u_flag = opcode & (1<<23);
            if u_flag != 0 {
                *operand = rn_val + (opcode & 0x0FFF);
            } else {
                *operand = rn_val - (opcode & 0x0FFF);
            }
        },

        ModeEncoding::RegisterOffset => {
            let rm = opcode & 0xF;
            let rm_val = cpu.register[cpu.idx[rm as usize]];
            let u_flag = opcode & (1<<23);
            if u_flag != 0 {
                *operand = rn_val + rm_val;
            } else {
                *operand = rn_val - rm_val;
            }
        },

        ModeEncoding::ScaledOffset => {
            // fucking mouthful of bitshift shenanigans I don't want to write for now
        },

        ModeEncoding::ImmediatePre => { 
            let u_flag = opcode & (1<<23);
            if u_flag != 0 {
                *operand = rn_val + (opcode & 0x0FFF);
            } else {
                *operand = rn_val - (opcode & 0x0FFF);
            }
            
            if cpu.pass_condition(opcode) {
                cpu.register[cpu.idx[rn as usize]] = *operand;
            }
        },

        ModeEncoding::RegisterPre => { 
            let rm = opcode & 0xF;
            let rm_val = cpu.register[cpu.idx[rm as usize]]; 
            let u_flag = opcode & (1<<23);
            if u_flag != 0 {
                *operand = rn_val + rm_val;
            } else {
                *operand = rn_val - rm_val;
            }
            
            if cpu.pass_condition(opcode) {
                cpu.register[cpu.idx[rn as usize]] = *operand;
            }
        },

        ModeEncoding::ScaledPre => {
            // delaying this for as long as humanly possible
        },

        ModeEncoding::ImmediatePost => {
            *operand = rn_val;
            if cpu.pass_condition(opcode) {
                let u_flag = opcode & (1<<23);
                if u_flag != 0 {
                    cpu.register[cpu.idx[rn as usize]] = rn_val + (opcode & 0x0FFF);
                } else {
                    cpu.register[cpu.idx[rn as usize]] = rn_val - (opcode & 0x0FFF);
                }
            }
        },

        ModeEncoding::RegisterPost => {
            *operand = rn_val;
            if cpu.pass_condition(opcode) {
                let rm = opcode & 0xF;
                let rm_val = cpu.register[cpu.idx[rm as usize]]; 
                let u_flag = opcode & (1<<23);
                
                if u_flag != 0 {
                    cpu.register[cpu.idx[rn as usize]] = rn_val + rm_val;
                } else {
                    cpu.register[cpu.idx[rn as usize]] = rn_val - rm_val;
                }
            }
        },

        ModeEncoding::ScaledPost => {
            // "I'm in hell." -Obito Uchiha
        },
        
        ModeEncoding::Undefined => {
            panic!("UNDEFINED ADDRESSING MODE 2 CASE!")
        },
    }
}

fn decode_operand(opcode: u32) -> ModeEncoding {
    if immediate(opcode) {
        if pre_indexed(opcode) {
            return ModeEncoding::ImmediatePre;
        }
        else if post_indexed(opcode) {
            return ModeEncoding::ImmediatePost;
        }
        else if offset(opcode) {
            return ModeEncoding::ImmediateOffset;
        }
    }
    else if register(opcode) {
        if pre_indexed(opcode) {
            return ModeEncoding::RegisterPre;
        }
        else if post_indexed(opcode) {
            return ModeEncoding::RegisterPost;
        }
        else if offset(opcode) {
            return ModeEncoding::RegisterOffset;
        }
    }
    else if scaled(opcode) {
        if pre_indexed(opcode) {
            return ModeEncoding::ScaledPre;
        }
        else if post_indexed(opcode) {
            return ModeEncoding::ScaledPost;
        }
        else if offset(opcode) {
            return ModeEncoding::ScaledOffset
        }
    }

    return ModeEncoding::Undefined
}

fn immediate(opcode: u32) -> bool {
    (opcode & (1<<25)) != 0
}

fn register(opcode: u32) -> bool {
    (opcode & 0x0000_0FF0) == 0
}

fn scaled(opcode: u32) -> bool {
    (opcode & (1<<4)) == 0
}

fn pre_indexed(opcode: u32) -> bool {
    (opcode & (1<<21)) == 0
}

fn post_indexed(opcode: u32) -> bool {
    (opcode & (1<<24)) == 0
}

fn offset(opcode: u32) -> bool {
    (opcode & (1<<24)) != 0
}
