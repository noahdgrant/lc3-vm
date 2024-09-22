use std::error::Error;
use std::fmt;

// TODO: Add line and column number to error message
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssemblerError {
    InvalidBinary(String),
    InvalidDecimal(String),
    InvalidHex(String),
    InvalidNumber(String),
    UnknownPseudoOp(String),
    OrigUsage(String),
    EndUsage(String),
    InvalidSymbol(String),
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerError::InvalidBinary(s) => write!(f, "Invalid binary number: {}", s),
            AssemblerError::InvalidDecimal(s) => write!(f, "Invalid decimal number: {}", s),
            AssemblerError::InvalidHex(s) => write!(f, "Invalid hex number: {}", s),
            AssemblerError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            AssemblerError::UnknownPseudoOp(s) => write!(f, "Unkown Pseudo-op: {}", s),
            AssemblerError::OrigUsage(s) => write!(f, "{}", s),
            AssemblerError::EndUsage(s) => write!(f, "{}", s),
            AssemblerError::InvalidSymbol(s) => write!(f, "Invalid symbol {}", s),
        }
    }
}

impl Error for AssemblerError {}

pub fn assemble(program: String) -> Result<Vec<u16>, AssemblerError> {
    let output: Vec<u16> = Vec::new();
    let mut line_number = 0;
    let mut first_line = true;
    let mut orig_found = false;
    let mut end_found = false;
    let mut _halt_found = false;
    let mut location_counter = 0;
    let mut symbol_table: Vec<(&str, u16)> = Vec::new();
    let instructions = ["ADD", "BRnzp"];
    let directives = [".ORIG", ".END"];
    let protected_words = ["ORIG", "END"];

    // First pass
    // Label | Opcode | Operand(s) | Comment
    for line in program.lines() {
        line_number += 1;

        // STEP 1: Sanitize line
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

        if end_found {
            return Err(AssemblerError::EndUsage(
                "{line.to_string()} after .END".to_string(),
            ));
        }

        // Get code before comment if there is a comment at the end of the line
        let code = line.split(';').next().unwrap_or(line).trim();
        let parts: Vec<_> = code.split_whitespace().collect();
        let first_part = parts.first().expect("Empty line");

        // STEP 2: Initialize location counter to starting address
        if first_line {
            first_line = false;
            // TODO: Create a parse_directive() function and move this to it
            if *first_part == ".ORIG" {
                if parts.len() != 2 {
                    return Err(AssemblerError::OrigUsage(
                        ".ORIG usage: .ORIG <numeric> Given: {first_line}".to_string(),
                    ));
                }
                location_counter = encode_numeric(parts.get(1).expect("Missing numeric"))?;
            } else {
                return Err(AssemblerError::OrigUsage(
                    "The first line must be .ORIG".to_string(),
                ));
            }
        }

        // STEP 3: Check for label
        if instructions.contains(first_part) || directives.contains(first_part) {
            if *first_part == ".ORIG" {
                if orig_found {
                    return Err(AssemblerError::OrigUsage(
                        "Can only have one .ORIG".to_string(),
                    ));
                } else {
                    orig_found = true;
                }
            } else if *first_part == ".END" {
                end_found = true;
                if parts.len() != 1 {
                    return Err(AssemblerError::EndUsage(
                        ".END usage: .END Given: {line}".to_string(),
                    ));
                }
            } else if protected_words.contains(first_part) {
                return Err(AssemblerError::InvalidSymbol(first_part.to_string()));
            } else {
                location_counter += 1;
            }
        } else {
            // TODO: check that the symbol is not a protected word
            symbol_table.push((first_part, location_counter));
        }
        println!("Line #{}: {:?}", line_number, parts);
        println!("Location counter: {}", location_counter);
        println!("Symbol table: {:?}", symbol_table);
    }

    if !end_found {
        return Err(AssemblerError::EndUsage("Missing .END".to_string()));
    }

    //    for line in program.lines() {
    //        line_number += 1;
    //
    //        // Remove whitespace
    //        let line = line.trim();
    //
    //        // Ignore empty lines
    //        if line.is_empty() {
    //            continue;
    //        }
    //
    //        // Ignore comments
    //        if line.starts_with(";") {
    //            continue;
    //        }
    //
    //        if end_found {
    //            return Err(AssemblerError::EndUsage(
    //                "{line.to_string()} after .END".to_string(),
    //            ));
    //        }
    //
    //        // Get code before comment if there is a comment at the end of the line
    //        let code = line.split(';').next().unwrap_or(line).trim();
    //
    //        let parts: Vec<_> = code.split_whitespace().collect();
    //        let line_parts: Vec<String> = parts.iter().map(|part| part.replace(",", "")).collect();
    //        let first_part = line_parts.first().expect("Should be a symbol").as_str();
    //
    //        if first_line {
    //            if first_part == ".ORIG" {
    //                first_line = false;
    //            } else {
    //                return Err(AssemblerError::OrigUsage(
    //                    "The first line must be .ORIG".to_string(),
    //                ));
    //            }
    //        }
    //
    //        // TODO: first_part won't work with labels
    //        if first_part.starts_with(".") {
    //            match first_part {
    //                ".ORIG" => {
    //                    if orig_found {
    //                        return Err(AssemblerError::OrigUsage(
    //                            "Can only have one .ORIG".to_string(),
    //                        ));
    //                    } else {
    //                        orig_found = true;
    //                        if line_parts.len() != 2 {
    //                            return Err(AssemblerError::OrigUsage(
    //                                ".ORIG usage: .ORIG <numeric> Given: {first_line}".to_string(),
    //                            ));
    //                        }
    //                        location_counter =
    //                            encode_numeric(line_parts.get(1).expect("Missing numeric"))?;
    //                    }
    //                }
    //                ".END" => {
    //                    end_found = true;
    //                }
    //                ".FILL" | ".BLKW" | ".STRINGZ" => {
    //                    todo!("Add missing label error")
    //                }
    //                _ => return Err(AssemblerError::UnknownPseudoOp(first_part.to_string())),
    //            }
    //        }
    //
    //        /*
    //            1. check if first part of the line is an opcode
    //                - if yes, check if there is a symbol in any of the operands
    //                    - if yes, add it to symbol table if not already in it
    //                - if no, add to symbol table if not already in it
    //        */
    //
    //        //if !is_opcode(first_part) {
    //        //    symbol_table.push((first_part, location_counter));
    //        //}
    //    }
    //
    //    if !end_found {
    //        return Err(AssemblerError::EndUsage("Missing .END".to_string()));
    //    }

    Ok(output)
}

// TODO: add unit tests for this function
// - valid and invalid hex (x3 vs X3)
// - valid and invalid binary (b0111 vs B0111)
// - decimal (#3)
fn encode_numeric(s: &str) -> Result<u16, AssemblerError> {
    let symbol = s.chars().next().unwrap();

    // TODO: figure out better way to get the rest of the string after the first char
    let mut chars = s.chars();
    chars.next().unwrap();
    let number = chars.as_str();

    match symbol {
        'b' => u16::from_str_radix(number, 2)
            .map_err(|_| AssemblerError::InvalidBinary(number.to_string())),
        '#' => number
            .parse::<u16>()
            .map_err(|_| AssemblerError::InvalidDecimal(number.to_string())),
        'x' => u16::from_str_radix(number, 16)
            .map_err(|_| AssemblerError::InvalidHex(number.to_string())),
        _ => Err(AssemblerError::InvalidNumber(number.to_string())),
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
