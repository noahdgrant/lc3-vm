#[test]
fn simplest_program() {
    let program = "
.ORIG   x3000
HALT
.END
";
    let binary = vec![0x3000, 0xf025];
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn missing_orig() {
    let program = "
.END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_err());
}

#[test]
fn missing_end() {
    let program = "
.ORIG   x3000
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_err());
}

#[test]
fn error_if_code_before_orig() {
    let program = "
ADD     R0 R0 5
.ORIG   x3000
.END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_err());
}

#[test]
fn error_if_code_after_end() {
    let program = "
.ORIG   x3000
.END
ADD     R0 R0 5
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_err());
}

#[test]
fn orig_missing_numeric() {
    let program = "
.ORIG
.END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_err());
}

#[test]
fn error_if_end_has_operands() {
    let program = "
.ORIG   x3000
.END    x3000
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_err());
}

#[test]
fn error_if_orig_given_non_literal() {
    let program = "
.ORIG STARTADDR
.END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_err());
}
