pub const MEMORY_SIZE: usize = u16::MAX as usize;
pub const UNPRIVILEGED_MEMORY: u16 = 0x3000;

pub struct Memory {
    memory: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn dump(&self) {
        todo!();
    }

    pub fn read(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    pub fn is_privileged(&self, address: u16) -> bool {
        if address < UNPRIVILEGED_MEMORY {
            true
        } else {
            false
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
