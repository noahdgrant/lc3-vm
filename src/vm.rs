use crate::instruction;
use crate::register::{Register, Registers};

const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct VirtualMachine {
    registers: Registers,
    memory: [u16; MEMORY_SIZE],
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: [0; MEMORY_SIZE],
        }
    }

    fn read_memory(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    // TODO: Could return error if user tries to write to memory they are
    // not allowed to
    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    fn fetch(&mut self) -> u16 {
        self.read_memory(Register::PC as u16)
    }

    fn step(&mut self) {
        let instruction = self.fetch();
        self.registers.increment_pc_register();
        instruction::execute(self, instruction);
    }

    pub fn run(&mut self) {
        while self.registers.get(Register::PC) < MEMORY_SIZE as u16 {
            self.step();
        }
    }
}
