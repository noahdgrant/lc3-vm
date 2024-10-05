use std::error::Error;
use std::fmt;
use std::ops::Index;

use std::str::FromStr;

use vm::{Opcode, Register};

// TODO: Add logging system
// TODO: write cli tool (look at how gcc works)
// TODO: be able to convert between errors so I can remove unwraps() and expects()
// TODO: implement the Display trait for my types
// TODO: the first instruction shoule be the ORIG number

// TODO: Add line and column number to error message
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssemblerError {
    EndUsage(String),
    HaltUsage(String),
    InvalidBinary(String),
    InvalidDecimal(String),
    InvalidHex(String),
    InvalidNumber(String),
    InvalidSymbol(String),
    OrigUsage(String),
    UnknownPseudoOp(String),
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerError::EndUsage(s) => write!(f, "{}", s),
            AssemblerError::HaltUsage(s) => write!(f, "{}", s),
            AssemblerError::InvalidBinary(s) => write!(f, "Invalid binary number: {}", s),
            AssemblerError::InvalidDecimal(s) => write!(f, "Invalid decimal number: {}", s),
            AssemblerError::InvalidHex(s) => write!(f, "Invalid hex number: {}", s),
            AssemblerError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            AssemblerError::InvalidSymbol(s) => write!(f, "Invalid symbol {}", s),
            AssemblerError::OrigUsage(s) => write!(f, "{}", s),
            AssemblerError::UnknownPseudoOp(s) => write!(f, "Unkown Pseudo-op: {}", s),
        }
    }
}

impl Error for AssemblerError {}

const INSTRUCTIONS: [&str; 2] = ["ADD", "BRnzp"];
const DIRECTIVES: [&str; 3] = [".ORIG", ".END", "HALT"];
const PROTECTED_WORDS: [&str; 2] = ["ORIG", "END"];

pub fn assemble(program: String) -> Result<Vec<u16>, AssemblerError> {
    let mut output: Vec<u16> = Vec::new();
    let mut line_number = 0;
    let mut first_line = true;
    let mut halt_found = false;
    let mut end_found = false;
    let mut location_counter = 0;
    let mut symbol_table: Vec<(&str, u16)> = Vec::new();

    // FIRST PASS
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
            return Err(AssemblerError::EndUsage(format!("{} after .END", line)));
        }

        // Get code before comment if there is a comment at the end of the line
        let code = line.split(';').next().unwrap_or(line).trim();
        let parts: Vec<&str> = code
            .split_whitespace()
            .map(|s| s.trim_end_matches(","))
            .collect();
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
                println!("Starting location: {location_counter:x}");
                continue;
            } else {
                return Err(AssemblerError::OrigUsage(
                    "The first line must be .ORIG".to_string(),
                ));
            }
        }

        // STEP 3: Check for label
        if PROTECTED_WORDS.contains(first_part) {
            return Err(AssemblerError::InvalidSymbol(first_part.to_string()));
        } else if INSTRUCTIONS.contains(first_part) || DIRECTIVES.contains(first_part) {
            if *first_part == ".ORIG" {
                return Err(AssemblerError::OrigUsage(
                    "Can only have one .ORIG".to_string(),
                ));
            } else if *first_part == ".END" {
                end_found = true;
                if parts.len() != 1 {
                    return Err(AssemblerError::EndUsage(
                        ".END usage: .END Given: {line}".to_string(),
                    ));
                }
            } else if *first_part == "HALT" {
                if parts.len() != 1 {
                    return Err(AssemblerError::HaltUsage(
                        "HALT usage: <optional symbol> HALT Given: {line}".to_string(),
                    ));
                }
                halt_found = true;
            } else {
                location_counter += 1;
            }
        } else {
            if parts.contains(&"HALT") {
                if *parts.last().unwrap() != "HALT" || parts.len() != 2 {
                    return Err(AssemblerError::HaltUsage(
                        "HALT usage: <optional symbol> HALT Given: {line}".to_string(),
                    ));
                }
                halt_found = true;
            }
            symbol_table.push((first_part, location_counter));
            location_counter += 1;
        }

        println!("Line #{}: {:?}", line_number, parts);
        println!("Location counter: 0x{:x}", location_counter);
        print!("Symbol table: ");
        for symbol in &symbol_table {
            print!("({}, 0x{:x}) ", symbol.0, symbol.1);
        }
        println!("\n");
    }

    if !end_found {
        return Err(AssemblerError::EndUsage("Missing .END".to_string()));
    }

    if !halt_found {
        return Err(AssemblerError::HaltUsage("Missing HALT".to_string()));
    }

    // SECOND PASS
    first_line = true;
    for line in program.lines() {
        line_number += 1;

        // TODO: create a sanitize_line() function so this stuff isn't repeated twice
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

        // Get code before comment if there is a comment at the end of the line
        let code = line.split(';').next().unwrap_or(line).trim();
        let parts: Vec<&str> = code
            .split_whitespace()
            .map(|s| s.trim_end_matches(","))
            .collect();
        let first_part = parts.first().expect("Empty line");

        // STEP 2: Encode instructions
        // TODO: This check can be removed and the encoding part can be moved
        // to the encode_directive() funciton since we already check if the .ORIG
        // is good in the first pass. There is no need to do this again
        if first_line {
            first_line = false;
            let program_start = encode_numeric(parts.get(1).expect("Checked .ORIG in first pass"))?;
            output.push(program_start);
        }

        if DIRECTIVES.contains(first_part) {
            if *first_part == "HALT" {
                output.push(0xf025);
            }
            //let encoded_line = encode_directive(parts, &symbol_table)?;
            //output.push(encoded_line);
        } else {
            let encoded_line = encode_instruction(parts, &symbol_table)?;
            output.push(encoded_line);
        }
    }

    Ok(output)
}

