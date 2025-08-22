mod mem;

use crate::gb::mem::Memory;

use std::{fs, io::Write};

// Offsets for shifting to the corresponding bits
const Z_FLAG: u8 = 7;
const N_FLAG: u8 = 6;
const H_FLAG: u8 = 5;
const C_FLAG: u8 = 4;

pub struct Gameboy {
    pc: u16,
    sp: u16,
    a_reg: u8,
    b_reg: u8,
    c_reg: u8,
    d_reg: u8,
    e_reg: u8,
    f_reg: u8,
    h_reg: u8,
    l_reg: u8,
    mem: Memory,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            pc: 0x100,
            sp: 0xFFFE,
            a_reg: 1,
            b_reg: 0,
            c_reg: 0x13,
            d_reg: 0,
            e_reg: 0xD8,
            f_reg: 0b1011_0000,
            h_reg: 1,
            l_reg: 0x4D,
            mem: Memory::new(),
        }
    }

    fn get_af(&self) -> u16 {
        (self.a_reg as u16) << 8 | self.f_reg as u16
    }
    fn set_af(&mut self, value: u16) {
        self.a_reg = ((value & 0xFF00) >> 8) as u8;
        self.f_reg = (value & 0xFF) as u8;
    }

    fn get_bc(&self) -> u16 {
        (self.b_reg as u16) << 8 | self.c_reg as u16
    }
    fn set_bc(&mut self, value: u16) {
        self.b_reg = ((value & 0xFF00) >> 8) as u8;
        self.c_reg = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d_reg as u16) << 8 | self.e_reg as u16
    }
    fn set_de(&mut self, value: u16) {
        self.d_reg = ((value & 0xFF00) >> 8) as u8;
        self.e_reg = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h_reg as u16) << 8 | self.l_reg as u16
    }
    fn set_hl(&mut self, value: u16) {
        self.h_reg = ((value & 0xFF00) >> 8) as u8;
        self.l_reg = (value & 0xFF) as u8;
    }

    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.f_reg |= 1 << flag;
        } else {
            self.f_reg &= !(1 << flag);
        }
    }
    fn read_flag(&mut self, flag: u8) -> u8 {
        if (self.f_reg & (1 << flag)) == 0 {
            0
        } else {
            1
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.mem.load_rom(data);
    }

    fn tick_system(&mut self) {
        // TODO
    }

    fn m_tick(&mut self) {
        for _x in 0..4 {
            self.tick_system();
        }
    }

    pub fn tick(&mut self) {
        // let mut file = fs::OpenOptions::new()
        //     .create(true) // Create the file if it doesn't exist
        //     .write(true) // Enable writing
        //     .append(true) // Enable appending
        //     .open("log.txt").unwrap();

        // let mem0 = self.mem.read(self.pc);
        // let mem1 = self.mem.read(self.pc + 1);
        // let mem2 = self.mem.read(self.pc + 2);
        // let mem3 = self.mem.read(self.pc + 3);
        // let output = format!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})\n",
        //     self.a_reg, self.f_reg, self.b_reg, self.c_reg, self.d_reg, self.e_reg, self.h_reg, self.l_reg, self.sp, self.pc, mem0, mem1, mem2, mem3);
        // let _ = file.write_all(output.as_bytes());


        let op: u16 = self.fetch();
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        let higher = self.read_next() as u16;
        if higher == 0xCB {
            let lower = self.read_next() as u16;
            (higher << 8) | lower
        } else {
            higher
        }
    }

    fn execute(&mut self, op: u16) {
        // Determine if op is prefixed
        let prefixed: bool = (op >> 8) == 0xCB;
        // Parse out segments of op for categorization
        // Middle 3 must be separated because they can be grouped in different variations
        let block = (op & 0b1100_0000) >> 6;
        let mid_1 = (op & 0b0010_0000) >> 5;
        let mid_2 = (op & 0b0001_0000) >> 4;
        let mid_3 = (op & 0b0000_1000) >> 3;
        let middle = (mid_1, mid_2, mid_3);
        let bottom = op & 0b0000_0111;

        if prefixed {
            let initial = self.get_r8(bottom as u8);
            let result: u8;
            match block {
            // Block 0 (00) (Shifts/Rotates)
                0b00 => {
                    match middle {
                        // RLC
                        (0, 0, 0) => {
                            result = self.rotate_left(initial, true);
                        }

                        // RRC
                        (0, 0, 1) => {
                            result = self.rotate_right(initial, true);
                        }

                        // RL
                        (0, 1, 0) => {
                            result = self.rotate_left(initial, false);
                        }

                        // RR
                        (0, 1, 1) => {
                            result = self.rotate_right(initial, false);
                        }
                        
                        // SLA
                        (1, 0, 0) => {
                            self.set_flag(C_FLAG, (initial >> 7) == 1);
                            result = initial << 1;
                            self.set_flag(Z_FLAG, result == 0);
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                        }
                        
                        // SRA
                        (1, 0, 1) => {
                            self.set_flag(C_FLAG, (initial & 1) == 1);
                            result = (initial >> 1) | (initial & 0b1000_0000);
                            self.set_flag(Z_FLAG, result == 0);
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                        }
                        
                        // SWAP
                        (1, 1, 0) => {
                            result = (initial << 4) | (initial >> 4);
                            self.set_flag(Z_FLAG, result == 0);
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                            self.set_flag(C_FLAG, false);
                        }
                        
                        // SRL
                        (1, 1, 1) => {
                            self.set_flag(C_FLAG, (initial & 1) == 1);
                            result = initial >> 1;
                            self.set_flag(Z_FLAG, result == 0);
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                        }
                        
                        (_, _, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                    self.set_r8(bottom as u8, result);
                }

            // Block 1 (01) (BIT bit, r8)
                0b01 => {
                    let bit = ((middle.0 << 2) | (middle.1 << 1) | middle.2) as u8;
                    self.set_flag(Z_FLAG, (initial >> bit) & 1 == 0);
                    self.set_flag(N_FLAG, false);
                    self.set_flag(H_FLAG, true);
                }

            // Block 2 (10) (RES bit, r8)
                0b10 => {
                    let bit = ((middle.0 << 2) | (middle.1 << 1) | middle.2) as u8;
                    result = initial & !(1 << bit);
                    self.set_r8(bottom as u8, result);
                }

            // Block 3 (11) (SET bit, r8)
                0b11 => {
                    let bit = ((middle.0 << 2) | (middle.1 << 1) | middle.2) as u8;
                    result = initial | (1 << bit);
                    self.set_r8(bottom as u8, result);
                }

            // Nonexistent opcodes
                _ => unimplemented!("Unimplemented opcode: {}", op),
            }
        } else {
            match (block, middle, bottom) {
            // Block 0 (00)
                // NOP
                (0b00, (0, 0, 0), 0b000) => {},
                
                // STOP (Should not be called on DMG, UPDATE IF UPGRADING TO CGB)
                (0b00, (0, 1, 0), 0b000) => unimplemented!("STOP should not be called"),

                // LD (u16), SP
                (0b00, (0, 0, 1), 0b000) => {
                    let sp_low = self.sp as u8;
                    let sp_high = (self.sp >> 8) as u8;
                    let addr_low = self.read_next() as u16;
                    let addr_high = self.read_next() as u16;
                    let addr = (addr_high << 8) | addr_low;
                    self.ld_mem(addr, sp_low);
                    self.ld_mem(addr.wrapping_add(1), sp_high);
                }

                // JR s8 (Unconditional)
                (0b00, (0, 1, 1), 0b000) => {
                    let steps = self.read_next() as i8;
                    self.m_tick();
                    self.pc = self.pc.wrapping_add_signed(steps as i16);
                }

                // JR cond s8 (Conditional)
                (0b00, (1, _, _), 0b000) => {
                    let steps = self.read_next() as i8;
                    let cond: bool;
                    match (mid_2, mid_3) {
                        // Not Z
                        (0, 0) => {
                            cond = self.read_flag(Z_FLAG) == 0;
                        }
                        // Z
                        (0, 1) => {
                            cond = self.read_flag(Z_FLAG) == 1;
                        }
                        // Not C
                        (1, 0) => {
                            cond = self.read_flag(C_FLAG) == 0;
                        }
                        // C
                        (1, 1) => {
                            cond = self.read_flag(C_FLAG) == 1;
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                    if cond {
                        self.m_tick();
                        self.pc = self.pc.wrapping_add_signed(steps as i16);
                    }
                }

                // LD r16, u16
                (0b00, (_, _, 0), 0b001) => {
                    match (mid_1, mid_2) {
                        // LD to BC
                        (0, 0) => {
                            self.c_reg = self.read_next();
                            self.b_reg = self.read_next();
                        }
                        // LD to DE
                        (0, 1) => {
                            self.e_reg = self.read_next();
                            self.d_reg = self.read_next();
                        }
                        // LD to HL
                        (1, 0) => {
                            self.l_reg = self.read_next();
                            self.h_reg = self.read_next();
                        }
                        // LD to SP
                        (1, 1) => {
                            let lower = self.read_next() as u16;
                            let higher = self.read_next() as u16;
                            self.sp = (higher << 8) | lower;
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // ADD HL, r16
                (0b00, (_, _, 1), 0b001) => {
                    match (mid_1, mid_2) {
                        // BC
                        (0, 0) => {
                            let result = self.add_16(self.get_hl(), self.get_bc());
                            self.set_hl(result);
                        }
                        // DE
                        (0, 1) => {
                            let result = self.add_16(self.get_hl(), self.get_de());
                            self.set_hl(result);
                        }
                        // HL
                        (1, 0) => {
                            let result = self.add_16(self.get_hl(), self.get_hl());
                            self.set_hl(result);
                        }
                        // SP
                        (1, 1) => {
                            let result  = self.add_16(self.get_hl(), self.sp);
                            self.set_hl(result);
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // LD (r16), A
                (0b00, (_, _, 0), 0b010) => {
                    self.m_tick();
                    match (mid_1, mid_2) {
                        // BC
                        (0, 0) => {
                            self.mem.write(self.get_bc(), self.a_reg);
                        }
                        // DE
                        (0, 1) => {
                            self.mem.write(self.get_de(), self.a_reg);
                        }
                        // HL+
                        (1, 0) => {
                            self.mem.write(self.get_hl(), self.a_reg);
                            self.set_hl(self.get_hl().wrapping_add(1));
                        }
                        // HL-
                        (1, 1) => {
                            self.mem.write(self.get_hl(), self.a_reg);
                            self.set_hl(self.get_hl().wrapping_sub(1));
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // LD A, (r16)
                (0b00, (_, _, 1), 0b010) => {
                    self.m_tick();
                    match (mid_1, mid_2) {
                        // BC
                        (0, 0) => {
                            self.a_reg = self.mem.read(self.get_bc());
                        }
                        // DE
                        (0, 1) => {
                            self.a_reg = self.mem.read(self.get_de());
                        }
                        // HL+
                        (1, 0) => {
                            self.a_reg = self.mem.read(self.get_hl());
                            self.set_hl(self.get_hl().wrapping_add(1));
                        }
                        // HL-
                        (1, 1) => {
                            self.a_reg = self.mem.read(self.get_hl());
                            self.set_hl(self.get_hl().wrapping_sub(1));
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // INC r16
                (0b00, (_, _, 0), 0b011) => {
                    self.m_tick();
                    match (mid_1, mid_2) {
                        // BC
                        (0, 0) => {
                            self.set_bc(self.get_bc().wrapping_add(1));
                        }
                        // DE
                        (0, 1) => {
                            self.set_de(self.get_de().wrapping_add(1));
                        }
                        // HL
                        (1, 0) => {
                            self.set_hl(self.get_hl().wrapping_add(1));
                        }
                        // SP
                        (1, 1) => {
                            self.sp = self.sp.wrapping_add(1);
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // DEC r16
                (0b00, (_, _, 1), 0b011) => {
                    self.m_tick();
                    match (mid_1, mid_2) {
                        // BC
                        (0, 0) => {
                            self.set_bc(self.get_bc().wrapping_sub(1));
                        }
                        // DE
                        (0, 1) => {
                            self.set_de(self.get_de().wrapping_sub(1));
                        }
                        // HL
                        (1, 0) => {
                            self.set_hl(self.get_hl().wrapping_sub(1));
                        }
                        // SP
                        (1, 1) => {
                            self.sp = self.sp.wrapping_sub(1);
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // INC r8
                (0b00, _, 0b100) => {
                    let r8 = ((middle.0 << 2) | (middle.1 << 1) | middle.2) as u8;
                    let initial = self.get_r8(r8);
                    let result = self.inc_8(initial);
                    self.set_r8(r8, result);
                }

                // DEC r8
                (0b00, _, 0b101) => {
                    self.set_flag(N_FLAG, true);
                    let r8 = ((middle.0 << 2) | (middle.1 << 1) | middle.2) as u8;
                    let initial = self.get_r8(r8);
                    let result = self.dec_8(initial);
                    self.set_r8(r8, result);
                }

                // LD r8, u8
                (0b00, _, 0b110) => {
                    let r8 = ((middle.0 << 2) | (middle.1 << 1) | middle.2) as u8;
                    let imm = self.read_next();
                    self.set_r8(r8, imm);
                }

                // Accumulator/Flag operations
                (0b00, _, 0b111) => {
                    match middle {
                        // RLCA
                        (0, 0, 0) => {
                            self.a_reg = self.rotate_left(self.a_reg, true);
                            self.set_flag(Z_FLAG, false);
                        }
                        // RRCA
                        (0, 0, 1) => {
                            self.a_reg = self.rotate_right(self.a_reg, true);
                            self.set_flag(Z_FLAG, false);
                        }
                        // RLA
                        (0, 1, 0) => {
                            self.a_reg = self.rotate_left(self.a_reg, false);
                            self.set_flag(Z_FLAG, false);
                        }
                        // RRA
                        (0, 1, 1) => {
                            self.a_reg = self.rotate_right(self.a_reg, false);
                            self.set_flag(Z_FLAG, false);
                        }
                        // DAA
                        (1, 0, 0) => {
                            let mut offset: u8 = 0;
                            let subtract = self.read_flag(N_FLAG) == 1;

                            if (!subtract && self.a_reg & 0xF > 0x9) || self.read_flag(H_FLAG) == 1 {
                                offset |= 0x6;
                            }
                            if (!subtract && self.a_reg > 0x99) || self.read_flag(C_FLAG) == 1 {
                                offset |= 0x60;
                                self.set_flag(C_FLAG, true);
                            } else {
                                self.set_flag(C_FLAG, false);
                            }

                            if !subtract {
                                self.a_reg = self.a_reg.wrapping_add(offset);
                            } else {
                                self.a_reg = self.a_reg.wrapping_sub(offset);
                            }

                            self.set_flag(Z_FLAG, self.a_reg == 0);
                            self.set_flag(H_FLAG, false);
                            
                        }
                        // CPL
                        (1, 0, 1) => {
                            self.a_reg = !self.a_reg;
                            self.set_flag(N_FLAG, true);
                            self.set_flag(H_FLAG, true);
                        }
                        // SCF
                        (1, 1, 0) => {
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                            self.set_flag(C_FLAG, true);
                        }
                        // CCF
                        (1, 1, 1) => {
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                            let flipped = self.read_flag(C_FLAG) == 0;
                            self.set_flag(C_FLAG, flipped);
                        }
                        (_, _, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }
            
            // Block 1 (01)
                // HALT
                (0b01, (1, 1, 0), 0b110) => {
                    // TODO
                }

                // LD r8, r8
                (0b01, _, _) => {
                    let r8 = ((middle.0 << 2) | (middle.1 << 1) | middle.2) as u8;
                    let source = self.get_r8(bottom as u8);
                    self.set_r8(r8, source);
                }
            
            // Block 2 (10) (ALU A, r8)
                // ADD
                (0b10, (0, 0, 0), _) => {
                    let source = self.get_r8(bottom as u8);
                    self.a_reg = self.add_8(self.a_reg, source, 0);
                }

                // ADC
                (0b10, (0, 0, 1), _) => {
                    let source = self.get_r8(bottom as u8);
                    let carry = self.read_flag(C_FLAG);
                    self.a_reg = self.add_8(self.a_reg, source, carry);
                }

                // SUB
                (0b10, (0, 1, 0), _) => {
                    let source = self.get_r8(bottom as u8);
                    self.a_reg = self.sub_8(self.a_reg, source, 0);
                }

                // SBC
                (0b10, (0, 1, 1), _) => {
                    let source = self.get_r8(bottom as u8);
                    let carry = self.read_flag(C_FLAG);
                    self.a_reg = self.sub_8(self.a_reg, source, carry);
                }

                // AND
                (0b10, (1, 0, 0), _) => {
                    let source = self.get_r8(bottom as u8);
                    self.a_reg &= source;
                    self.set_flag(Z_FLAG, self.a_reg == 0);
                    self.set_flag(N_FLAG, false);
                    self.set_flag(H_FLAG, true);
                    self.set_flag(C_FLAG, false);
                }

                // XOR
                (0b10, (1, 0, 1), _) => {
                    let source = self.get_r8(bottom as u8);
                    self.a_reg ^= source;
                    self.set_flag(Z_FLAG, self.a_reg == 0);
                    self.set_flag(N_FLAG, false);
                    self.set_flag(H_FLAG, false);
                    self.set_flag(C_FLAG, false);
                }

                // OR
                (0b10, (1, 1, 0), _) => {
                    let source = self.get_r8(bottom as u8);
                    self.a_reg |= source;
                    self.set_flag(Z_FLAG, self.a_reg == 0);
                    self.set_flag(N_FLAG, false);
                    self.set_flag(H_FLAG, false);
                    self.set_flag(C_FLAG, false);
                }
                
                // CP
                (0b10, (1, 1, 1), _) => {
                    let source = self.get_r8(bottom as u8);
                    self.sub_8(self.a_reg, source, 0);
                }

            // Block 3 (11)
                // RET cond
                (0b11, (0, _, _), 0b000) => {
                    self.m_tick();
                    let cond: bool;
                    match (mid_2, mid_3) {
                        // Not Z
                        (0, 0) => {
                            cond = self.read_flag(Z_FLAG) == 0;
                        }
                        // Z
                        (0, 1) => {
                            cond = self.read_flag(Z_FLAG) == 1;
                        }
                        // Not C
                        (1, 0) => {
                            cond = self.read_flag(C_FLAG) == 0;
                        }
                        // C
                        (1, 1) => {
                            cond = self.read_flag(C_FLAG) == 1;
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                    if cond {
                        let value = self.pop_16();
                        self.m_tick();
                        self.pc = value;
                    }
                }

                // LD (FF00 + u8), A
                (0b11, (1, 0, 0), 0b000) => {
                    let offset = self.read_next() as u16;
                    self.m_tick();
                    self.mem.write(0xFF00 + offset, self.a_reg);
                }

                // LD A, (FF00 + u8)
                (0b11, (1, 1, 0), 0b000) => {
                    let offset = self.read_next() as u16;
                    self.m_tick();
                    self.a_reg = self.mem.read(0xFF00 + offset);
                }

                // ADD SP, i8
                (0b11, (1, 0, 1), 0b000) => {
                    let value = self.read_next() as i8;
                    self.m_tick();
                    self.m_tick();
                    self.sp = self.add_sp_signed(value);
                }

                // LD HL, SP + i8
                (0b11, (1, 1, 1), 0b000) => {
                    let value = self.read_next() as i8;
                    self.m_tick();
                    let result = self.add_sp_signed(value);
                    self.set_hl(result);
                }

                // POP r16
                (0b11, (_, _, 0), 0b001) => {
                    let value = self.pop_16();
                    match (mid_1, mid_2) {
                        // BC
                        (0, 0) => {
                            self.set_bc(value);
                        }
                        // DE
                        (0, 1) => {
                            self.set_de(value);
                        }
                        // HL
                        (1, 0) => {
                            self.set_hl(value);
                        }
                        // AF
                        (1, 1) => {
                            self.set_af(value & 0xFFF0);
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // RET
                (0b11, (0, 0, 1), 0b001) => {
                    let value = self.pop_16();
                    self.m_tick();
                    self.pc = value;
                }

                // RETI
                (0b11, (0, 1, 1), 0b001) => {
                    let value = self.pop_16();
                    self.m_tick();
                    self.pc = value;
                    // TODO Interrupt flags
                }

                // JP HL
                (0b11, (1, 0, 1), 0b001) => {
                    self.pc = self.get_hl();
                }

                // LD SP, HL
                (0b11, (1, 1, 1), 0b001) => {
                    self.sp = self.get_hl();
                }

                // JP cond
                (0b11, (0, _, _), 0b010) => {
                    let lower = self.read_next() as u16;
                    let higher = self.read_next() as u16;
                    let combined = (higher << 8) | lower;
                    let cond: bool;
                    match (mid_2, mid_3) {
                        // Not Z
                        (0, 0) => {
                            cond = self.read_flag(Z_FLAG) == 0;
                        }
                        // Z
                        (0, 1) => {
                            cond = self.read_flag(Z_FLAG) == 1;
                        }
                        // Not C
                        (1, 0) => {
                            cond = self.read_flag(C_FLAG) == 0;
                        }
                        // C
                        (1, 1) => {
                            cond = self.read_flag(C_FLAG) == 1;
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                    if cond {
                        self.m_tick();
                        self.pc = combined;
                    }
                }

                // LD (FF00 + C), A
                (0b11, (1, 0, 0), 0b010) => {
                    self.m_tick();
                    self.mem.write(0xFF00 + (self.c_reg as u16), self.a_reg);
                }

                // LD A, (FF00 + C)
                (0b11, (1, 1, 0), 0b010) => {
                    self.m_tick();
                    self.a_reg = self.mem.read(0xFF00 + (self.c_reg as u16));
                }

                // LD (u16), A
                (0b11, (1, 0, 1), 0b010) => {
                    let lower = self.read_next() as u16;
                    let higher = self.read_next() as u16;
                    self.m_tick();
                    self.mem.write((higher << 8) | lower, self.a_reg);
                }

                // LD A, (u16)
                (0b11, (1, 1, 1), 0b010) => {
                    let lower = self.read_next() as u16;
                    let higher = self.read_next() as u16;
                    self.m_tick();
                    self.a_reg = self.mem.read((higher << 8) | lower);
                }

                // JP u16
                (0b11, (0, 0, 0), 0b011) => {
                    let lower = self.read_next() as u16;
                    let higher = self.read_next() as u16;
                    self.m_tick();
                    self.pc = (higher << 8) | lower;
                }

                // DI
                (0b11, (1, 1, 0), 0b011) => {
                    // TODO
                }

                // EI
                (0b11, (1, 1, 1), 0b011) => {
                    // TODO
                }

                // CALL cond
                (0b11, (0, _, _), 0b100) => {
                    let lower = self.read_next() as u16;
                    let higher = self.read_next() as u16;
                    let combined = (higher << 8) | lower;
                    let cond: bool;
                    match (mid_2, mid_3) {
                        // Not Z
                        (0, 0) => {
                            cond = self.read_flag(Z_FLAG) == 0;
                        }
                        // Z
                        (0, 1) => {
                            cond = self.read_flag(Z_FLAG) == 1;
                        }
                        // Not C
                        (1, 0) => {
                            cond = self.read_flag(C_FLAG) == 0;
                        }
                        // C
                        (1, 1) => {
                            cond = self.read_flag(C_FLAG) == 1;
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                    if cond {
                        self.push_16(self.pc);
                        self.pc = combined;
                    }
                }

                // PUSH r16
                (0b11, (_, _, 0), 0b101) => {
                    match (mid_1, mid_2) {
                        // BC
                        (0, 0) => {
                            self.push_16(self.get_bc());
                        }
                        // DE
                        (0, 1) => {
                            self.push_16(self.get_de());
                        }
                        // HL
                        (1, 0) => {
                            self.push_16(self.get_hl());
                        }
                        // AF
                        (1, 1) => {
                            self.push_16(self.get_af());
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // CALL u16
                (0b11, (0, 0, 1), 0b101) => {
                    let lower = self.read_next() as u16;
                    let higher = self.read_next() as u16;
                    let combined = (higher << 8) | lower;
                    self.push_16(self.pc);
                    self.pc = combined;
                }

                // ALU operations
                (0b11, _, 0b110) => {
                    match middle {
                        // ADD
                        (0, 0, 0) => {
                            let operand = self.read_next();
                            self.a_reg = self.add_8(self.a_reg, operand, 0);
                        }
                        // ADC
                        (0, 0, 1) => {
                            let operand = self.read_next();
                            let carry = self.read_flag(C_FLAG);
                            self.a_reg = self.add_8(self.a_reg, operand, carry);
                        }
                        // SUB
                        (0, 1, 0) => {
                            let operand = self.read_next();
                            self.a_reg = self.sub_8(self.a_reg, operand, 0);
                        }
                        // SBC
                        (0, 1, 1) => {
                            let operand = self.read_next();
                            let carry = self.read_flag(C_FLAG);
                            self.a_reg = self.sub_8(self.a_reg, operand, carry);
                        }
                        // AND
                        (1, 0, 0) => {
                            let operand = self.read_next();
                            self.a_reg &= operand;
                            self.set_flag(Z_FLAG, self.a_reg == 0);
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, true);
                            self.set_flag(C_FLAG, false);
                        }
                        // XOR
                        (1, 0, 1) => {
                            let operand = self.read_next();
                            self.a_reg ^= operand;
                            self.set_flag(Z_FLAG, self.a_reg == 0);
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                            self.set_flag(C_FLAG, false);
                        }
                        // OR
                        (1, 1, 0) => {
                            let operand = self.read_next();
                            self.a_reg |= operand;
                            self.set_flag(Z_FLAG, self.a_reg == 0);
                            self.set_flag(N_FLAG, false);
                            self.set_flag(H_FLAG, false);
                            self.set_flag(C_FLAG, false);
                        }
                        // CP
                        (1, 1, 1) => {
                            let operand = self.read_next();
                            self.sub_8(self.a_reg, operand, 0);
                        }
                        (_, _, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // RST
                (0b11, _, 0b111) => {
                    self.push_16(self.pc);
                    self.pc = (mid_1 << 5) | (mid_2 << 4) | (mid_3 << 3);
                }

            // Nonexistent opcodes
                (_, _, _) => unimplemented!("Unimplemented opcode: {}", op),
            }
        }
    }

    // Reads byte at PC and increments PC
    fn read_next(&mut self) -> u8 {
        self.m_tick();
        let next = self.mem.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        next
    }

    // Load value into mem at addr
    fn ld_mem(&mut self, addr: u16, value: u8) {
        self.m_tick();
        self.mem.write(addr, value);
    }

    // 16 bit wrapping addition, updates Half Carry and Carry flags based on bits 11 and 15
    fn add_16(&mut self, first: u16, second: u16) -> u16 {
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (first & 0xFFF) + (second & 0xFFF) > 0xFFF);
        self.m_tick();
        let result = first.overflowing_add(second);
        self.set_flag(C_FLAG, result.1);
        result.0
    }

    // 8 bit wrapping addition, updates Half Carry, Zero, and Carry flags
    fn add_8(&mut self, first: u8, second: u8, carry: u8) -> u8 {
        let c_result = first.overflowing_add(carry);
        let mut c = c_result.1;

        let result = c_result.0.overflowing_add(second);
        c |= result.1;

        self.set_flag(Z_FLAG, result.0 == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (first & 0xF) + (second & 0xF) + carry > 0xF);
        self.set_flag(C_FLAG, c);
        result.0
    }

    // Adds a signed 8 bit integer to SP and updates Half Carry and Carry flags based on bits 3 and 7
    fn add_sp_signed(&mut self, small: i8) -> u16 {
        self.set_flag(Z_FLAG, false);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (self.sp & 0xF).wrapping_add((small as u16) & 0xF) > 0xF);
        self.set_flag(C_FLAG, (self.sp & 0xFF).wrapping_add((small as u16) & 0xFF) > 0xFF);
        self.sp.wrapping_add_signed(small.into())
    }

    fn inc_8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, (value & 0xF) + 1 > 0xF);
        result
    }

    // 8 bit wrapping subtraction, updates Half Carry, Zero, and Carry flags
    fn sub_8(&mut self, first: u8, second: u8, carry: u8) -> u8 {
        let c_result = first.overflowing_sub(carry);
        let mut c = c_result.1;

        let result = c_result.0.overflowing_sub(second);
        c |= result.1;

        self.set_flag(Z_FLAG, result.0 == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, (first & 0xF) < ((second & 0xF) + carry));
        self.set_flag(C_FLAG, c);
        result.0
    }

    fn dec_8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.set_flag(Z_FLAG, result == 0);
        self.set_flag(N_FLAG, true);
        self.set_flag(H_FLAG, (value & 0xF) < 1);
        result
    }

    fn rotate_left(&mut self, mut value: u8, c: bool) -> u8 {
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, false);
        let highest = value >> 7;
        value <<= 1;
        if c {
            value |= highest;
        } else {
            value |= self.read_flag(C_FLAG);
        }
        self.set_flag(C_FLAG, highest == 1);
        self.set_flag(Z_FLAG, value == 0);
        value
    }

    fn rotate_right(&mut self, mut value: u8, c: bool) -> u8 {
        self.set_flag(N_FLAG, false);
        self.set_flag(H_FLAG, false);
        let lowest = value & 1;
        value >>= 1;
        if c {
            value |= lowest << 7;
        } else {
            value |= self.read_flag(C_FLAG) << 7;
        }
        self.set_flag(C_FLAG, lowest == 1);
        self.set_flag(Z_FLAG, value == 0);
        value
    }

    // Retrieves value from the register encoded into 3 bits
    fn get_r8(&mut self, r8: u8) -> u8 {
        match r8 {
            // B
            0b000 => {
                self.b_reg
            }
            // C
            0b001 => {
                self.c_reg
            }
            // D
            0b010 => {
                self.d_reg
            }
            // E
            0b011 => {
                self.e_reg
            }
            // H
            0b100 => {
                self.h_reg
            }
            // L
            0b101 => {
                self.l_reg
            }
            // (HL)
            0b110 => {
                self.m_tick();
                self.mem.read(self.get_hl())
            }
            // A
            0b111 => {
                self.a_reg
            }
            _ => {
                panic!("Invalid value for bits")
            }
        }
    }

    // Sets value of the register encoded into 3 bits
    fn set_r8(&mut self, r8: u8, value: u8) {
        match r8 {
            // B
            0b000 => {
                self.b_reg = value;
            }
            // C
            0b001 => {
                self.c_reg = value;
            }
            // D
            0b010 => {
                self.d_reg = value;
            }
            // E
            0b011 => {
                self.e_reg = value;
            }
            // H
            0b100 => {
                self.h_reg = value;
            }
            // L
            0b101 => {
                self.l_reg = value;
            }
            // (HL)
            0b110 => {
                self.m_tick();
                self.mem.write(self.get_hl(), value);
            }
            // A
            0b111 => {
                self.a_reg = value;
            }
            _ => {
                panic!("Invalid value for bits")
            }
        }
    }

    fn push_16(&mut self, value: u16) {
        let higher = (value >> 8) as u8;
        let lower = value as u8;

        self.m_tick();
        self.sp = self.sp.wrapping_sub(1);

        self.m_tick();
        self.mem.write(self.sp, higher);
        self.sp = self.sp.wrapping_sub(1);

        self.m_tick();
        self.mem.write(self.sp, lower);
    }

    fn pop_16(&mut self) -> u16 {
        self.m_tick();
        let lower = self.mem.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        self.m_tick();
        let higher = self.mem.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        higher << 8 | lower
    }
}