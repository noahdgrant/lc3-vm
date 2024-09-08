#[test]
fn ignore_before_orig() {
    let program = "
; ignore before .ORIG
.ORIG   x3000
.END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|binary| binary.is_empty()));
}

#[test]
fn ignore_end_of_line_with_space() {
    let program = "
.ORIG   x3000 ; ignore at end with space
.END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|binary| binary.is_empty()));
}

#[test]
fn ignore_end_of_line_without_space() {
    let program = "
.ORIG   x3000
.END; ignore at the end with no space
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|binary| binary.is_empty()));
}

#[test]
fn ignore_after_orig_before_end() {
    let program = "
.ORIG   x3000
; ignore after orig and before end
.END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|binary| binary.is_empty()));
}

#[test]
fn ignore_after_end() {
    let program = "
.ORIG   x3000
.END
; ignore after .END
";
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|binary| binary.is_empty()));
}
