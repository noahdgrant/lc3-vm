use assembler::{assemble, AssemblerError};

// ORIG
#[test]
fn orig_is_first_line() {
    let program = "
        HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::OrigUsage(
            ".ORIG cannot have a symbol before it - given: HALT".into()
        ))
    );
}

#[test]
fn orig_missing_operand() {
    let program = "
.ORIG
        HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::OrigUsage(
            ".ORIG can only have 1 operand - given: 0".into()
        ))
    );
}

#[test]
fn orig_given_operands() {
    let program = "
.ORIG   x3000 x3000
        HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::OrigUsage(
            ".ORIG can only have 1 operand - given: 2".into()
        ))
    );
}

#[test]
fn symbol_before_orig() {
    let program = "
SYMBOL  .ORIG x3000
        HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::OrigUsage(
            ".ORIG cannot have a symbol before it - given: SYMBOL".into()
        ))
    );
}

#[test]
fn multiple_orig() {
    let program = "
.ORIG x3000
.ORIG x3001
        HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::OrigUsage("Can only have one .ORIG".into()))
    );
}

#[test]
fn orig_given_non_numeric() {
    let program = "
.ORIG   STARTADDR
        HALT
.END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::InvalidNumber("STARTADDR".into()))
    );
}

// END
#[test]
fn missing_end() {
    let program = "
.ORIG   x3000
        HALT
";
    let output = assemble(program.to_string());
    assert_eq!(output, Err(AssemblerError::EndUsage("Missing .END".into())));
}

#[test]
fn code_after_end() {
    let program = "
.ORIG   x3000
        HALT
.END
        ADD     R1, R2, R3
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::EndUsage(
            "Code after .END: ADD     R1, R2, R3".into()
        ))
    );
}

#[test]
fn end_given_operand() {
    let program = "
.ORIG   x3000
        HALT
.END    x3000
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::EndUsage(
            ".END cannot have operand - given: .END    x3000".into()
        ))
    );
}

#[test]
fn symbol_before_end() {
    let program = "
.ORIG   x3000
        HALT
SYMBOL .END
";
    let output = assemble(program.to_string());
    assert_eq!(
        output,
        Err(AssemblerError::EndUsage(
            ".END cannot have symbol or operand - given: SYMBOL .END".into()
        ))
    );
}
