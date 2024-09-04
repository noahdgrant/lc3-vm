use crate::instruction;
use crate::memory::{Memory, MEMORY_SIZE};
use crate::register::{Register, Registers};

#[derive(Default)]
pub struct VirtualMachine {
    pub registers: Registers,
    pub memory: Memory,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.registers.get(Register::PC.into());
        self.memory.read(pc)
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.registers.increment_pc_register();
        instruction::execute(self, instruction);
    }

    pub fn run(&mut self) {
        // TODO: Add 'or HALT' to stop early
        while self.registers.get(Register::PC.into()) < MEMORY_SIZE as u16 {
            self.step();
        }
    }
}
