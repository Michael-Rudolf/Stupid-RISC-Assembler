use std::fs::File;
use rfd::FileDialog;
use std::io::prelude::*;

mod assembler;
mod instruction;

fn main() {
    let path = FileDialog::new().pick_file().unwrap();

    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(reason) => panic!("Couldn't open {}: {}", display, reason),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut asm = String::new();
    match file.read_to_string(&mut asm) {
        Err(reason) => panic!("Couldn't read {}: {}", display, reason),
        Ok(_) => {/*Do nothing*/},
    }

    let mut assembler = assembler::Assembler::new(asm);
    assembler.assemble();

    let binary: Vec<u8> = assembler.output;

    let store_path = path.to_str().unwrap().to_string().split('.').nth(0).unwrap().to_string() + ".bin";
    if let binary_file = File::create(store_path){
        _ = binary_file.unwrap().write_all(&binary);
    }
}
