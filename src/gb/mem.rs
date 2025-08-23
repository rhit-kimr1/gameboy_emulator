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
            if_reg: 0,
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
                if select == 0b00 {
                    0xF
                }
                // D-Pad
                else if select == 0b01 {
                    0x10 | self.joypad_dpad
                }
                // Buttons
                else if select == 0b10 {
                    0x20 | self.joypad_buttons
                }
                // Both
                else {
                    0x30 | (self.joypad_buttons & self.joypad_dpad)
                }
            }

            // DIV
            else if addr == 0xFF04 {
                (self.sys_clock >> 8) as u8
            }

            // Interrupt Flag
            else if addr == 0xFF0F {
                self.if_reg
            }
            
            //Temporary values so tests can run
            else if addr == 0xFF44 {
                0x90
            }
            else {
                0
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
        if addr < 0xA000 {
            self.vram[index - 0x8000] = data;
        }
        // SRAM
        if addr < 0xC000 {
            self.sram[index - 0xA000] = data;
        }
        // WRAM
        if addr < 0xE000 {
            self.wram[index - 0xC000] = data;
        }
        // Echo RAM
        if addr < 0xFE00 {
            self.wram[index - 0xE000] = data;
        }
        // OAM
        if addr < 0xFEA0 {
            self.oam[index - 0xFE00] = data;
        }
        // Prohibited Range (read-only)
        if addr < 0xFF00 {}
        // IO
        if addr < 0xFF80 {
            // Joypad
            if addr == 0xFF00 {
                self.joypad = (data & 0x30) | 0xC0;
            }

            // Serial
            if addr == 0xFF01 {
                // TODO
            }
            if addr == 0xFF02 {
                // TODO
            }

            // DIV
            if addr == 0xFF04 {
                self.sys_clock = 0;
            }

            // Interrupt Flag
            if addr == 0xFF0F {
                self.if_reg = data;
            }

            // Capturing writes to Serial Data for tests
            if addr == 0xFF01 {
                print!("{}", data as char);
            }
        }
        // HRAM
        if addr < 0xFFFF {
            self.hram[index - 0xFF80] = data;
        }
        // Interrupt Enable Register
        else {
            self.ie_reg = data & 0x1F;
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let end = data.len() as usize;
        self.rom_bank[0..end].copy_from_slice(data);
    }

    pub fn inc_clk(&mut self) {
        // Technically inaccurate as DIV should be represented by bits 6-13 instead of 8-15, but the top 2 bits do not motter for DMG
        self.sys_clock += 4;
    }
}