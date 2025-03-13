use colored::Colorize;
use crate::argument_parser::ArgumentParser;

#[derive(Copy)]
pub struct Instruction{
    task: u8,
    required_arguments: u8,
    arg0: u8,
    arg1: u8,
}

impl Instruction {
    pub fn to_vec(&self) -> Vec<u8>{
        let mut result: Vec<u8> = vec![self.task];

        match self.required_arguments{
            0 => { result },
            1 => { result.push(self.arg0); result },
            2 => { result.push(self.arg0); result.push(self.arg1); result },
            _ => { panic!("{}", "Amount of arguments not supported".red()) },
        }
    }
}

// Task encoding after bits:
// Extension (1 = yes)
// If no extension:
// 1 means internal only
// 0 means external ops (RAM load, etc.)
// 010: ALU
// 011: Other internal (mov, etc)
// 000: Memory OP
// 001: Reserved for future applications

//15/256 used = 5.8%
pub const STANDARD_OUTPUT_WRITE_INSTRUCTION: u8 = 0b0000_0001;
pub const STANDARD_OUTPUT_CLEAR_INSTRUCTION: u8 = 0b0000_0010;
pub const ADD_INSTRUCTION: u8 = 0b0100_0000;
pub const SUB_INSTRUCTION: u8 = 0b0100_0001;
pub const MUL_INSTRUCTION: u8 = 0b0100_0010;
pub const DIV_INSTRUCTION: u8 = 0b0100_0011;
pub const MOD_INSTRUCTION: u8 = 0b0100_0100;
pub const HALT_INSTRUCTION: u8 = 0b0110_0000;
pub const MOVE_INSTRUCTION: u8 = 0b0110_0001;
pub const LOAD_IMMEDIATE_TO_INTERNAL_INSTRUCTION: u8 = 0b0110_1010;
pub const PUSH_BYTE_INSTRUCTION: u8 = 0b0110_0101;
pub const POP_BYTE_INSTRUCTION: u8 = 0b0110_1100;
pub const JUMP_INSTRUCTION: u8 = 0b0110_0010;
// Jumps to arg2 if arg1 is 0
pub const JUMP_ZERO_INSTRUCTION: u8 = 0b0110_0011;
pub const LOAD_BYTE_INSTRUCTION: u8 = 0b0110_0100;
pub const STORE_BYTE_INSTRUCTION: u8 = 0b0111_0100;

#[allow(dead_code)]
pub const RESERVED_REGISTER: u8 = 11+128;
#[allow(dead_code)]
pub const FLAGS_REGISTER: u8 = 12 + 128;
#[allow(dead_code)]
pub const EXEC_PTR_REGISTER: u8 = 15 + 128;
pub const FRAME_PTR_REGISTER: u8 = 13 + 128;
#[allow(dead_code)]
pub const EMPTY_ARGUMENT: u8 = 0;
impl Instruction{
    pub fn new(task: u8, required_arguments: u8, arg0: u8, arg1: u8) -> Instruction{
        Instruction{task, required_arguments, arg0, arg1}
    }

    pub fn bytes_required_by_instruction_by_name(named: String) -> u8{
        match named.as_str(){
            "halt" | "soc" => 1,
            "jmp" | "pushb" | "popb" | "sow" => 2,
            "add" | "sub" | "mul" | "div" | "mod" | "jmpz" | "mov" | "ldb" | "stb" | "inc" | "dec" => 3,
            _ => 0
        }
    }

    pub fn from_string(instruction: String, current_line: u32) -> Option<Instruction> {
        let splitted = ArgumentParser::line_to_argument_parts(instruction.as_str());

        if splitted.is_empty() { return None }
        let task_string = splitted[0].to_ascii_lowercase();

        match splitted.len() {
            1 => {
                match task_string.as_ref() {
                    "halt" => Some(Instruction::new(HALT_INSTRUCTION, 0,0, 0)),
                    "soc" => Some(Instruction::new(STANDARD_OUTPUT_CLEAR_INSTRUCTION, 0, 0, 0)),
                    _ => None
                }
            },
            2 => {
                let arg1 = ArgumentParser::argument_to_8_bit_binary(splitted[1].as_str(), current_line as i32);

                match task_string.as_ref() {
                    "jmp" => Some(Instruction::new(JUMP_INSTRUCTION, 1, arg1, 0)),
                    "pushb" => Some(Instruction::new(PUSH_BYTE_INSTRUCTION, 1, arg1, 0)),
                    "popb" => Some(Instruction::new(POP_BYTE_INSTRUCTION, 1, arg1, 0)),
                    "inc" => Some(Instruction::new(ADD_INSTRUCTION, 2, arg1, 1)),
                    "dec" => Some(Instruction::new(SUB_INSTRUCTION, 2, arg1, 1)),
                    "sow" => Some(Instruction::new(STANDARD_OUTPUT_WRITE_INSTRUCTION, 1, arg1, 0)),
                    _ => None
                }
            }
            3 => {
                let arg1 = ArgumentParser::argument_to_8_bit_binary(splitted[1].as_str(), current_line as i32);
                let arg2 = ArgumentParser::argument_to_8_bit_binary(splitted[2].as_str(), current_line as i32);

                match task_string.as_ref() {
                    "add" => Some(Instruction::new(ADD_INSTRUCTION, 2, arg1, arg2)),
                    "sub" => Some(Instruction::new(SUB_INSTRUCTION, 2, arg1, arg2)),
                    "mul" => Some(Instruction::new(MUL_INSTRUCTION, 2, arg1, arg2)),
                    "div" => Some(Instruction::new(DIV_INSTRUCTION, 2, arg1, arg2)),
                    "mod" => Some(Instruction::new(MOD_INSTRUCTION, 2, arg1, arg2)),
                    "jmpz" => Some(Instruction::new(JUMP_ZERO_INSTRUCTION, 2, arg1, arg2)),
                    "mov" => Some(Instruction::new(MOVE_INSTRUCTION, 2, arg1, arg2)),
                    "ldb" => Some(Instruction::new(LOAD_BYTE_INSTRUCTION, 2, arg1, arg2)),
                    "stb" => Some(Instruction::new(STORE_BYTE_INSTRUCTION, 2, arg1, arg2)),
                    _ => None
                }
            }
            _ => None,
        }
    }
}

impl Clone for Instruction{
    fn clone(&self) -> Instruction{
        Instruction{task: self.task, required_arguments: self.required_arguments, arg0: self.arg0, arg1: self.arg1}
    }
}

