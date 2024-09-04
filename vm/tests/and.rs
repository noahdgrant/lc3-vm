use vm::{Register, VirtualMachine};

#[test]
fn immediate_mode() {
    // 0001 000 000 1 00111 = 0x1027 = ADD R0 R0 7
    // 0101 000 000 1 00010 = 0x5022 = AND R0 R0 2
    let binary = vec![0x1027, 0x5022];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();
    vm.step();

    assert_eq!(2, vm.registers.get(Register::R0.into()));
}

#[test]
fn register_mode() {
    // 0001 000 000 1 01111 = 0x102F = ADD R0 R0 15
    // 0001 001 001 1 00011 = 0x1263 = ADD R1 R1 3
    // 0101 010 001 0 00000 = 0x5440 = AND R2 R1 R0
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
}
