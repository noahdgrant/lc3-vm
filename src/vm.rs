use crate::register::Registers;

const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct VirtualMachine {
    memory: [u16; MEMORY_SIZE],
    registers: Registers,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }
}
