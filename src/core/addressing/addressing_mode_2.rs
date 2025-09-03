use crate::core::cpu::ARM7TDMI;
use crate::core::cpu::Flag;

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
            let rm = opcode & 0xF;
            let rm_val = cpu.register[cpu.idx[rm as usize]];
            let u_flag = opcode & (1<<23);
            let shift = (opcode & 0b0110_0000) >> 5;
            let shift_imm = (opcode >> 6) & 0b0001_1111;
            let index: u32;

            if shift == 0b00 {
                index = rm_val << shift_imm;
            }
            else if shift == 0b01 {
                if shift_imm == 0 {
                    index = 0;
                } else {
                    index = rm_val >> shift_imm;
                }
            }
            else if shift == 0b10 {
                if shift_imm == 0 {
                    if (rm_val >> 31) == 1 {
                        index = 0xFFFF_FFFF;
                    } else {
                        index = 0;
                    }
                } else {
                    index = ((rm_val as i32) >> shift_imm) as u32;
                }
            }
            else if shift == 0b11 {
                if shift_imm == 0 {
                    let c_flag = if cpu.get_flag(Flag::C) {1} else {0};
                    index = (c_flag << 31) | (rm_val >> 1); 
                } else {
                    index = rm_val.rotate_right(shift_imm);
                }
            }
            else {
                panic!("OH FUCK! CHECK ADDMODE2 SCALED OFFSET");
            } 
            
            if u_flag != 0 {
                *operand = rn_val + index;
            } else {
                *operand = rn_val - index;
            }
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
            let rm = opcode & 0xF;
            let rm_val = cpu.register[cpu.idx[rm as usize]];
            let u_flag = opcode & (1<<23);
            let shift = (opcode & 0b0110_0000) >> 5;
            let shift_imm = (opcode >> 6) & 0b0001_1111;
            let index: u32;

            if shift == 0b00 {
                index = rm_val << shift_imm;
            }
            else if shift == 0b01 {
                if shift_imm == 0 {
                    index = 0;
                } else {
                    index = rm_val >> shift_imm;
                }
            }
            else if shift == 0b10 {
                if shift_imm == 0 {
                    if rm_val >> 31 == 1 {
                        index = 0xFFFF_FFFF;
                    } else {
                        index = 0;
                    }
                } else {
                    index = ((rm_val as i32) >> shift_imm) as u32;
                }
            }
            else if shift == 0b11 {       
                if shift_imm == 0 {
                    let c_flag = if cpu.get_flag(Flag::C) {1} else {0};
                    index = (c_flag << 31) | (rm_val >> 1); 
                } else {
                    index = rm_val.rotate_right(shift_imm);
                }
            }
            else {
                panic!("OH SHIT! CHECK ADDMODE2 SCALED PRE");
            }

            if u_flag != 0 {
                *operand = rn_val + index;
            } else {
                *operand = rn_val - index;
            }

            if cpu.pass_condition(opcode) {
                cpu.register[cpu.idx[rn as usize]] = *operand;
            }
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
            let rm = opcode & 0xF;
            let rm_val = cpu.register[cpu.idx[rm as usize]];
            let u_flag = opcode & (1<<23);
            let shift = (opcode & 0b0110_0000) >> 5;
            let shift_imm = (opcode >> 6) & 0b0001_1111;
            let index: u32;

            *operand = rn_val;
            if shift == 0b00 {
                index = rm_val << shift_imm;
            }
            else if shift == 0b01 {
                if shift_imm == 0 {
                    index = 0;
                } else {
                    index = rm_val >> shift_imm;
                }
            }
            else if shift == 0b10 {
                if shift_imm == 0 {
                    if rm_val >> 31 == 1 {
                        index = 0xFFFF_FFFF;
                    } else {
                        index = 0;
                    }
                } else {
                    index = ((rm_val as i32) >> shift_imm) as u32;
                }
            }
            else if shift == 0b11 {       
                if shift_imm == 0 {
                    let c_flag = if cpu.get_flag(Flag::C) {1} else {0};
                    index = (c_flag << 31) | (rm_val >> 1); 
                } else {
                    index = rm_val.rotate_right(shift_imm);
                }
            }
            else {
                panic!("OH SHIT! CHECK ADDMODE2 SCALED POST");
            }

            if cpu.pass_condition(opcode) {
                if u_flag != 0 {
                    cpu.register[cpu.idx[rn as usize]] = rn_val + index;
                } else {
                    cpu.register[cpu.idx[rn as usize]] = rn_val - index;
                }
            }
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
