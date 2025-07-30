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
}