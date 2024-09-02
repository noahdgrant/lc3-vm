use crate::instruction;
use crate::register::{Register, Registers};

pub const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct VirtualMachine {
    pub registers: Registers,
    memory: [u16; MEMORY_SIZE],
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: [0; MEMORY_SIZE],
        }
    }

    // TODO: Add a check to see if this is outside the available memory
    pub fn read_memory(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    // TODO: Could return error if user tries to write to memory they are
    // not allowed to or is outside the valid range
    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.registers.get(Register::PC as u16);
        self.read_memory(pc)
    }

    pub fn step(&mut self) {
        //self.dump_registers();

        let instruction = self.fetch();
        // TODO: turn this into debug log
        println!("Executing 0x{:X}\n", instruction);

        self.registers.increment_pc_register();
        instruction::execute(self, instruction);
    }

    pub fn run(&mut self) {
        // TODO: Add 'or HALT' to stop early
        while self.registers.get(Register::PC as u16) < MEMORY_SIZE as u16 {
            self.step();
        }
    }

    pub fn dump_memory(&self) {
        todo!();
    }

    pub fn dump_registers(&self) {
        // TODO: implement Display for registers to make this easier
        println!("R0: 0x{:X}", self.registers.get(Register::R0 as u16));
        println!("R1: 0x{:X}", self.registers.get(Register::R1 as u16));
        println!("R2: 0x{:X}", self.registers.get(Register::R2 as u16));
        println!("R3: 0x{:X}", self.registers.get(Register::R3 as u16));
        println!("R4: 0x{:X}", self.registers.get(Register::R4 as u16));
        println!("R5: 0x{:X}", self.registers.get(Register::R5 as u16));
        println!("R6: 0x{:X}", self.registers.get(Register::R6 as u16));
        println!("R7: 0x{:X}", self.registers.get(Register::R7 as u16));
        println!("PC: 0x{:X}", self.registers.get(Register::PC as u16));
        println!("COND: 0x{:X}", self.registers.get(Register::COND as u16));
    }
}
