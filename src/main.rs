mod core;

use crate::core::bus::Memory;

fn main() {
    let mut memory = Memory::new();
    let address:u32 = 0x00FF_ABCD;

    for i in 0..100 {
        memory.wbyte((address+i) as usize, i as u8);
    }

    for i in 1..100 {
        let data = memory.rbyte((address+(100-i)) as usize);
        println!("{:#04x}", data);
    }
}
