use vm::{Register, VirtualMachine};

#[test]
fn zero_offset() {
    // 0010 011 000000000 = 0x2600 = LD R3 0
    let binary = vec![0x2600, 0xFFFF];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    vm.registers.set(Register::PC.into(), address);
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();

    assert_eq!(0xFFFF, vm.registers.get(Register::R3.into()));
}

#[test]
fn positive_offset() {
    // 0010 011 000000001 = 0x2601 = LD R3 1
    let binary = vec![0x2601, 0x0000, 0x1111];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    vm.registers.set(Register::PC.into(), address);
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();

    assert_eq!(0x1111, vm.registers.get(Register::R3.into()));
}

#[test]
fn negative_offset() {
    // 0010 011 111111111 = 0x27FF = LD R3 -1
    let binary = vec![0x27FF];
    let mut address = 0x3000;

    let mut vm = VirtualMachine::new();
    vm.registers.set(Register::PC.into(), address);
    for line in binary {
        vm.memory.write(address, line);
        address += 1;
    }
    vm.step();

    assert_eq!(0x27FF, vm.registers.get(Register::R3.into()));
}
