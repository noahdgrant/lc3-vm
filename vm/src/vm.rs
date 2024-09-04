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
        self.registers.increment_pc_register();
        let instruction = self.memory.read(pc);
        self.registers.set(Register::IR.into(), instruction);
        instruction
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        instruction::execute(self, instruction);
    }

    pub fn run(&mut self) {
        // TODO: Add 'or HALT' to stop early or maybe change this to just a loop
        while self.registers.get(Register::PC.into()) < MEMORY_SIZE as u16 {
            self.step();
        }
    }

    pub fn get_mode(&self) -> PrivilegeMode {
        if ((PrivilegeMode::User as u16) << 15) & self.registers.get(Register::PSR.into()) == 1 {
            PrivilegeMode::User
        } else {
            PrivilegeMode::Privileged
        }
    }
}

#[derive(PartialEq, Eq)]
#[repr(u16)]
pub enum PrivilegeMode {
    Privileged = 0,
    User = 1,
}

impl From<PrivilegeMode> for u16 {
    fn from(mode: PrivilegeMode) -> Self {
        mode as u16
    }
}
