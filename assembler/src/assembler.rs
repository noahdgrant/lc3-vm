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
        }
    }
}

impl Error for AssemblerError {}

pub fn assemble(program: String) -> Result<Vec<u16>, AssemblerError> {
    let output: Vec<u16> = Vec::new();
    let mut first_line = true;
    let mut orig_found = false;
    let mut end_found = false;
    let mut _memory_offset = 0;

    for line in program.lines() {
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
        let line_parts: Vec<String> = parts.iter().map(|part| part.replace(",", "")).collect();
        let first_part = line_parts.first().expect("Should be a symbol").as_str();

        if first_line {
            if first_part == ".ORIG" {
                first_line = false;
            } else {
                return Err(AssemblerError::OrigUsage(
                    "The first line must be .ORIG".to_string(),
                ));
            }
        }

        // Pseudo-ops
        if first_part.starts_with(".") {
            match first_part {
                ".ORIG" => {
                    if orig_found {
                        return Err(AssemblerError::OrigUsage(
                            "Can only have one .ORIG".to_string(),
                        ));
                    } else {
                        orig_found = true;
                        if line_parts.len() != 2 {
                            return Err(AssemblerError::OrigUsage(
                                "Usage: .ORIG <numeric> Given: {first_line}".to_string(),
                            ));
                        }
                        //memory_offset =
                        //    encode_numeric(line_parts.get(1).expect("Missing numeric"))?;
                    }
                }
                ".FILL" => {
                    todo!()
                }
                ".BLKW" => {
                    todo!()
                }
                ".STRINGZ" => {
                    todo!()
                }
                ".END" => {
                    end_found = true;
                }
                _ => return Err(AssemblerError::UnknownPseudoOp(first_part.to_string())),
            }
        }

        // Opcodes

        // Labels
    }

    if !end_found {
        return Err(AssemblerError::EndUsage("Missing .END".to_string()));
    }

    Ok(output)
}

// fn encode_numeric(s: &String) -> Result<u16, AssemblerError> {
//     let symbol = s.chars().next().unwrap();
//
//     // TODO: figure out better way to get the rest of the string after the first char
//     let mut chars = s.chars();
//     chars.next().unwrap();
//     let number = chars.as_str();
//
//     println!("Encoding: {} {}", symbol, number);
//
//     match symbol {
//         'b' | 'B' => u16::from_str_radix(number, 2)
//             .map_err(|_| AssemblerError::InvalidBinary(number.to_string())),
//         '#' => u16::from_str_radix(number, 10)
//             .map_err(|_| AssemblerError::InvalidDecimal(number.to_string())),
//         'x' | 'X' => u16::from_str_radix(number, 16)
//             .map_err(|_| AssemblerError::InvalidHex(number.to_string())),
//         _ => Err(AssemblerError::InvalidNumber(number.to_string())),
//     }
// }
