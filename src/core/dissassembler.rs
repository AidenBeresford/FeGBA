
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

    (opcode & mask) == format || (opcode & mask) == link // could we not just mask with 0b0000_1110_...? someone check this please
}
