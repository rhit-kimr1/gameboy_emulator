pub struct Memory {
    sys_clock: u16,
    // Split into 2 0x4000 arrays when implementing MBCs
    rom_bank: [u8; 0x8000],
    vram: [u8; 0x2000],
    sram: [u8; 0x2000],
    // Split into 2 0x1000 arrays if upgrading to CGB
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    hram: [u8; 0x7F],
    joypad: u8,
    joypad_buttons: u8,
    joypad_dpad: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    timer_and: bool,
    tima_overflowed: bool,
    if_reg: u8,
    ie_reg: u8,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            sys_clock: 0xAB00, 
            rom_bank: [0; 0x8000],
            vram: [0; 0x2000],
            sram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            hram: [0; 0x7F],
            joypad: 0xCF,
            joypad_buttons: 0xF,
            joypad_dpad: 0xF,
            tima: 0,
            tma: 0,
            tac: 0xF8,
            timer_and: false,
            tima_overflowed: false,
            if_reg: 0xE1,
            ie_reg: 0,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let index = addr as usize;
        // Unused Addresses
        if addr == 0xFF03 || (0xFF08 <= addr) && (addr <= 0xFF0E) || addr == 0xFF15 || addr == 0xFF1F || (0xFF27 <= addr) && (addr <= 0xFF2F) ||
            (0xFF4C <= addr) && (addr <= 0xFF4E) || (0xFF56 <= addr) && (addr <= 0xFF67) || (0xFF6C <= addr) && (addr <= 0xFF6F) {
                0xFF
            }
        // ROM
        else if addr < 0x8000 {
            self.rom_bank[index]
        }
        // VRAM
        else if addr < 0xA000 {
            self.vram[index - 0x8000]
        }
        // SRAM
        else if addr < 0xC000 {
            self.sram[index - 0xA000]
        }
        // WRAM
        else if addr < 0xE000 {
            self.wram[index - 0xC000]
        }
        // Echo RAM
        else if addr < 0xFE00 {
            self.wram[index - 0xE000]
        }
        // OAM
        else if addr < 0xFEA0 {
            self.oam[index - 0xFE00]
        }
        // Prohibited Range
        else if addr < 0xFF00 {
            // OAM corruption bug not implemented
            0
        }
        // IO
        else if addr < 0xFF80 {
            // Joypad
            if addr == 0xFF00 {
                let select = (self.joypad >> 4) & 0b0011;
                // None
                if select == 0b11 {
                    0xF
                }
                // D-Pad
                else if select == 0b10 {
                    0xE0 | self.joypad_dpad
                }
                // Buttons
                else if select == 0b01 {
                    0xD0 | self.joypad_buttons
                }
                // Both
                else {
                    0xC0 | (self.joypad_buttons & self.joypad_dpad)
                }
            }

             // Serial
            else if addr == 0xFF01 {
                // TODO
                0xFF
            }
            else if addr == 0xFF02 {
                // TODO
                0xFF
            }

            // Timers
            // DIV
            else if addr == 0xFF04 {
                (self.sys_clock >> 8) as u8
            }
            // TIMA
            else if addr == 0xFF05 {
                self.tima
            }
            // TMA
            else if addr == 0xFF06 {
                self.tma
            }
            // TAC
            else if addr == 0xFF07 {
                self.tac
            }

            // Interrupt Flag
            else if addr == 0xFF0F {
                self.if_reg
            }

            // Audio
            else if addr < 0xFF27 {
                // TODO
                0xFF
            }
            // Wave Pattern
            else if addr < 0xFF40 {
                // TODO
                0xFF
            }

            // LCD
            // else if addr < 0xFF4C {
                
            // }

            // VRAM Bank Select
            else if addr == 0xFF4F {
                // TODO if upgrading to CGB
                0xFF
            }

            // VRAM DMA
            else if addr < 0xFF56 {
                // TODO if upgrading to CGB
                0xFF
            }

            // Color Palettes
            else if addr < 0xFF6C {
                // TODO if upgrading to CGB
                0xFF
            }

            // WRAM Bank Select
            else if addr == 0xFF70 {
                // TODO if upgrading to CGB
                0xFF
            }

            else {
                0xFF
            }
        }
        // HRAM
        else if addr < 0xFFFF {
            self.hram[index - 0xFF80]
        }
        // Interrupt Enable Register
        else {
            self.ie_reg
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let index = addr as usize;
        // Unused Addresses
        if addr == 0xFF03 || (0xFF08 <= addr) && (addr <= 0xFF0E) || addr == 0xFF15 || addr == 0xFF1F || (0xFF27 <= addr) && (addr <= 0xFF2F) ||
            (0xFF4C <= addr) && (addr <= 0xFF4E) || (0xFF56 <= addr) && (addr <= 0xFF67) || (0xFF6C <= addr) && (addr <= 0xFF6F) {
                // Do nothing
            }
        // ROM (read-only)
        if addr < 0x8000 {}
        // VRAM
        else if addr < 0xA000 {
            self.vram[index - 0x8000] = data;
        }
        // SRAM
        else if addr < 0xC000 {
            self.sram[index - 0xA000] = data;
        }
        // WRAM
        else if addr < 0xE000 {
            self.wram[index - 0xC000] = data;
        }
        // Echo RAM
        else if addr < 0xFE00 {
            self.wram[index - 0xE000] = data;
        }
        // OAM
        else if addr < 0xFEA0 {
            self.oam[index - 0xFE00] = data;
        }
        // Prohibited Range (read-only)
        else if addr < 0xFF00 {}
        // IO
        else if addr < 0xFF80 {
            // Joypad
            if addr == 0xFF00 {
                self.joypad = (data & 0x30) | 0xC0;
            }

            // Serial
            if addr == 0xFF01 {
                // TODO

                // Capturing writes to Serial Data for tests
                print!("{}", data as char);
            }
            if addr == 0xFF02 {
                // TODO
            }

            // Timers
            // DIV
            if addr == 0xFF04 {
                self.sys_clock = 0;
            }
            // TIMA
            if addr == 0xFF05 {
                self.tima = data;
                self.tima_overflowed = false;
            }
            // TMA
            if addr == 0xFF06 {
                self.tma = data;
            }
            // TAC
            if addr == 0xFF07 {
                self.tac = data | 0xF8;
            }

            // Interrupt Flag
            if addr == 0xFF0F {
                self.if_reg = data | 0xE0;
            }

            // Audio
            else if addr < 0xFF27 {
                // TODO
            }
            // Wave Pattern
            else if addr < 0xFF40 {
                // TODO
            }

            // VRAM Bank Select
            else if addr == 0xFF4F {
                // TODO if upgrading to CGB
            }

            // VRAM DMA
            else if addr < 0xFF56 {
                // TODO if upgrading to CGB
            }

            // Color Palettes
            else if addr < 0xFF6C {
                // TODO if upgrading to CGB
            }

            // WRAM Bank Select
            else if addr == 0xFF70 {
                // TODO if upgrading to CGB
            }
        }
        // HRAM
        else if addr < 0xFFFF {
            self.hram[index - 0xFF80] = data;
        }
        // Interrupt Enable Register
        else {
            self.ie_reg = data;
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let end = data.len() as usize;
        self.rom_bank[0..end].copy_from_slice(data);
    }

    pub fn inc_clk(&mut self) {
        // Technically inaccurate as DIV should be represented by bits 6-13 instead of 8-15, but the top 2 bits do not motter for DMG
        self.sys_clock = self.sys_clock.wrapping_add(4);
        self.detect_and();
    }

    fn detect_and(&mut self) {
        let select = self.tac & 0b11;
        let clock_bit: u8;
        let enable = ((self.tac >> 2) & 1) as u16;
        if select == 0 {
            clock_bit = 9;
        }
        else if select == 1 {
            clock_bit = 3;
        }
        else if select == 2 {
            clock_bit = 5;
        }
        else {
            clock_bit = 7;
        }
        let and_result = (self.sys_clock >> clock_bit) & enable == 1;
        if self.timer_and && !and_result {
            if self.tima == 0xFF {
                self.tima_overflowed = true;
            }
            self.tima = self.tima.wrapping_add(1);
        }
        self.timer_and = and_result;
    }

    pub fn check_overflow(&mut self) {
        if self.tima_overflowed {
            self.tima_overflowed = false;
            self.tima = self.tma;
            self.if_reg |= 0b00100;
        }
    }
}