// Assembler
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;

use lc3_vm::{OpCode, Register};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <file.asm>", args[0]);
    }

    let file_path = &args[1];
    println!("Assembling {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("Contents:\n{contents}\n");

    let mut output: Vec<u16> = Vec::new();

    for line in contents.lines() {
        // Remove whitespace
        let line = line.trim();

        // Ignore empty lines
        if line.is_empty() {
            continue;
        }

        // Ignore comments
        if line.starts_with(";") {
            continue;
        }

        // TODO: Remove this and make it mandatory to start with .ORIG x3000
        // TODO: Remove this and make it mandatory to end with .END
        if line.starts_with(".") {
            continue;
        }

        println!("Line:\n{}\n", line);

        // Get code before comment if there is a comment at the end of the line
        let before_comment = line.split(';').next().unwrap_or(line).trim();

        let parts: Vec<_> = before_comment.split_whitespace().collect();
        let cleaned_parts: Vec<String> = parts.iter().map(|part| part.replace(",", "")).collect();
        println!("Parts:");
        for part in &cleaned_parts {
            println!("{}", part);
        }
        println!("\n");

        let instruction = encode_line(cleaned_parts);
        println!("Instruction: {}\n", instruction);
        output.push(instruction);
    }

    println!("Output file:");
    for instruction in &output {
        println!("{}", instruction);
    }

    // output assembled file to same directory as input but change the extension
    let _ = write_file(&output, "hello.bin").unwrap();
}

fn encode_line(parts: Vec<String>) -> u16 {
    let mut instruction: u16;
    let sr2: Option<Register>;

    let op_code = OpCode::from_str(parts[0].as_str()).unwrap();
    println!("Encoded line:");
    println!("op: {:?}", &op_code);

    match op_code {
        //OpCode::BR => ,
        OpCode::ADD => {
            assert!(parts.len() == 4);
            let dr = Register::from_str(parts[1].as_str()).unwrap();
            let sr1 = Register::from_str(parts[2].as_str()).unwrap();

            instruction = ((op_code as u16) << 12) + ((dr as u16) << 9) + ((sr1 as u16) << 6);

            // TODO: figure out a better way to do this
            if parts[3].starts_with(&['x', 'X', 'b', 'B', '#']) {
                let imm5_flag = 1;
                let imm5 = encode_numeric(&parts[3]).unwrap();
                instruction += (imm5_flag << 5) + imm5;

                println!("dr: {}", dr as u16);
                println!("sr1: {}", sr1 as u16);
                println!("imm5 flag: {}", imm5_flag);
                println!("imm5: {}", imm5);
            }
        }
        //OpCode::LD => ld(vm, instruction),
        //OpCode::ST => st(vm, instruction),
        //OpCode::JSR => jsr(vm, instruction),
        //OpCode::AND => and(vm, instruction),
        //OpCode::LDR => ldr(vm, instruction),
        //OpCode::STR => str(vm, instruction),
        //OpCode::RTI => rti(vm, instruction),
        //OpCode::NOT => not(vm, instruction),
        //OpCode::LDI => ldi(vm, instruction),
        //OpCode::STI => sti(vm, instruction),
        //OpCode::JMP => jmp(vm, instruction),
        //OpCode::RES => res(vm, instruction),
        //OpCode::LEA => lea(vm, instruction),
        OpCode::TRAP => {
            instruction = 0xFFFF;
        }
        _ => panic!("Op code not implemented {:?}", op_code),
    }

    instruction
}

#[derive(Debug, PartialEq, Eq)]
struct ParseNumbericError;

fn encode_numeric(s: &String) -> Result<u16, ParseNumbericError> {
    let symbol = s.chars().next().unwrap();

    // TODO: figure out better way to get the rest of the string after the first char
    let mut chars = s.chars();
    chars.next();
    let number = chars.as_str();

    println!("Encoding: {} {}", symbol, number);

    match symbol {
        'x' | 'X' => u16::from_str_radix(number, 16).map_err(|_| ParseNumbericError),
        'b' | 'B' => u16::from_str_radix(number, 2).map_err(|_| ParseNumbericError),
        '#' => u16::from_str_radix(number, 10).map_err(|_| ParseNumbericError),
        _ => Err(ParseNumbericError),
    }
}

fn write_file(data: &[u16], filename: &str) -> std::io::Result<()> {
    let mut buffer = Vec::with_capacity(data.len() * 2); // Allocate enough space for u16s
    for value in data {
        buffer.extend_from_slice(&value.to_le_bytes()); // Convert u16 to little-endian bytes
    }

    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&buffer)?;
    writer.flush()?;

    Ok(())
}
