# ARM7TDMI Instruction Quick Reference

## ARM Data Processing Instructions

| Opcode | Mnemonic | Operation | Flags |
|--------|----------|-----------|-------|
| 0x0 | AND | Rd = Rn AND Op2 | N, Z, C |
| 0x1 | EOR | Rd = Rn XOR Op2 | N, Z, C |
| 0x2 | SUB | Rd = Rn - Op2 | N, Z, C, V |
| 0x3 | RSB | Rd = Op2 - Rn | N, Z, C, V |
| 0x4 | ADD | Rd = Rn + Op2 | N, Z, C, V |
| 0x5 | ADC | Rd = Rn + Op2 + C | N, Z, C, V |
| 0x6 | SBC | Rd = Rn - Op2 - !C | N, Z, C, V |
| 0x7 | RSC | Rd = Op2 - Rn - !C | N, Z, C, V |
| 0x8 | TST | Rn AND Op2 | N, Z, C |
| 0x9 | TEQ | Rn XOR Op2 | N, Z, C |
| 0xA | CMP | Rn - Op2 | N, Z, C, V |
| 0xB | CMN | Rn + Op2 | N, Z, C, V |
| 0xC | ORR | Rd = Rn OR Op2 | N, Z, C |
| 0xD | MOV | Rd = Op2 | N, Z, C |
| 0xE | BIC | Rd = Rn AND NOT Op2 | N, Z, C |
| 0xF | MVN | Rd = NOT Op2 | N, Z, C |

## ARM Load/Store Instructions

| Mnemonic | Description |
|----------|-------------|
| LDR | Load word (32-bit) |
| STR | Store word (32-bit) |
| LDRB | Load byte (8-bit, unsigned) |
| STRB | Store byte (8-bit) |
| LDRH | Load halfword (16-bit, unsigned) |
| STRH | Store halfword (16-bit) |
| LDRSB | Load signed byte |
| LDRSH | Load signed halfword |

## ARM Block Transfer

| Mnemonic | Description | Direction | When |
|----------|-------------|-----------|------|
| LDMIA/LDMFD | Load multiple increment after | Up | After |
| STMIA/STMFD | Store multiple increment after | Up | After |
| LDMIB/LDMED | Load multiple increment before | Up | Before |
| STMIB/STMED | Store multiple increment before | Up | Before |
| LDMDA/LDMFA | Load multiple decrement after | Down | After |
| STMDA/STMFA | Store multiple decrement after | Down | After |
| LDMDB/LDMEA | Load multiple decrement before | Down | Before |
| STMDB/STMEA | Store multiple decrement before | Down | Before |

## ARM Branch Instructions

| Mnemonic | Description |
|----------|-------------|
| B | Branch |
| BL | Branch with link (saves return address in LR) |
| BX | Branch and exchange (switch ARM/THUMB) |

## ARM Multiply Instructions

| Mnemonic | Description |
|----------|-------------|
| MUL | Multiply (32-bit result) |
| MLA | Multiply-accumulate |
| UMULL | Unsigned multiply long (64-bit result) |
| UMLAL | Unsigned multiply-accumulate long |
| SMULL | Signed multiply long |
| SMLAL | Signed multiply-accumulate long |

## THUMB Instructions

### Format 1: Move Shifted Register
```
LSL Rd, Rs, #Offset5
LSR Rd, Rs, #Offset5
ASR Rd, Rs, #Offset5
```

### Format 2: Add/Subtract
```
ADD Rd, Rs, Rn
ADD Rd, Rs, #Offset3
SUB Rd, Rs, Rn
SUB Rd, Rs, #Offset3
```

### Format 3: Move/Compare/Add/Subtract Immediate
```
MOV Rd, #Offset8
CMP Rd, #Offset8
ADD Rd, #Offset8
SUB Rd, #Offset8
```

### Format 4: ALU Operations
```
AND Rd, Rs    EOR Rd, Rs    LSL Rd, Rs    LSR Rd, Rs
ASR Rd, Rs    ADC Rd, Rs    SBC Rd, Rs    ROR Rd, Rs
TST Rd, Rs    NEG Rd, Rs    CMP Rd, Rs    CMN Rd, Rs
ORR Rd, Rs    MUL Rd, Rs    BIC Rd, Rs    MVN Rd, Rs
```

