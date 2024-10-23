pub enum InstructionSet {
    ARM(u32),
    THUMB(u32),
}

pub enum Instruction {
    BranchAndBranchExchange,
    BlockDataTransfer,
    BranchAndBranchWithLink,
    SoftwareInterruptA,
    Undefined,
    SingleDataTransfer,
    SingleDataSwap,
    MultiplyAndMultiplyLong,
    HalfwordDataTransfer,
    PSRTransfer,
    DataProcessing, // END OF ARM
    SoftwareInterruptT,
    UncoditionalBranch,
    ConditionalBranch,
    MultipleLoadStore,
    LongBranchWithLink,
    AddOffsetToSP,
    PushPopRegister,
    LoadStoreHalfword,
    SPRelativeLoadStore,
    LoadAddress,
    LoadStoreImmediateOffset,
    LoadStoreRegisterOffset,
    LoadStoreSignExtended,
    PCRelativeLoad,
    HiRegisterOperation,
    ALUOperation,
    MoveCompareAddSubImmediate,
    AddSubtract,
    MoveShiftedRegister,
}

pub fn disassemble(inset: InstructionSet) -> Instruction {
    match inset {
        InstructionSet::ARM(op) => {
            return disassemble_arm(op);
        }
        InstructionSet::THUMB(op) => {
            return disassemble_thumb(op);
        }
    }
}

// TODO
fn disassemble_arm(opcode: u32) -> Instruction {
    Instruction::SoftwareInterruptA // placeholder
}

// TODO
fn disassemble_thumb(opcode: u32) -> Instruction {
    Instruction::SoftwareInterruptT // placeholder
}

fn arm_branch_and_branch_exchange(opcode: u32) -> bool {
    let format: u32 = 0b0000_0001_0010_1111_1111_1111_0001_0000;
    let mask: u32 = 0b0000_1111_1111_1111_1111_1111_1111_0000;

    (opcode & mask) == format
}

fn arm_block_data_transfer(opcode: u32) -> bool {
    let format: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000;
    let mask: u32 = 0b0000_1110_0000_0000_0000_0000_0000_0000;

    (opcode & mask) == format
}

fn arm_branch_and_branch_with_link(opcode: u32) -> bool {
    let format: u32 = 0b0000_1010_0000_0000_0000_0000_0000_0000;
    let link: u32 = 0b0000_1011_0000_0000_0000_0000_0000_0000;
    let mask: u32 = 0b0000_1111_0000_0000_0000_0000_0000_0000;

    (opcode & mask) == format || (opcode & mask) == link
}

fn arm_software_interrupt(opcode: u32) -> bool {
    let format: u32 = 0b0000_1111_0000_0000_0000_0000_0000_0000;
    let mask: u32 = 0b0000_1111_0000_0000_0000_0000_0000_0000;

    (opcode & mask) == format
}

fn arm_undefined(opcode: u32) -> bool {
    let format: u32 = 0b0000_0110_0000_0000_0000_0000_0001_0000;
    let mask: u32 = 0b0000_1110_0000_0000_0000_0000_0001_0000;

    (opcode & mask) == format
}

fn arm_single_data_transfer(opcode: u32) -> bool {
    let format: u32 = 0b0000_0100_0000_0000_0000_0000_0000_0000;
    let mask: u32 = 0b0000_1100_0000_0000_0000_0000_0000_0000;
    (opcode & mask) == format
}

fn arm_single_data_swap(opcode: u32) -> bool {
    let format: u32 = 0b0000_0001_0000_0000_0000_0000_1001_0000;
    let mask: u32 = 0b0000_1111_1000_0000_0000_1111_1111_0000;
    (opcode & mask) == format
}

fn arm_multiply_and_multiply_long(opcode: u32) -> bool {
    let multiply_format: u32 = 0b0000_0000_0000_0000_0000_0000_1001_0000;
    let multiply_long_format: u32 = 0b0000_0000_1000_0000_0000_0000_1001_0000;
    let mask: u32 = 0b0000_1111_1000_0000_0000_0000_1111_0000;
    (opcode & mask) == multiply_format || (opcode & mask) == multiply_long_format
}

fn arm_halfword_data_transfer_register(opcode: u32) -> bool {
    let format: u32 = 0b0000_0000_0000_0000_0000_0000_1001_0000;
    let mask: u32 = 0b0000_1110_0100_0000_0000_1111_1001_0000;
    (opcode & mask) == format
}

fn arm_halfword_data_transfer_immediate(opcode: u32) -> bool {
    let format: u32 = 0b0000_0000_0100_0000_0000_0000_1001_0000;
    let mask: u32 = 0b0000_1110_0100_0000_0000_0000_1001_0000;
    (opcode & mask) == format
}

