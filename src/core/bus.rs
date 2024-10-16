pub struct Memory {
    ram: Box<[u8]>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {ram: vec![0u8; 0xFFFF_FFFF].into_boxed_slice(),}
    }

    pub fn rbyte(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn wbyte(&mut self, addr: usize, data: u8) {
        self.ram[addr] = data;
    }
}

