enum ShifterEncoding {
    Immediate,
    Register,
    ScaledRegister,
    ImmediatePreIndexed,
    RegisterPreIndexed,
    ScaledRegisterPreIndexed,
    ImmediatePostIndexed,
    RegisterPostIndexed,
    ScaledRegisterPostIndexed,
}

pub fn shifter_carry_out(&self, opcode: u32) -> u32 {
    let c_flag = get_flag(C);
    let u = (opcode >> 23) & 1;
    let rn = (opcode >> 16) & 0xF;
    let rn_val = self.register[self.idx[rn]] as usize;

    let address: u32;
    match decode_operand(opcode) {
        ShifterEncoding::Immediate => {
            let offset_12 = opcode & 0xFFF;
            if u == 1{
                address = rn_val + offset_12;
            } else {
                address = rn_val - offset_12;
            }
        },
        ShifterEncoding::Register => {
            let rm = opcode & 0xF;
            let rm_val = self.register[self.idx[rm]] as usize;
            if u == 1{
                address = rn_val + rm_val;
            } else {
                address = rn_val - rm_val;
            }
        },
        ShifterEncoding::ScaledRegister => {
            let shift_imm = (opcode >> 7) & 0x1F;
            let shift = (opcode >> 5) & 0b11;
            let rm = opcode & 0xF;
            let rm_val = self.register[self.idx[rm]] as usize;

            let index;
            match shift {
                0 => {
                    index = rm_val << shift_imm;
                },
                1 => {
                    if shift_imm == 0 {
                        index = 0
                    } else {
                        index = rm_val >> shift_imm;
                    }
                },
                2 => {
                    if shift_imm == 0 {
                        if rm_val >> 31 == 1 {
                            index = 0xFFFFFFFF;
                        } else {
                            index = 0;
                        }
                    } else {
                        // rm_val must be signed to get asr
                        index = (rm_val as i32) >> shift_imm;
                    }
                },
                3 => {
                    if shift_imm == 0 {
                        index = c_flag << 31 || rm_val >> 1;
                    } else {
                        index = rm_val.rotate_right(shift_imm);
                    }
                },
            }
            if u == 1 {
                address = rn_val + index;
            } else {
                address = rn_val - index;
            }
        },
        ShifterEncoding::ImmediatePreIndexed => {
            let offset_12 = opcode & 0xFFF;
            if u == 1 {
                address = rn_val + offset_12;
            } else {
                address = rn_val - offset_12;
            }
            if pass_condition(opcode) {
                self.register[self.idx[rn]] = address;
            }
        },
        ShifterEncoding::RegisterPreIndexed => {
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
        ShifterEncoding::ScaledRegisterPreIndexed => {
            let shift_imm = (opcode >> 7) & 0x1F;
            let shift = (opcode >> 5) & 0b11;
            let rm = opcode & 0xF;
            let rm_val = self.register[self.idx[rm]] as usize;

            let index;
            match shift {
                0 => {
                    index = rm_val << shift_imm;
                },
                1 => {
                    if shift_imm == 0 {
                        index = 0
                    } else {
                        index = rm_val >> shift_imm;
                    }
                },
                2 => {
                    if shift_imm == 0 {
                        if rm_val >> 31 == 1 {
                            index = 0xFFFFFFFF;
                        } else {
                            index = 0;
                        }
                    } else {
                        // rm_val must be signed to get asr
                        index = (rm_val as i32) >> shift_imm;
                    }
                },
                3 => {
                    if shift_imm == 0 {
                        index = c_flag << 31 || rm_val >> 1;
                    } else {
                        index = rm_val.rotate_right(shift_imm);
                    }
                },
            }
            if u == 1 {
                address = rn_val + index;
            } else {
                address = rn_val - index;
            }
            if self.pass_condition(opcode) {
                self.register[self.idx[rn]] = address;
            }
        },
        ShifterEncoding::ImmediatePostIndexed => {
            offset_12 = opcode & 0xFFF;

            address = rn_val;
            if self.pass_condition(opcode) {
                if u == 1 {
                    self.register[self.idx[rn]] += offset_12;
                } else {
                    self.register[self.idx[rn]] -= offset_12;
                }
            }
        },
        ShifterEncoding::RegisterPostIndexed => {
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
        ShifterEncoding::ScaledRegisterPostIndexed => {
            let shift_imm = (opcode >> 7) & 0x1F;
            let shift = (opcode >> 5) & 0b11;
            let rm = opcode & 0xF;
            let rm_val = self.register[self.idx[rm]] as usize;

            address = rn_val;
            let index;
            match shift {
                0 => {
                    index = rm_val << shift_imm;
                },
                1 => {
                    if shift_imm == 0 {
                        index = 0
                    } else {
                        index = rm_val >> shift_imm;
                    }
                },
                2 => {
                    if shift_imm == 0 {
                        if rm_val >> 31 == 1 {
                            index = 0xFFFFFFFF;
                        } else {
                            index = 0;
                        }
                    } else {
                        // rm_val must be signed to get asr
                        index = (rm_val as i32) >> shift_imm;
                    }
                },
                3 => {
                    if shift_imm == 0 {
                        index = c_flag << 31 || rm_val >> 1;
                    } else {
                        index = rm_val.rotate_right(shift_imm);
                    }
                },
            }
            if self.pass_condition(opcode) {
                if u == 1 {
                    self.register[self.idx[rn]] += index;
                } else {
                    self.register[self.idx[rn]] -= index;
                }
            }

        },
    }
    address
}



fn decode_operand(opcode: u32) -> ShifterEncoding {
    let i = (opcode >> 25) & 1;
    let p = (opcode >> 24) & 1;
    let w = (opcode >> 21) & 1;

    // Immediate
    if i == 0 {
        if p == 0 {
            ShifterEncoding::ImmediatePostIndexed;
        } else {
            if w == 0 {
                ShifterEncoding::Immediate;
            } else {
                ShifterEncoding::ImmediatePreIndexed;
            }
        }
    } else {
        // unscaled register
        if (opcode & 0x0FF0) == 0 {
            if p == 0 {
                ShifterEncoding::RegisterPostIndexed;
            } else {
                if w == 0 {
                    ShifterEncoding::Register;
                } else {
                    ShifterEncoding::RegisterPreIndexed;
                }
            }
        } else {
            if p == 0 {
                ShifterEncoding::ScaledRegisterPostIndexed;
            } else {
                if w == 0 {
                    ShifterEncoding::ScaledRegister;
                } else {
                    ShifterEncoding::ScaledRegisterPreIndexed;
                }
            }
        }
    }
}
