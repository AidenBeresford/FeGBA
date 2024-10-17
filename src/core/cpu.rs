use crate::core::bus::Memory;
use crate::core::bus::BusAccess;

pub struct ARM7TDMI {
    rlow: [u32; 8], // first 8 registers; accessible by THUMB
    pub r08: [u32; 2], // these arrays are for banking registers
    r09: [u32; 2],
    r10: [u32; 2],
    r11: [u32; 2],
    r12: [u32; 2],
    sp: [u32; 6],
    lr: [u32; 6],
    pc: u32,
    cpsr: u32,
    spsr: [u32; 5], // if anyone is willing to help cull these down please be my guest
}

impl Default for ARM7TDMI {
    fn default() -> ARM7TDMI {
        ARM7TDMI {
            rlow: [0; 8],
            r08: [0; 2],
            r09: [0; 2],
            r10: [0; 2],
            r11: [0; 2],
            r12: [0; 2],
            sp: [0x0300_7F00, 0x0300_7FA0, 0, 0, 0, 0x0300_7FE0],
            lr: [0; 6],
            pc: 0x0800_0000,
            cpsr: 0b0000_0000_0000_0000_0000_0000_0001_0011,
            spsr: [0; 5],
        }
    }
}

impl BusAccess for ARM7TDMI {
    fn lbyte(&mut self, memory: &Memory, addr: usize) {
        self.r08[0] = memory.rbyte(addr) as u32;
    }

    fn sbyte(&self, memory: &mut Memory, addr: usize) {
        memory.wbyte(addr, self.r08[0] as u8)
    }

}

impl ARM7TDMI {
    pub fn wreg(&mut self, reg: usize, data: u32) {
        self.rlow[reg] = data;
    }
    
    pub fn rreg(&self, reg: usize) -> u32 {
        self.rlow[reg]
    }
}
