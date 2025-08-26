pub struct Video {
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc:u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    mode: u8,
}

impl Video {
    pub fn new() -> Self {
        Self {
            lcdc: 0x91,
            stat: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            mode: 2,
        }
    }

    pub fn tick(&mut self) {
        // TODO
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        if (addr < 0x8000) || ((addr >= 0xA000) && (addr < 0xFE00)) || ((addr >= 0xFEA0) && (addr < 0xFF40)) || (addr > 0xFF4B) {
            // Non-PPU address
            0xFF
        }
        else if addr < 0xA000 {
            if self.mode == 3 {
                0xFF
            } else {
                self.vram[(addr - 0x8000) as usize]
            }
        }
        else if addr < 0xFEA0 {
            if self.mode == 2 || self.mode == 3 {
                0xFF
            } else {
                self.oam[(addr - 0xFE00) as usize]
            }
        }
        else if addr == 0xFF40 {
            self.lcdc
        }
        else if addr == 0xFF41 {
            self.stat
        }
        else if addr == 0xFF42 {
            self.scy
        }
        else if addr == 0xFF43 {
            self.scx
        }
        else if addr == 0xFF44 {
            self.ly
        }
        else if addr == 0xFF45 {
            self.lyc
        }
        else if addr == 0xFF46 {
            // Handled in memory unit
            0xFF
        }
        else if addr == 0xFF47 {
            self.bgp
        }
        else if addr == 0xFF48 {
            self.obp0
        }
        else if addr == 0xFF49 {
            self.obp1
        }
        else if addr == 0xFF4A {
            self.wy
        }
        else {
            self.wx
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if (addr < 0x8000) || ((addr >= 0xA000) && (addr < 0xFE00)) || ((addr >= 0xFEA0) && (addr < 0xFF40)) || (addr > 0xFF4B) {
            // Non-PPU address
        }
        else if addr < 0xA000 {
            self.vram[(addr - 0x8000) as usize] = data;
        }
        else if addr < 0xFEA0 {
            self.oam[(addr - 0xFE00) as usize] = data;
        }
        else if addr == 0xFF40 {
            self.lcdc = data;
        }
        else if addr == 0xFF41 {
            self.stat = data | 0x80 | (self.stat & 0x7);
        }
        else if addr == 0xFF42 {    
            self.scy = data;
        }
        else if addr == 0xFF43 {
            self.scx = data;
        }
        else if addr == 0xFF44 {
            // Read only
        }
        else if addr == 0xFF45 {
            self.lyc = data;
        }
        else if addr == 0xFF46 {
            // Handled in memory unit
        }
        else if addr == 0xFF47 {
            self.bgp = data;
        }
        else if addr == 0xFF48 {
            self.obp0 = data;
        }
        else if addr == 0xFF49 {
            self.obp1 = data;
        }
        else if addr == 0xFF4A {
            self.wy = data;
        }
        else {
            self.wx = data;
        }
    }
}