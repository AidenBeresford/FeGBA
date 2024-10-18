fn is_branch_and_branch_exchange(opcode : i32) -> bool
{
    let branch_and_exchange_form: i32 = 0b0000_0001_0010_1111_1111_1111_0001_0000; 
    let format_mask: i32  = 0b0000_1111_1111_1111_1111_1111_1111_0000; 
    let extracted_format: i32 = format_mask & branch_and_exchange_form;
    return branch_and_exchange_form == extracted_format;
}