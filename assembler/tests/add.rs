#[test]
fn add_literal_to_register() {
    let program = "
.ORIG   x3000
ADD R0, R0, x3
HALT
.END
";
    let binary: Vec<u16> = vec![0x3000, 0x1023, 0xf025];
    let output = assembler::assemble(program.to_string());
    println!("{:?}", output);
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn add_register_to_register() {
    let program = "
.ORIG   x3000
ADD R0, R0, #3
ADD R1, R1, b0100
ADD R2, R0, R1
HALT
.END
";
    let binary: Vec<u16> = vec![0x3000, 0x1023, 0x1264, 0x1401, 0xf025];
    let output = assembler::assemble(program.to_string());
    println!("{:?}", output);
    assert!(output.is_ok_and(|b| b == binary));
}
