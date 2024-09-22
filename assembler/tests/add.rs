#[test]
fn add_literal_to_register() {
    let _program = "
.ORIG   x3000
ADD R0,R0,x3
HALT
.END
";
}

#[test]
fn add_register_to_register() {
    let _program = "
.ORIG   x3000
ADD R0, R0, x3
ADD R1, R1, x4
ADD R2, R0, R1
HALT
.END
";
}
