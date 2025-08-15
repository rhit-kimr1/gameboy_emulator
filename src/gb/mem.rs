const MEM_SIZE: usize = 0xFFFF;

pub struct Memory {
    array: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            array: [0; MEM_SIZE],
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        // TODO
        0
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // TODO
    }
}