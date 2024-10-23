pub fn is_branch_and_branch_exchange(opcode: u32) -> bool {
    const BRANCH_AND_EXCHANGE_FORMAT: u32 = 0b0000_0001_0010_1111_1111_1111_0001_0000;
    const FORMAT_MASK: u32 = 0b0000_1111_1111_1111_1111_1111_1111_0000;

    let extracted_format = opcode & FORMAT_MASK;

    extracted_format == BRANCH_AND_EXCHANGE_FORMAT
}

fn arm_block_data_transfer(opcode: u32) -> bool {
    let format: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000;
    let mask: u32 = 0b0000_1110_0000_0000_0000_0000_0000_0000;

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
