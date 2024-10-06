use assembler::{assemble, AssemblerError};

// HALT
#[test]
fn missing_halt() {
    let program = "
.ORIG   x3000
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::HaltUsage("Missing HALT".into()))
    );
}

#[test]
fn halt_given_operand() {
    let program = "
.ORIG   x3000
        HALT x3000
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::HaltUsage(
            "HALT cannot have operand - given: HALT x3000".into()
        ))
    );
}

#[test]
fn symbol_before_halt() {
    let program = "
.ORIG   x3000
SYMBOL  HALT
.END
";
    let binary = vec![0x3000, 0xf025];
    let output = assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}
