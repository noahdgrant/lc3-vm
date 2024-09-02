use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

use vm::VirtualMachine;

// TODO: add different file formats that can be passed to the Vm
// - default is a binary file (.obj)
// - also allow a text file with hex values to be passed in

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <file.obj>", args[0]);
    }

    let file_path = &args[1];
    println!("Loading {file_path}");

    let contents = read_file(file_path).expect("Error reading file");
    for line in &contents {
        println!("{:?}", line);
    }

    let mut vm = VirtualMachine::new();

    let mut base_address = 0x3000; // TODO: Grab this from the first line of the asm file
    for line in &contents {
        vm.memory.write(base_address, *line);
        base_address += 1;
    }

    vm.run();
}

fn read_file(filename: &str) -> std::io::Result<Vec<u16>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut data = Vec::new();
    for chunk in buffer.chunks_exact(2) {
        let value = u16::from_le_bytes(chunk.try_into().unwrap());
        data.push(value);
    }
    Ok(data)
}