fn encode_instruction(
    parts: Vec<&str>,
    symbol_table: &Vec<(&str, u16)>,
) -> Result<u16, AssemblerError> {
    let mut instruction: u16 = 0;
    let mut symbol: Option<(&str, u16)> = None;

    let first_part = parts.first().expect("Should exist");
    for (sym, val) in symbol_table {
        if first_part == sym {
            symbol = Some((sym, *val));
            break;
        }
    }

    let opcode = match symbol {
        None => Opcode::from_str(first_part).unwrap(),
        Some((_, _)) => Opcode::from_str(parts.index(1)).unwrap(),
    };

    println!("Opcode: {:?}", opcode);

    let operands = match symbol {
        None => &parts[1..],
        Some((_, _)) => &parts[2..],
    };

    println!("Operands: {:?}", operands);

    match opcode {
        Opcode::ADD => {
            assert!(operands.len() == 3);
            let dr = Register::from_str(operands[0]).unwrap();
            let sr1 = Register::from_str(operands[1]).unwrap();
            println!("dr: {}", dr as u16);
            println!("sr1: {}", sr1 as u16);

            instruction += ((dr as u16) << 9) + ((sr1 as u16) << 6);

            match operands[2]
                .chars()
                .next()
                .expect("Missing ADD instruction argument")
            {
                'x' | 'b' | '#' => {
                    let imm5_flag = 1;
                    let imm5 = encode_numeric(operands[2]).unwrap();
                    instruction += (imm5_flag << 5) + imm5;
                    println!("imm5 flag: {}", imm5_flag);
                    println!("imm5: {}", imm5);
                }
                'R' => {
                    let sr2 = Register::from_str(operands[2]).unwrap();
                    instruction += sr2 as u16;
                    println!("sr2: {}", sr2 as u16);
                }
                _ => {
                    let mut found = false;
                    for (symbol, value) in symbol_table {
                        if operands[2] == *symbol {
                            instruction += value;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return Err(AssemblerError::InvalidSymbol(operands[2].to_string()));
                    }
                }
            }
        }
        _ => unimplemented!("Unimplemented opcode {:?}", opcode),
    }

    instruction += (opcode as u16) << 12;

    println!("Encoded line: 0x{instruction:x}");

    Ok(instruction)
}

//fn encode_directive(
//    parts: Vec<&str>,
//    symbol_table: &Vec<(&str, u16)>,
//) -> Result<u16, AssemblerError> {
//    if *first_part == ".ORIG" {
//        if orig_found {
//            return Err(AssemblerError::OrigUsage(
//                "Can only have one .ORIG".to_string(),
//            ));
//        } else {
//            orig_found = true;
//        }
//    } else if *first_part == ".END" {
//        end_found = true;
//        if parts.len() != 1 {
//            return Err(AssemblerError::EndUsage(
//                ".END usage: .END Given: {line}".to_string(),
//            ));
//        }
//    } else {
//    }
//}

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
