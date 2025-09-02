enum ModeEncoding {
    Immediate,
    Register,
    ImmediatePreIndexed,
    RegisterPreIndexed,
    ImmediatePostIndexed,
    RegisterPostIndexed,
}

pub fn get_address(&self, opcode: u32) -> u32 {
    let c_flag = get_flag(C);
    let u = (opcode >> 23) & 1;
    let rn = (opcode >> 16) & 0xF;
    let rn_val = self.register[self.idx[rn]] as usize;

    let address: u32;
    match decode_operand(opcode) {
        ModeEncoding::Immediate => {
            let immedH = (opcode >> 8) & 0xF;
            let immedL = opcode & 0xF;

            let offset_8 = (immedH << 4) | immedL;
            if u == 1{
                address = rn_val + offset_8;
            } else {
                address = rn_val - offset_8;
            }
        },
        ModeEncoding::Register => {
            let rm = opcode & 0xF;
            let rm_val = self.register[self.idx[rm]] as usize;
            if u == 1{
                address = rn_val + rm_val;
            } else {
                address = rn_val - rm_val;
            }
        },
        ModeEncoding::ImmediatePreIndexed => {
            let immedH = (opcode >> 8) & 0xF;
            let immedL = opcode & 0xF;

            let offset_8 = (immedH << 4) | immedL;
            if u == 1 {
                address = rn_val + offset_8;
            } else {
                address = rn_val - offset_8;
            }
            if pass_condition(opcode) {
                self.register[self.idx[rn]] = address;
            }
        },
        ModeEncoding::RegisterPreIndexed => {
            let rm = opcode & 0xF;
            let rm_val = self.register[self.idx[rm]] as usize;
            if u == 1 {
                address = rn_val + rm_val;
            } else {
                address = rn_val - rm_val;
            }
            if self.pass_condition(opcode) {
                self.register[self.idx[rn]] = address;
            }
        },
        ModeEncoding::ImmediatePostIndexed => {
            let immedH = (opcode >> 8) & 0xF;
            let immedL = opcode & 0xF;

            address = rn_val;
            let offset_8 = (immedH << 4) | immedL;
            if self.pass_condition(opcode) {
                if u == 1 {
                    self.register[self.idx[rn]] += offset_8;
                } else {
                    self.register[self.idx[rn]] -= offset_8;
                }
            }
        },
        ModeEncoding::RegisterPostIndexed => {
            rm = opcode & 0xF;
            rm_val = self.register[self.idx[rm]] as usize;

            address = rn_val;
            if self.pass_condition(opcode) {
                if u == 1 {
                    self.register[self.idx[rn]] += rm_val;
                } else {
                    self.register[self.idx[rn]] -= rm_val;
                }
            }
        },
    }
    address
}



fn decode_operand(opcode: u32) -> ModeEncoding {
    let p = (opcode >> 24) & 1;
    let i = (opcode >> 22) & 1;
    let w = (opcode >> 21) & 1;

    if i == 0 {
        if p == 0 {
            ModeEncoding::RegisterPostIndexed;
        } else {
            if w == 0 {
                ModeEncoding::Register;
            } else {
                ModeEncoding::RegisterPreIndexed;
            }
        }
    } else {
        if p == 0 {
            ModeEncoding::ImmediatePostIndexed;
        } else {
            if w == 0 {
                ModeEncoding::Immediate;
            } else {
                ModeEncoding::ImmediatePreIndexed;
            }
        }
    }
}