fn arm_psr_transfer_mrs(opcode: u32) -> bool {
    let format: u32 = 0b0000_0001_0000_1111_0000_0000_0000_0000;
    let mask: u32 = 0b0000_1111_1011_1111_0000_0000_0000_0000;
    (opcode & mask) == format
}

fn arm_psr_transfer_msr(opcode: u32) -> bool {
    let format: u32 = 0b0000_0001_0010_0000_1111_0000_0000_0000;
    let mask: u32 = 0b0000_1101_1011_0000_1111_0000_0000_0000;
    (opcode & mask) == format
}

fn arm_data_processing(opcode: u32) -> bool {
    let format: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
    let mask: u32 = 0b0000_1100_0000_0000_0000_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_software_interrupt(opcode: u16) -> bool {
    let format: u16 = 0b1101_1111_0000_0000;
    let mask: u16 = 0b1111_1111_0000_0000;
    (opcode & mask) == format
}

fn thumb_unconditional_branch(opcode: u16) -> bool {
    let format: u16 = 0b1110_0000_0000_0000;
    let mask: u16 = 0b1111_1000_0000_0000;
    (opcode & mask) == format
}

fn thumb_conditional_branch(opcode: u16) -> bool {
    let format: u16 = 0b1101_0000_0000_0000;
    let mask: u16 = 0b1111_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_multiple_load_store(opcode: u16) -> bool {
    let format: u16 = 0b1100_0000_0000_0000;
    let mask: u16 = 0b1111_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_long_branch_with_link(opcode: u16) -> bool {
    let format: u16 = 0b1111_0000_0000_0000;
    let mask: u16 = 0b1111_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_add_offset_to_stack_pointer(opcode: u16) -> bool {
    let format: u16 = 0b1011_0000_0000_0000;
    let mask: u16 = 0b1111_1111_0000_0000;
    (opcode & mask) == format
}

fn thumb_push_pop_registers(opcode: u16) -> bool {
    let format: u16 = 0b1011_0100_0000_0000;
    let mask: u16 = 0b1111_0110_0000_0000;
    (opcode & mask) == format
}

fn thumb_load_store_halfword(opcode: u16) -> bool {
    let format: u16 = 0b1000_0000_0000_0000;
    let mask: u16 = 0b1111_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_sp_relative_load_store(opcode: u16) -> bool {
    let format: u16 = 0b1001_0000_0000_0000;
    let mask: u16 = 0b1111_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_load_address(opcode: u16) -> bool {
    let format: u16 = 0b1010_0000_0000_0000;
    let mask: u16 = 0b1111_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_load_store_immediate_offset(opcode: u16) -> bool {
    let format: u16 = 0b0110_0000_0000_0000;
    let mask: u16 = 0b1110_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_load_store_register_offset(opcode: u16) -> bool {
    let format: u16 = 0b0101_0000_0000_0000;
    let mask: u16 = 0b1111_0010_0000_0000;
    (opcode & mask) == format
}

fn thumb_load_store_sign_extended(opcode: u16) -> bool {
    let format: u16 = 0b0101_0010_0000_0000;
    let mask: u16 = 0b1111_0010_0000_0000;
    (opcode & mask) == format
}

fn thumb_pc_relative_load(opcode: u16) -> bool {
    let format: u16 = 0b0100_1000_0000_0000;
    let mask: u16 = 0b1111_1000_0000_0000;
    (opcode & mask) == format
}

fn thumb_hi_register_operation(opcode: u16) -> bool {
    let format: u16 = 0b0100_0100_0000_0000;
    let mask: u16 = 0b1111_1100_0000_0000;
    (opcode & mask) == format
}

fn thumb_alu_operations(opcode: u16) -> bool {
    let format: u16 = 0b0100_0000_0000_0000;
    let mask: u16 = 0b1111_1100_0000_0000;
    (opcode & mask) == format
}

fn thumb_move_compare_add_sub_immediate(opcode: u16) -> bool {
    let format: u16 = 0b0010_0000_0000_0000;
    let mask: u16 = 0b1110_0000_0000_0000;
    (opcode & mask) == format
}

fn thumb_add_subtract(opcode: u16) -> bool {
    let format: u16 = 0b0001_1000_0000_0000;
    let mask: u16 = 0b1111_1000_0000_0000;
    (opcode & mask) == format
}

fn thumb_move_shifted_register(opcode: u16) -> bool {
    let format: u16 = 0b0000_0000_0000_0000;
    let mask: u16 = 0b1110_0000_0000_0000;
    (opcode & mask) == format
}
