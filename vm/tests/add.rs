use vm::{Register, VirtualMachine};

#[test]
fn immediate_mode_positive() {
    // 0001 000 000 1 00001 = 0x1021 = ADD R0 R0 1
    // 0001 001 001 1 00010 = 0x1262 = ADD R1 R1 2
    let binary = vec![0x1021, 0x1262];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();
    vm.step();

    assert_eq!(1, vm.registers.get(Register::R0.into()));
    assert_eq!(2, vm.registers.get(Register::R1.into()));
    assert_eq!(1, 0x0001 & vm.registers.get(Register::PSR.into()));
}

#[test]
fn immediate_mode_negative() {
    // 0001 000 000 1 11111 = 0x103F = ADD R0 R0 -1
    // 0001 001 001 1 11110 = 0x127E = ADD R1 R1 -2
    // 0001 010 001 1 11110 = 0x127E = ADD R2 R1 -2
    let binary = vec![0x103F, 0x127E, 0x147E];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();
    vm.step();
    vm.step();

    assert_eq!(0xFFFF, vm.registers.get(Register::R0.into()));
    assert_eq!(0xFFFE, vm.registers.get(Register::R1.into()));
    assert_eq!(0xFFFC, vm.registers.get(Register::R2.into()));
    assert_eq!(1, (0x0004 & vm.registers.get(Register::PSR.into())) >> 2);
}

#[test]
fn immediate_mode_zero() {
    // 0001 000 000 1 00000 = 0x1020 = ADD R0 R0 0
    let binary = vec![0x1020];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();

    assert_eq!(0, vm.registers.get(Register::R0.into()));
    assert_eq!(1, (0x0002 & vm.registers.get(Register::PSR.into())) >> 1);
}

#[test]
fn register_mode() {
    // 0001 000 000 1 00001 = 0x1021 = ADD R0 R0 1
    // 0001 001 001 1 00010 = 0x1262 = ADD R1 R1 2
    // 0001 010 001 0 00000 = 0x1440 = ADD R2 R1 R0
    let binary = vec![0x1021, 0x1262, 0x1440];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();
    vm.step();
    vm.step();

    assert_eq!(3, vm.registers.get(Register::R2.into()));
    assert_eq!(1, (0x0001 & vm.registers.get(Register::PSR.into())));
}