### Format 5: Hi Register Operations
```
ADD Rd, Hs
ADD Hd, Rs
ADD Hd, Hs
CMP Rd, Hs
CMP Hd, Rs
CMP Hd, Hs
MOV Rd, Hs
MOV Hd, Rs
MOV Hd, Hs
BX Rs
BX Hs
```

## Condition Codes

| Code | Mnemonic | Meaning | Flags |
|------|----------|---------|-------|
| 0000 | EQ | Equal | Z set |
| 0001 | NE | Not equal | Z clear |
| 0010 | CS/HS | Carry set/unsigned higher or same | C set |
| 0011 | CC/LO | Carry clear/unsigned lower | C clear |
| 0100 | MI | Minus/negative | N set |
| 0101 | PL | Plus/positive or zero | N clear |
| 0110 | VS | Overflow | V set |
| 0111 | VC | No overflow | V clear |
| 1000 | HI | Unsigned higher | C set and Z clear |
| 1001 | LS | Unsigned lower or same | C clear or Z set |
| 1010 | GE | Signed greater or equal | N == V |
| 1011 | LT | Signed less than | N != V |
| 1100 | GT | Signed greater than | Z clear and N == V |
| 1101 | LE | Signed less or equal | Z set or N != V |
| 1110 | AL | Always | (ignored) |

## Registers

### General Purpose (0-12)
- R0-R7: Available in both ARM and THUMB
- R8-R12: High registers (ARM mode, or THUMB with Format 5)

### Special Purpose
- R13 (SP): Stack Pointer
- R14 (LR): Link Register (return address)
- R15 (PC): Program Counter
  - ARM mode: PC = current instruction + 8
  - THUMB mode: PC = current instruction + 4

### Status Registers
- CPSR: Current Program Status Register
- SPSR: Saved Program Status Register (per mode)

## CPU Modes

| Mode | Encoding | Description |
|------|----------|-------------|
| USR | 10000 | User (normal program execution) |
| FIQ | 10001 | Fast Interrupt |
| IRQ | 10010 | Interrupt |
| SVC | 10011 | Supervisor (software interrupt) |
| ABT | 10111 | Abort |
| UND | 11011 | Undefined instruction |
| SYS | 11111 | System (privileged user mode) |

## Flags

| Bit | Flag | Meaning |
|-----|------|---------|
| 31 | N | Negative |
| 30 | Z | Zero |
| 29 | C | Carry |
| 28 | V | Overflow |
| 7 | I | IRQ disable |
| 6 | F | FIQ disable |
| 5 | T | THUMB state (1 = THUMB, 0 = ARM) |
| 4-0 | M[4:0] | Mode bits |

## Addressing Modes

### Mode 1: Data Processing Operands
- Immediate: `#value` with rotation
- Register: `Rm`
- Register with shift: `Rm, LSL #n`, `Rm, LSR Rs`, etc.

### Mode 2: Load/Store Word/Byte
- Immediate offset: `[Rn, #+/-offset]`
- Register offset: `[Rn, +/-Rm]`
- Scaled register: `[Rn, +/-Rm, shift]`
- Pre-indexed: `[Rn, #+/-offset]!`
- Post-indexed: `[Rn], #+/-offset`

### Mode 3: Load/Store Halfword
- Immediate offset: `[Rn, #+/-offset8]`
- Register offset: `[Rn, +/-Rm]`

### Mode 4: Load/Store Multiple
- IA: Increment After
- IB: Increment Before
- DA: Decrement After
- DB: Decrement Before

## Example Programs

### Simple Addition (ARM)
```assembly
MOV R0, #5      ; E3A00005
MOV R1, #3      ; E3A01003
ADD R2, R0, R1  ; E0802001
```

### Factorial Loop (ARM)
```assembly
MOV R0, #5      ; Input
MOV R1, #1      ; Result
loop:
MUL R1, R0, R1  ; R1 = R0 * R1
SUBS R0, R0, #1 ; R0--, set flags
BNE loop        ; Branch if not zero
```

### Conditional Example (ARM)
```assembly
CMP R0, #10     ; Compare R0 with 10
ADDGT R1, R1, #1 ; If R0 > 10, increment R1
ADDLE R2, R2, #1 ; If R0 <= 10, increment R2
```

## Memory Access

All memory accesses should be aligned:
- Word (32-bit): Address must be divisible by 4
- Halfword (16-bit): Address must be divisible by 2
- Byte (8-bit): Any address

The GBA uses little-endian byte ordering.
