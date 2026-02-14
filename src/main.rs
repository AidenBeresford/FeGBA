mod core;
mod constants;

use crate::core::bus::Memory;
use crate::core::cpu::ARM7TDMI;

fn main() {
    println!("ARM7TDMI Emulator Starting...\n");

    let mut cpu: ARM7TDMI = Default::default();
    let mut memory = Memory::new();

    let test_program: Vec<u32> = vec![
        0xE3A00005,  // MOV R0, #5
        0xE3A01003,  // MOV R1, #3
        0xE0802001,  // ADD R2, R0, R1
        0xEAFFFFFE,  // B . (infinite loop)
    ];

    println!("Test program loaded at 0x08000000");
    println!("Program: MOV R0, #5; MOV R1, #3; ADD R2, R0, R1; B .\n");

    // Load program
    for (i, &instruction) in test_program.iter().enumerate() {
        let addr = 0x0800_0000 + (i * 4) as u32;
        memory.write_word(addr, instruction);
    }

    // VERIFY: Read back what we wrote
    println!("=== MEMORY VERIFICATION ===");
    for i in 0..4 {
        let addr = 0x0800_0000 + (i * 4) as u32;
        let read_back = memory.read_word(addr);
        let expected = test_program[i];
        println!("Addr {:#010x}: wrote {:#010x}, read {:#010x} - {}", 
            addr, expected, read_back,
            if read_back == expected { "✓ OK" } else { "✗ FAILED!" });
    }
    println!();

    println!("Initial state:");
    cpu.dump_registers();

    println!("\nExecuting instructions...\n");
    for step in 0..4 {
        let pc = cpu.register[15];
        let opcode = memory.read_word(pc);
        println!("Step {}: PC={:#010x}, Opcode={:#010x}", step, pc, opcode);
        
        // Manual decode check
        let cond = (opcode >> 28) & 0xF;
        println!("  Condition: {:#x} (E=AL should pass)", cond);
        
        cpu.step(&mut memory);
        
        if step < 3 {
            println!("  After execution:");
            println!("    R0={}, R1={}, R2={}", 
                cpu.register[cpu.idx[0]],
                cpu.register[cpu.idx[1]], 
                cpu.register[cpu.idx[2]]);
            println!();
        }
    }

    println!("\nFinal state:");
    cpu.dump_registers();
    
    println!("\nExpected: R0=5, R1=3, R2=8");
    println!("Actual:   R0={}, R1={}, R2={}", 
        cpu.register[cpu.idx[0]],
        cpu.register[cpu.idx[1]],
        cpu.register[cpu.idx[2]]);
    
    if cpu.register[cpu.idx[0]] == 5 && 
       cpu.register[cpu.idx[1]] == 3 && 
       cpu.register[cpu.idx[2]] == 8 {
        println!("\n✓ Test PASSED!");
    } else {
        println!("\n✗ Test FAILED!");
        println!("\nDEBUG: Memory was written correctly but instructions didn't execute!");
        println!("This means the CPU step() function is not calling execute functions properly.");
    }
}