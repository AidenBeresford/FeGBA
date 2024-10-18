pub fn is_branch_and_branch_exchange(opcode: u32) -> bool {
    const BRANCH_AND_EXCHANGE_FORMAT: u32 = 0b0000_0001_0010_1111_1111_1111_0001_0000;
    const FORMAT_MASK: u32 = 0b0000_1111_1111_1111_1111_1111_1111_0000;

    let extracted_format = opcode & FORMAT_MASK;

    return extracted_format == BRANCH_AND_EXCHANGE_FORMAT
}
