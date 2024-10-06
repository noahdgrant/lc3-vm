use assembler::{assemble, AssemblerError};

#[test]
fn add_literal_to_register() {
    let program = "
.ORIG   x3000
ADD R0, R0, x3
HALT
.END
";
    let binary: Vec<u16> = vec![0x3000, 0x1023, 0xf025];
    let output = assemble(program.to_string());
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
    let output = assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn add_with_symbol() {
    // TODO
    let _program = "
.ORIG   x3000
HALT
.END
";
}

#[test]
fn add_missing_operand() {
    let program = "
.ORIG   x3000
ADD R0, R0,
HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::Operands(
            "ADD needs 3 operands - given: 2".into()
        ))
    );
}

#[test]
fn add_invalid_operand() {
    let program = "
.ORIG   x3000
ADD R1, x5, R1
HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::Operands("Unknown register x5".into()))
    );
}
