mod mem;
use std::str::ParseBoolError;

use crate::gb::mem::Memory;

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
            pc: 0,
            sp: 0,
            a_reg: 0,
            b_reg: 0,
            c_reg: 0,
            d_reg: 0,
            e_reg: 0,
            f_reg: 0,
            h_reg: 0,
            l_reg: 0,
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
        if (self.f_reg >> flag) == 0 {
            0
        } else {
            1
        }
    }

    fn tick_system(&mut self) {
        // TODO
    }

    fn m_tick(&mut self) {
        for x in 0..4 {
            self.tick_system();
        }
    }

    pub fn tick(&mut self) {
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

        } else {
            match (block, middle, bottom) {
            // Block 0 (00)
                // NOP
                (0b00, (0, 0, 0), 0b000) => {},
                
                // STOP (Should not be called on DMG, UPDATE IF UPGRADING TO CGB)
                (0b00, (0, 1, 0), 0b000) => unimplemented!("STOP should not be called"),

                // LD (u16), SP
                (0b00, (0, 0, 1), 0b000) => {
                    let sp_low = (self.sp & 0xF) as u8;
                    let sp_high = (self.sp >> 8) as u8;
                    let addr_low = self.read_next() as u16;
                    let addr_high = self.read_next() as u16;
                    let addr = (addr_high << 8) | addr_low;
                    self.ld_mem(addr, sp_low);
                    self.ld_mem(addr + 1, sp_high);
                }

                // JR u8 (Unconditional)
                (0b00, (0, 1, 1), 0b000) => {
                    let steps = self.read_next() as u16;
                    self.m_tick();
                    self.pc += steps;
                }

                // JR cond u8 (Conditional)
                (0b00, (1, _, _), 0b000) => {
                    let steps = self.read_next() as u16;
                    match (mid_2, mid_3) {
                        // Not Z
                        (0, 0) => {
                            if self.read_flag(Z_FLAG) == 0 {
                                self.m_tick();
                                self.pc += steps;
                            }
                        }
                        // Z
                        (0, 1) => {
                            if self.read_flag(Z_FLAG) == 1 {
                                self.m_tick();
                                self.pc += steps;
                            }
                        }
                        // Not C
                        (1, 0) => {
                            if !self.read_flag(C_FLAG) == 0 {
                                self.m_tick();
                                self.pc += steps;
                            }
                        }
                        // C
                        (1, 1) => {
                            if self.read_flag(Z_FLAG) == 1 {
                                self.m_tick();
                                self.pc += steps;
                            }
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
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
                    self.set_flag(N_FLAG, false);
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
                            self.set_hl(self.get_hl() + 1);
                        }
                        // HL-
                        (1, 1) => {
                            self.mem.write(self.get_hl(), self.a_reg);
                            self.set_hl(self.get_hl() - 1);
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
                            self.set_hl(self.get_hl() + 1);
                        }
                        // HL-
                        (1, 1) => {
                            self.a_reg = self.mem.read(self.get_hl());
                            self.set_hl(self.get_hl() - 1);
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
                            self.set_bc(self.get_bc() + 1);
                        }
                        // DE
                        (0, 1) => {
                            self.set_de(self.get_de() + 1);
                        }
                        // HL
                        (1, 0) => {
                            self.set_hl(self.get_hl() + 1);
                        }
                        // SP
                        (1, 1) => {
                            self.sp += 1;
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
                            self.set_bc(self.get_bc() - 1);
                        }
                        // DE
                        (0, 1) => {
                            self.set_de(self.get_de() - 1);
                        }
                        // HL
                        (1, 0) => {
                            self.set_hl(self.get_hl() - 1);
                        }
                        // SP
                        (1, 1) => {
                            self.sp -= 1;
                        }
                        (_, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // INC r8
                (0b00, (_, _, _), 0b100) => {
                    self.set_flag(N_FLAG, false);
                    match (mid_1, mid_2, mid_3) {
                        // B
                        (0, 0, 0) => {
                            self.b_reg = self.add_8(self.b_reg, 1, false);
                        }
                        // C
                        (0, 0, 1) => {
                            self.c_reg = self.add_8(self.c_reg, 1, false);
                        }
                        // D
                        (0, 1, 0) => {
                            self.d_reg = self.add_8(self.d_reg, 1, false);
                        }
                        // E
                        (0, 1, 1) => {
                            self.e_reg = self.add_8(self.e_reg, 1, false);
                        }
                        // H
                        (1, 0, 0) => {
                            self.h_reg = self.add_8(self.h_reg, 1, false);
                        }
                        // L
                        (1, 0, 1) => {
                            self.l_reg = self.add_8(self.l_reg, 1, false);
                        }
                        // (HL)
                        (1, 1, 0) => {
                            self.m_tick();
                            let value = self.mem.read(self.get_hl());
                            let result = self.add_8(value, 1, false);
                            self.m_tick();
                            self.mem.write(self.get_hl(), result);
                        }
                        // A
                        (1, 1, 1) => {
                            self.a_reg = self.add_8(self.a_reg, 1, false);
                        }
                        (_, _, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // DEC r8
                (0b00, (_, _, _), 0b101) => {
                    self.set_flag(N_FLAG, true);
                    match (mid_1, mid_2, mid_3) {
                        // B
                        (0, 0, 0) => {
                            self.b_reg = self.sub_8(self.b_reg, 1, false);
                        }
                        // C
                        (0, 0, 1) => {
                            self.c_reg = self.sub_8(self.c_reg, 1, false);
                        }
                        // D
                        (0, 1, 0) => {
                            self.d_reg = self.sub_8(self.d_reg, 1, false);
                        }
                        // E
                        (0, 1, 1) => {
                            self.e_reg = self.sub_8(self.e_reg, 1, false);
                        }
                        // H
                        (1, 0, 0) => {
                            self.h_reg = self.sub_8(self.h_reg, 1, false);
                        }
                        // L
                        (1, 0, 1) => {
                            self.l_reg = self.sub_8(self.l_reg, 1, false);
                        }
                        // (HL)
                        (1, 1, 0) => {
                            self.m_tick();
                            let value = self.mem.read(self.get_hl());
                            let result = self.sub_8(value, 1, false);
                            self.m_tick();
                            self.mem.write(self.get_hl(), result);
                        }
                        // A
                        (1, 1, 1) => {
                            self.a_reg = self.sub_8(self.a_reg, 1, false);
                        }
                        (_, _, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // LD r8, u8
                (0b00, (_, _, _), 0b110) => {
                    let imm = self.read_next();
                    match (mid_1, mid_2, mid_3) {
                        // B
                        (0, 0, 0) => {
                            self.b_reg = imm;
                        }
                        // C
                        (0, 0, 1) => {
                            self.c_reg = imm;
                        }
                        // D
                        (0, 1, 0) => {
                            self.d_reg = imm;
                        }
                        // E
                        (0, 1, 1) => {
                            self.e_reg = imm;
                        }
                        // H
                        (1, 0, 0) => {
                            self.h_reg = imm;
                        }
                        // L
                        (1, 0, 1) => {
                            self.l_reg = imm;
                        }
                        // (HL)
                        (1, 1, 0) => {
                            self.m_tick();
                            self.mem.write(self.get_hl(), imm);
                        }
                        // A
                        (1, 1, 1) => {
                            self.a_reg = imm;
                        }
                        (_, _, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

                // Accumulator/Flag operations
                (0b00, (_, _, _), 0b111) => {
                    match (mid_1, mid_2, mid_3) {
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
                                offset |= 0x66;
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
                    // Determine register source from lowest 3 bits
                    let mut source: u8;
                    match bottom {
                        // B
                        0b000 => {
                            source = self.b_reg;
                        }
                        // C
                        0b001 => {
                            source = self.c_reg;
                        }
                        // D
                        0b010 => {
                            source = self.d_reg;
                        }
                        // E
                        0b011 => {
                            source = self.e_reg;
                        }
                        // H
                        0b100 => {
                            source = self.h_reg;
                        }
                        // L
                        0b101 => {
                            source = self.l_reg;
                        }
                        // (HL)
                        0b110 => {
                            self.m_tick();
                            source = self.mem.read(self.get_hl());
                        }
                        // A
                        0b111 => {
                            source = self.a_reg;
                        }
                        _ => {
                            panic!("Invalid value for bits")
                        }
                    }

                    // Load into destination determined by middle 3 bits
                    match (mid_1, mid_2, mid_3) {
                        // B
                        (0, 0, 0) => {
                            self.b_reg = source;
                        }
                        // C
                        (0, 0, 1) => {
                            self.c_reg = source;
                        }
                        // D
                        (0, 1, 0) => {
                            self.d_reg = source;
                        }
                        // E
                        (0, 1, 1) => {
                            self.e_reg = source;
                        }
                        // H
                        (1, 0, 0) => {
                            self.h_reg = source;
                        }
                        // L
                        (1, 0, 1) => {
                            self.l_reg = source;
                        }
                        // (HL)
                        (1, 1, 0) => {
                            self.m_tick();
                            self.mem.write(self.get_hl(), source);
                        }
                        // A
                        (1, 1, 1) => {
                            self.a_reg = source;
                        }
                        (_, _, _) => {
                            panic!("Invalid value for bits")
                        }
                    }
                }

            // Nonexistent opcodes
                (_, _, _) => unimplemented!("Unimplemented opcode: {}", op),
            }
        }
    }

    // Reads byte at SP and increments SP
    fn read_next(&mut self) -> u8 {
        self.m_tick();
        let next = self.mem.read(self.sp);
        self.sp += 1;
        next
    }

    // Load value into mem at addr
    fn ld_mem(&mut self, addr: u16, value: u8) {
        self.m_tick();
        self.mem.write(addr, value);
    }

    // 16 bit wrapping addition, updates Half Carry and Carry flags
    fn add_16(&mut self, first: u16, second: u16) -> u16 {
        self.set_flag(H_FLAG, (first & 0xFFF).overflowing_add(second & 0xFFF).1);
        self.m_tick();
        let result = first.overflowing_add(second);
        self.set_flag(C_FLAG, result.1);
        result.0
    }

    // 8 bit wrapping addition, updates Half Carry, Zero, and Carry (If not INC/DEC) flags
    fn add_8(&mut self, first: u8, second: u8, carry: bool) -> u8 {
        self.set_flag(H_FLAG, (first & 0xF).overflowing_add(second & 0xF).1);
        let result = first.overflowing_add(second);
        self.set_flag(Z_FLAG, result.0 == 0);
        if carry {
            self.set_flag(C_FLAG, result.1);
        }
        result.0
    }

    fn sub_8(&mut self, first: u8, second: u8, carry: bool) -> u8 {
        self.set_flag(H_FLAG, (first & 0xF).overflowing_sub(second & 0xF).1);
        let result = first.overflowing_sub(second);
        self.set_flag(Z_FLAG, result.0 == 0);
        if carry {
            self.set_flag(C_FLAG, result.1);
        }
        result.0
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
}