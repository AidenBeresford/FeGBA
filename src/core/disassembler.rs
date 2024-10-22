pub enum InstructionSet {
    ARM(u32),
    THUMB(u32)
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
    MoveShiftedRegister
}

pub fn disassemble(inset: InstructionSet) -> Instruction {
    match inset {
        InstructionSet::ARM(op) => {return disassemble_arm(op);},
        InstructionSet::THUMB(op) => {return disassemble_thumb(op);},
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

// block data transfer here

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
