use std::fs::File;
use rfd::FileDialog;
use std::io::prelude::*;
use colored::Colorize;
use std::env;
use std::path::PathBuf;

mod assembler;
mod instruction;
mod argument_parser;
mod replacement;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_path: PathBuf = Default::default();
    get_inputs(args, &mut input_path);
    let display = input_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&input_path) {
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
    let binding = input_path.to_str().unwrap().to_string().clone();
    let characters = binding.chars();
    let last_accepted_character = characters.clone().count() - 4;
    let store_path = characters.collect::<Vec<char>>()[0..last_accepted_character].iter().collect::<String>() + ".bin";
    let binary_file = File::create(store_path);
    _ = binary_file.unwrap().write_all(&binary);
}

fn get_inputs(args: Vec<String>, input_path: &mut PathBuf) {
    if args.contains(&"-v".to_string()){
        // Visual setup
        let dialog = FileDialog::new();
        if let Some(path) = dialog.pick_file(){
            *input_path = path;
        }
        return;
    }
    // Command line args setup
    *input_path = PathBuf::from(get_parameter("-f", args.clone()));
    /*if !input_path.(".asm"){
        let error = "File name must end with .asm".red().to_string();
        panic!("{}", error);
    }*/


}

fn get_parameter(name: &str, args: Vec<String>) -> String {
    let mut parameter_position: Option<usize> = None;
    for i in 0..args.len() - 1{
        if let Some(argument) = args.get(i) {
            if argument == name{
                parameter_position = Some(i + 1);
            }
        }
    }

    if parameter_position.is_none() {
        let error = format!("Parameter {} expected but not found", name).red().to_string();
        panic!("{}", error);
    }

    if let Some(result) = args.iter().nth(parameter_position.unwrap()) { return result.to_string(); }
    let error = format!("Parameter {} has an expected value.", name).red().to_string();
    panic!("{}", error);
}

#[allow(dead_code)]
fn get_parameter_uint(name: &str, args: Vec<String>) -> u64 {
    let parameter_value = get_parameter(name, args);

    if let Some(parameter_value_u64) = parameter_value.parse::<u64>().ok(){ return parameter_value_u64; }
    let error = format!("Parameter {} should be a positive number but {} was found instead.", name, parameter_value).red().to_string();
    panic!("{}", error);
}
