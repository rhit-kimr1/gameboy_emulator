mod mem;
use crate::gb::mem::Memory;

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
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a_reg as u16) << 8 | self.f_reg as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a_reg = ((value & 0xFF00) >> 8) as u8;
        self.f_reg = (value & 0xFF) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        (self.b_reg as u16) << 8 | self.c_reg as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b_reg = ((value & 0xFF00) >> 8) as u8;
        self.c_reg = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d_reg as u16) << 8 | self.e_reg as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d_reg = ((value & 0xFF00) >> 8) as u8;
        self.e_reg = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h_reg as u16) << 8 | self.l_reg as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h_reg = ((value & 0xFF00) >> 8) as u8;
        self.l_reg = (value & 0xFF) as u8;
    }
}