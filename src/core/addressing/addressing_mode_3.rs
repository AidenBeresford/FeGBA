use crate::core::cpu::ARM7TDMI;

enum ModeEncoding {
    ImmediateOff,
    RegisterOff,
    ImmediatePre,
    RegisterPre,
    ImmediatePost,
    RegisterPost,
    Undefined,
}


pub fn addressing_mode_3(cpu: &mut ARM7TDMI, opcode: u32, operand: &mut u32) {  
    let rn = (opcode & 0x00F0_0000) >> 16;
    let rn_val = cpu.register[cpu.idx[rn as usize]];

    match decode_operand(opcode) {
        ModeEncoding::ImmediateOff => {
            let immed_l: u8 = (opcode & 0x0000_000F) as u8;
            let immed_h: u8 = ((opcode & 0x0000_0F00) >> 4) as u8;
            let u_flag = (opcode >> 23) & 1;
            let offset_8: u8 = immed_h | immed_l;

            if u_flag == 1 {
                *operand = rn_val + offset_8 as u32;
            } else {
                *operand = rn_val - offset_8 as u32;
            }
        },

        ModeEncoding::RegisterOff => {
            let rm = opcode & 0x000F;
            let rm_val = cpu.register[cpu.idx[rm as usize]]; 
            let u_flag = (opcode >> 23) & 1;
            
            if u_flag == 1 {
                *operand = rn_val + rm_val;
            } else {
                *operand = rn_val - rm_val;
            }
        },

        ModeEncoding::ImmediatePre => { 
            let immed_l: u8 = (opcode & 0x0000_000F) as u8;
            let immed_h: u8 = ((opcode & 0x0000_0F00) >> 4) as u8;
            let u_flag = (opcode >> 23) & 1;
            let offset_8: u8 = immed_h | immed_l;

            if u_flag == 1 {
                *operand = rn_val + offset_8 as u32;
            } else {
                *operand = rn_val - offset_8 as u32;
            }

            if cpu.pass_condition(opcode) {
                cpu.register[cpu.idx[rn as usize]] = *operand;
            }
        },

        ModeEncoding::RegisterPre => { 
            let rm = opcode & 0x000F;
            let rm_val = cpu.register[cpu.idx[rm as usize]]; 
            let u_flag = (opcode >> 23) & 1;
            
            if u_flag == 1 {
                *operand = rn_val + rm_val;
            } else {
                *operand = rn_val - rm_val;
            }

            if cpu.pass_condition(opcode) {
                cpu.register[cpu.idx[rn as usize]] = *operand;
            }
        },

        ModeEncoding::ImmediatePost => { 
            let immed_l: u8 = (opcode & 0x0000_000F) as u8;
            let immed_h: u8 = ((opcode & 0x0000_0F00) >> 4) as u8;
            let u_flag = (opcode >> 23) & 1;
            let offset_8: u8 = immed_h | immed_l;
            *operand = rn_val;

            if cpu.pass_condition(opcode) { 
                if u_flag == 1 {
                    cpu.register[cpu.idx[rn as usize]] = rn_val + offset_8 as u32;
                } else {
                    cpu.register[cpu.idx[rn as usize]] = rn_val - offset_8 as u32;
                }
            }
        },

        ModeEncoding::RegisterPost => { 
            let rm = opcode & 0x000F;
            let rm_val = cpu.register[cpu.idx[rm as usize]]; 
            let u_flag = (opcode >> 23) & 1;
             
            *operand = rn_val;
            
            if cpu.pass_condition(opcode) {
                if u_flag == 1 {    
                    cpu.register[cpu.idx[rn as usize]] = rn_val + rm_val;
                } else {
                    cpu.register[cpu.idx[rn as usize]] = rn_val - rm_val;
                }
            }
        },

        ModeEncoding::Undefined => {
            panic!("UNDEFINED ADDRESSING MODE 3 CASE!")
        }
    }
}

fn decode_operand(opcode: u32) -> ModeEncoding {
    if ((opcode >> 22) & 1) == 1 {
        if ((opcode >> 24) & 1) == 1 {
            if ((opcode >> 21) & 1) == 1 {ModeEncoding::ImmediatePre} else {ModeEncoding::ImmediateOff}
        } else {
            if ((opcode >> 21) & 1) == 1 {ModeEncoding::ImmediatePost} else {ModeEncoding::Undefined}
        }
    } else { 
        if ((opcode >> 24) & 1) == 1 {
            if ((opcode >> 21) & 1) == 1 {ModeEncoding::RegisterPre} else {ModeEncoding::RegisterOff}
        } else {
            if ((opcode >> 21) & 1) == 1 {ModeEncoding::RegisterPost} else {ModeEncoding::Undefined}
        }
    }
}

