use assembler::assemble;

#[test]
fn simplest_program() {
    let program = "
.ORIG   x3000
        HALT
.END
";
    let binary = vec![0x3000, 0xf025];
    let output = assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn undefined_symbol() {
    // TODO: Is this even valid?
    let _program = "
.ORIG   x3000
        HALT
        ADD R1, R2, SYMBOL
.END
";
}

#[test]
fn no_spaces_between_operands() {
    // TODO: valid
    let _program = "
.ORIG   x3000
        HALT
        ADD R1,R2,R3
.END
";
}

#[test]
fn commas_between_operands() {
    // TODO: valid
    let _program = "
.ORIG   x3000
        HALT
        ADD R1 R2 R3
.END
";
}
