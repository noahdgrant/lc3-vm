// TODO: figure out how the VM knows where to load the progam based on the .ORIG from the .asm file
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <file.asm>", args[0]);
    }

    let file = &args[1];
    let program = fs::read_to_string(file).expect("Could not read file {file}");

    let output = assembler::assemble(program);
    match output {
        Ok(binary) => {
            // TODO: output assembled file to same directory as input file
            let _ = write_file(&binary, "test.obj");
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

// fn encode_line(parts: Vec<String>) -> u16 {
//     let mut instruction: u16 = 0;
//
//     let op_code = OpCode::from_str(parts[0].as_str()).unwrap();
//     println!("Encoded line:");
//     println!("op: {:?}", &op_code);
//
//     instruction += (op_code as u16) << 12;
//
//     match op_code {
//         //OpCode::BR => ,
//         OpCode::ADD => {
//             assert!(parts.len() == 4);
//             let dr = Register::from_str(parts[1].as_str()).unwrap();
//             let sr1 = Register::from_str(parts[2].as_str()).unwrap();
//             println!("dr: {}", dr as u16);
//             println!("sr1: {}", sr1 as u16);
//
//             instruction += ((dr as u16) << 9) + ((sr1 as u16) << 6);
//
//             // TODO: figure out a better way to do this
//             match parts[3]
//                 .chars()
//                 .nth(0)
//                 .expect("Missing ADD instruction argument")
//             {
//                 'x' | 'X' | 'b' | 'B' | '#' => {
//                     let imm5_flag = 1;
//                     let imm5 = encode_numeric(&parts[3]).unwrap();
//                     instruction += (imm5_flag << 5) + imm5;
//                     println!("imm5 flag: {}", imm5_flag);
//                     println!("imm5: {}", imm5);
//                 }
//                 'R' => {
//                     let sr2 = Register::from_str(parts[3].as_str()).unwrap();
//                     instruction += sr2 as u16;
//                     println!("sr2: {}", sr2 as u16);
//                 }
//                 _ => panic!("Unknown symbol {}", parts[3]),
//             }
//         }
//         //OpCode::LD => ld(vm, instruction),
//         //OpCode::ST => st(vm, instruction),
//         //OpCode::JSR => jsr(vm, instruction),
//         //OpCode::AND => and(vm, instruction),
//         //OpCode::LDR => ldr(vm, instruction),
//         //OpCode::STR => str(vm, instruction),
//         //OpCode::RTI => rti(vm, instruction),
//         //OpCode::NOT => not(vm, instruction),
//         //OpCode::LDI => ldi(vm, instruction),
//         //OpCode::STI => sti(vm, instruction),
//         //OpCode::JMP => jmp(vm, instruction),
//         //OpCode::RES => res(vm, instruction),
//         //OpCode::LEA => lea(vm, instruction),
//         OpCode::TRAP => {
//             instruction = 0xFFFF;
//         }
//         _ => panic!("Op code not implemented {:?}", op_code),
//     }
//
//     instruction
// }

fn write_file(data: &[u16], filename: &str) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    for value in data {
        buffer.extend_from_slice(&value.to_le_bytes());
    }

    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&buffer)?;
    writer.flush()?;

    Ok(())
}
