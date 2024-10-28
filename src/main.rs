mod core;
mod constants;

use crate::core::bus::Memory;
use crate::core::bus::BusAccess;
use crate::core::cpu::ARM7TDMI;

/* TEST 1 - BASIC MEMORY OPERATIONS
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
*/

/* TEST 2 - CPU ACCESS */
fn main() {
    let mut memory = Memory::new();
    let mut cpu: ARM7TDMI = Default::default();
    let memstart:u32 = 0x0300_0000;

    for i in 0..0xFFFF {
        memory.wbyte((memstart+i) as usize, (i%8) as u8)
    }

    println!("{:#04x}", memory.rbyte(0x0300_0001));
    
    cpu.wreg(3, 9);
 
    cpu.lbyte(&mut memory, 0x0300_0000);

    println!("{:#04x}", cpu.r08[0]);
    println!("{:#04x}", cpu.rreg(3));
    
    cpu.sbyte(&mut memory, 0x0300_0001);
    println!("{:#04x}", memory.rbyte(0x0300_0001));
}
