use crate::core::cpu::ARM7TDMI;

enum ModeEncoding {
    IncrementAfter,
    IncrementBefore,
    DecrementAfter,
    DecrementBefore,
}


pub fn addressing_mode_4(cpu: &mut ARM7TDMI, opcode: u32, start_addr: &mut u32, end_addr: &mut u32) {
    let register_list: u16 = (opcode & 0xFFFF) as u16;
    let rn = opcode & 0xF_0000;
    let rn_val: u32 = cpu.register[cpu.idx[rn as usize]];
    let w_flag = (opcode >> 21) & 1;
    match decode_operand(opcode) {
        ModeEncoding::IncrementAfter => {
             *start_addr = rn_val;
             *end_addr = rn_val + (register_list.count_ones() * 4) - 4;
             
             if cpu.pass_condition(opcode) && w_flag == 1 {
                cpu.register[cpu.idx[rn as usize]] = rn_val + (register_list.count_ones() * 4);
             }
        },
        
        ModeEncoding::IncrementBefore => {
            *start_addr = rn_val + 4;
            *end_addr = rn_val + (register_list.count_ones() * 4);

            if cpu.pass_condition(opcode) && w_flag == 1 {
                cpu.register[cpu.idx[rn as usize]] = rn_val + (register_list.count_ones() * 4);
            }
        },
        
        ModeEncoding::DecrementAfter => {
            *start_addr = rn_val - (register_list.count_ones() * 4) + 4;
            *end_addr = rn_val;

            if cpu.pass_condition(opcode) && w_flag == 1 {
                cpu.register[cpu.idx[rn as usize]] = rn_val + (register_list.count_ones() * 4);
            }
        },
        
        ModeEncoding::DecrementBefore => {
            *start_addr = rn_val - (register_list.count_ones() * 4);
            *end_addr = rn_val - 4;

            if cpu.pass_condition(opcode) && w_flag == 1 {
                cpu.register[cpu.idx[rn as usize]] = rn_val + (register_list.count_ones() * 4);
            }
        },
    }
}


fn decode_operand(opcode: u32) -> ModeEncoding {
    if (opcode >> 23) & 1 == 1 {
        if (opcode >> 24) & 1 == 1 {ModeEncoding::IncrementBefore} else {ModeEncoding::IncrementAfter}
    } else {
        if (opcode >> 24) & 1 == 1 {ModeEncoding::DecrementBefore} else {ModeEncoding::DecrementAfter}
    }
}


