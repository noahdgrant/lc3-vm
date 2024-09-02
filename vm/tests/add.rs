use vm::{Register, VirtualMachine};

#[test]
fn immediate_mode() {
    // 0001 000 000 1 00001 = 0x1021 = ADD R0 R0 1
    // 0001 001 001 1 00010 = 0x1262 = ADD R1 R1 2
    let binary = vec![0x1021, 0x1262];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    for line in binary {
        vm.write_memory(address, line);
        address += 1;
    }
    vm.step();
    vm.step();

    assert_eq!(1, vm.registers.get(Register::R0 as u16));
    assert_eq!(2, vm.registers.get(Register::R1 as u16));
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
        vm.write_memory(address, line);
        address += 1;
    }
    vm.step();
    vm.step();
    vm.step();

    assert_eq!(3, vm.registers.get(Register::R2 as u16));
}
