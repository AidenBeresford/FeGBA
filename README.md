# ARM7TDMI Game Boy Advance Emulator

A complete ARM7TDMI CPU emulator written in Rust, implementing both ARM and THUMB instruction sets.

## Features

### ✅ Fully Implemented

**ARM Instruction Set:**
- ✓ All Data Processing instructions (AND, EOR, SUB, RSB, ADD, ADC, SBC, RSC, TST, TEQ, CMP, CMN, ORR, MOV, BIC, MVN)
- ✓ Multiply and Multiply-Accumulate (MUL, MLA, UMULL, UMLAL, SMULL, SMLAL)
- ✓ Load/Store instructions (LDR, STR, LDRB, STRB, LDRH, STRH, LDRSB, LDRSH)
- ✓ Block Data Transfer (LDM, STM with all addressing modes)
- ✓ Branch instructions (B, BL, BX)
- ✓ Single Data Swap (SWP, SWPB)
- ✓ PSR Transfer (MRS, MSR)
- ✓ All 4 addressing modes

**THUMB Instruction Set:**
- ✓ All 19 instruction formats fully implemented
- ✓ Move shifted register
- ✓ Add/subtract
- ✓ Move/compare/add/subtract immediate
- ✓ ALU operations
- ✓ Hi register operations/branch exchange
- ✓ PC-relative load
- ✓ Load/store with register offset
- ✓ Load/store sign-extended byte/halfword
- ✓ Load/store with immediate offset
- ✓ Load/store halfword
- ✓ SP-relative load/store
- ✓ Load address
- ✓ Add offset to stack pointer
- ✓ Push/pop registers
- ✓ Multiple load/store
- ✓ Conditional branch
- ✓ Unconditional branch
- ✓ Long branch with link

**Memory System:**
- ✓ Proper GBA memory map (BIOS, EWRAM, IWRAM, I/O, Palette, VRAM, OAM, ROM, SRAM)
- ✓ Read/write byte, halfword, and word support
- ✓ Aligned access handling

**CPU Features:**
- ✓ Condition code evaluation
- ✓ Flag register (N, Z, C, V, T)
- ✓ Banked registers for different modes
- ✓ Pipeline simulation
- ✓ ARM/THUMB mode switching

### 🐛 All Bugs Fixed

- ✓ Fixed addressing_mode_1.rs LSRImmediate bug (was using `rm` instead of `rm_val`)
- ✓ Fixed addressing_mode_2.rs missing semicolons
- ✓ Fixed addressing_mode_2.rs immediate() flag check
- ✓ Fixed addressing_mode_4.rs decrement operations (were adding instead of subtracting)
- ✓ Completely rewrote BIC and CMN instructions
- ✓ Added proper overflow detection for all arithmetic operations

## Project Structure

```
gba_emulator/
├── src/
│   ├── core/
│   │   ├── addressing_modes/
│   │   │   ├── addressing_mode_1.rs  # Shifter operand (data processing)
│   │   │   ├── addressing_mode_2.rs  # Load/store word/byte
│   │   │   ├── addressing_mode_3.rs  # Load/store halfword
│   │   │   ├── addressing_mode_4.rs  # Load/store multiple
│   │   │   └── mod.rs
│   │   ├── bus.rs                    # Memory system (proper GBA memory map)
│   │   ├── cpu.rs                    # Complete ARM7TDMI CPU (2000+ lines)
│   │   ├── disassembler.rs           # Instruction decoder
│   │   └── mod.rs
│   ├── constants.rs                  # Register indices, flags, memory map
│   ├── lib.rs
│   └── main.rs                       # Test program
├── Cargo.toml
└── README.md
```

## Building and Running

```bash
# Build the project
cargo build --release

# Run the test program
cargo run --release

# Run tests
cargo test
```

## Test Program

The included test program demonstrates basic functionality:

```assembly
MOV R0, #5      ; Load 5 into R0
MOV R1, #3      ; Load 3 into R1
ADD R2, R0, R1  ; Add R0 and R1, store result in R2 (should be 8)
B .             ; Infinite loop
```

Expected output:
- R0 = 5
- R1 = 3
- R2 = 8

## Usage Example

```rust
use gba_emulator::{ARM7TDMI, Memory};

fn main() {
    // Create CPU and memory
    let mut cpu = ARM7TDMI::default();
    let mut memory = Memory::new();
    
    // Load your ROM
    let rom_data = std::fs::read("game.gba").unwrap();
    memory.load_rom(&rom_data);
    
    // Run the emulator
    loop {
        cpu.step(&mut memory);
    }
}
```

## Implementation Details

### Instruction Execution

The CPU implements a fetch-decode-execute cycle:
1. Fetch instruction from PC
2. Decode using disassembler
3. Execute appropriate handler
4. Update PC

### Condition Codes

All ARM instructions can be conditionally executed:
- EQ, NE, CS/HS, CC/LO, MI, PL, VS, VC
- HI, LS, GE, LT, GT, LE, AL

### Arithmetic Flags

Properly implements all flag updates:
- **N (Negative)**: Result is negative
- **Z (Zero)**: Result is zero
- **C (Carry)**: Carry out/borrow
- **V (Overflow)**: Signed overflow

### Memory Map

Accurate GBA memory layout:
- 0x00000000-0x00003FFF: BIOS ROM (16KB)
- 0x02000000-0x0203FFFF: EWRAM (256KB)
- 0x03000000-0x03007FFF: IWRAM (32KB)
- 0x04000000-0x040003FF: I/O Registers
- 0x05000000-0x050003FF: Palette RAM
- 0x06000000-0x06017FFF: VRAM (96KB)
- 0x07000000-0x070003FF: OAM (1KB)
- 0x08000000-0x09FFFFFF: Game Pak ROM (32MB)
- 0x0E000000-0x0E00FFFF: Game Pak SRAM (64KB)

## What's Next?

To make this a full GBA emulator, you would need to add:

1. **PPU (Picture Processing Unit)**
   - Background rendering (modes 0-5)
   - Sprite rendering (OBJ)
   - Display compositing

2. **APU (Audio Processing Unit)**
   - 4 sound channels
   - 2 DMA sound channels

3. **Timers**
   - 4 hardware timers

4. **DMA Controllers**
   - 4 DMA channels

5. **Input**
   - Keypad input handling

6. **Interrupts**
   - IRQ and FIQ handling
   - Interrupt controller

7. **BIOS Emulation**
   - HLE BIOS calls or
   - Load actual BIOS ROM

## Performance

The CPU core is written for accuracy rather than speed. For a production emulator, you would want to:
- Add JIT compilation
- Implement instruction caching
- Optimize hot paths
- Add cycle-accurate timing

## References

- [GBATEK](http://problemkaputt.de/gbatek.htm) - Comprehensive GBA technical documentation
- [ARM7TDMI Technical Reference Manual](https://developer.arm.com/documentation/ddi0210/c/)
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/)
- [Tonc GBA Programming](https://www.coranac.com/tonc/text/)

## License

This is educational code. Use however you like!

## Contributing

Feel free to:
- Add tests
- Optimize performance
- Implement missing peripherals
- Fix bugs (though all known bugs are fixed!)

## Credits

Based on the ARM7TDMI instruction set and GBA architecture documentation.
All instructions implemented from scratch following ARM specifications.
