use crate::constants::memory::*;

pub struct Memory {
    bios: Vec<u8>,
    ewram: Vec<u8>,
    iwram: Vec<u8>,
    io: Vec<u8>,
    palette: Vec<u8>,
    vram: Vec<u8>,
    oam: Vec<u8>,
    rom: Vec<u8>,
    sram: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bios: vec![0; BIOS_SIZE],
            ewram: vec![0; EWRAM_SIZE],
            iwram: vec![0; IWRAM_SIZE],
            io: vec![0; IO_SIZE],
            palette: vec![0; PALETTE_SIZE],
            vram: vec![0; VRAM_SIZE],
            oam: vec![0; OAM_SIZE],
            rom: vec![0; ROM_SIZE],
            sram: vec![0; SRAM_SIZE],
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        let len = rom_data.len().min(ROM_SIZE);
        self.rom[..len].copy_from_slice(&rom_data[..len]);
    }

    pub fn load_bios(&mut self, bios_data: &[u8]) {
        let len = bios_data.len().min(BIOS_SIZE);
        self.bios[..len].copy_from_slice(&bios_data[..len]);
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            BIOS_START..=BIOS_END => {
                self.bios[(addr - BIOS_START) as usize]
            }
            EWRAM_START..=EWRAM_END => {
                self.ewram[(addr - EWRAM_START) as usize]
            }
            IWRAM_START..=IWRAM_END => {
                self.iwram[(addr - IWRAM_START) as usize]
            }
            IO_START..=IO_END => {
                self.io[(addr - IO_START) as usize]
            }
            PALETTE_START..=PALETTE_END => {
                self.palette[(addr - PALETTE_START) as usize]
            }
            VRAM_START..=VRAM_END => {
                self.vram[(addr - VRAM_START) as usize]
            }
            OAM_START..=OAM_END => {
                self.oam[(addr - OAM_START) as usize]
            }
            ROM_START..=ROM_END => {
                self.rom[(addr - ROM_START) as usize]
            }
            SRAM_START..=SRAM_END => {
                self.sram[(addr - SRAM_START) as usize]
            }
            _ => {
                // Unmapped memory
                0
            }
        }
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) {
        match addr {
            BIOS_START..=BIOS_END => {
                // BIOS is read-only
            }
            EWRAM_START..=EWRAM_END => {
                self.ewram[(addr - EWRAM_START) as usize] = value;
            }
            IWRAM_START..=IWRAM_END => {
                self.iwram[(addr - IWRAM_START) as usize] = value;
            }
            IO_START..=IO_END => {
                self.io[(addr - IO_START) as usize] = value;
            }
            PALETTE_START..=PALETTE_END => {
                self.palette[(addr - PALETTE_START) as usize] = value;
            }
            VRAM_START..=VRAM_END => {
                self.vram[(addr - VRAM_START) as usize] = value;
            }
            OAM_START..=OAM_END => {
                self.oam[(addr - OAM_START) as usize] = value;
            }
            ROM_START..=ROM_END => {
                self.rom[(addr - ROM_START) as usize] = value;
}
            SRAM_START..=SRAM_END => {
                self.sram[(addr - SRAM_START) as usize] = value;
            }
            _ => {
                // Unmapped memory
            }
        }
    }

    pub fn read_halfword(&self, addr: u32) -> u16 {
        let aligned_addr = addr & !1;
        let low = self.read_byte(aligned_addr) as u16;
        let high = self.read_byte(aligned_addr + 1) as u16;
        (high << 8) | low
    }

    pub fn write_halfword(&mut self, addr: u32, value: u16) {
        let aligned_addr = addr & !1;
        self.write_byte(aligned_addr, (value & 0xFF) as u8);
        self.write_byte(aligned_addr + 1, ((value >> 8) & 0xFF) as u8);
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        let aligned_addr = addr & !3;
        let b0 = self.read_byte(aligned_addr) as u32;
        let b1 = self.read_byte(aligned_addr + 1) as u32;
        let b2 = self.read_byte(aligned_addr + 2) as u32;
        let b3 = self.read_byte(aligned_addr + 3) as u32;
        (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        let aligned_addr = addr & !3;
        self.write_byte(aligned_addr, (value & 0xFF) as u8);
        self.write_byte(aligned_addr + 1, ((value >> 8) & 0xFF) as u8);
        self.write_byte(aligned_addr + 2, ((value >> 16) & 0xFF) as u8);
        self.write_byte(aligned_addr + 3, ((value >> 24) & 0xFF) as u8);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
