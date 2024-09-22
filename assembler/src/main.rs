// TODO: figure out how the VM knows where to load the progam based on the .ORIG from the .asm file
// maybe insert a line that reads something like LD PC,x3000 that tell the computer where to start
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::exit;

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
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}

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
