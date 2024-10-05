#[test]
fn ignore_before_orig() {
    let program = "
; ignore before .ORIG
.ORIG   x3000
HALT
.END
";
    let binary = vec![0x3000, 0xf025];
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn ignore_end_of_line_with_space() {
    let program = "
.ORIG   x3000 ; ignore at end with space
HALT
.END
";
    let binary = vec![0x3000, 0xf025];
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn ignore_end_of_line_without_space() {
    let program = "
.ORIG   x3000
HALT
.END; ignore at the end with no space
";
    let binary = vec![0x3000, 0xf025];
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn ignore_after_orig_before_end() {
    let program = "
.ORIG   x3000
; ignore after orig and before end
HALT
.END
";
    let binary = vec![0x3000, 0xf025];
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}

#[test]
fn ignore_after_end() {
    let program = "
.ORIG   x3000
HALT
.END
; ignore after .END
";
    let binary = vec![0x3000, 0xf025];
    let output = assembler::assemble(program.to_string());
    assert!(output.is_ok_and(|b| b == binary));
}
