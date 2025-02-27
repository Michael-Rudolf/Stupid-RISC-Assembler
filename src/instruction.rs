use crate::argument_parser::ArgumentParser;

#[derive(Copy)]
pub struct Instruction{
    task: u8,
    arg0: u8,
    arg1: u8,
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

//12/256 used = 4.3%
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
    pub fn new(task: u8, arg0: u8, arg1: u8) -> Instruction{
        Instruction{task, arg0, arg1}
    }

    pub fn to_binary(&self) -> Vec<u8>{
        [self.task, self.arg0, self.arg1].to_vec()
    }

    pub fn from_string(instruction: String, current_line: u32) -> Option<Instruction> {
        let splitted = instruction.split_whitespace().collect::<Vec<&str>>();

        let task_string = splitted[0].to_ascii_lowercase();

        match splitted.len() {
            1 => {
                match task_string.as_ref() {
                    "halt" => Some(Instruction::new(HALT_INSTRUCTION, FLAGS_REGISTER, EMPTY_ARGUMENT)),
                    _ => None
                }
            },
            2 => {
                let arg1 = ArgumentParser::argument_to_8_bit_binary(splitted[1], current_line);

                match task_string.as_ref() {
                    "jmp" => Some(Instruction::new(JUMP_INSTRUCTION, EXEC_PTR_REGISTER, arg1)),
                    "pushb" => Some(Instruction::new(PUSH_BYTE_INSTRUCTION, FRAME_PTR_REGISTER, arg1)),
                    "inc" => Some(Instruction::new(ADD_INSTRUCTION, arg1, 1)),
                    "dec" => Some(Instruction::new(SUB_INSTRUCTION, arg1, 1)),
                    _ => None
                }
            }
            3 => {
                let arg1 = ArgumentParser::argument_to_8_bit_binary(splitted[1], current_line);
                let arg2 = ArgumentParser::argument_to_8_bit_binary(splitted[2], current_line);

                match task_string.as_ref() {
                    "add" => Some(Instruction::new(ADD_INSTRUCTION, arg1, arg2)),
                    "sub" => Some(Instruction::new(SUB_INSTRUCTION, arg1, arg2)),
                    "mul" => Some(Instruction::new(MUL_INSTRUCTION, arg1, arg2)),
                    "div" => Some(Instruction::new(DIV_INSTRUCTION, arg1, arg2)),
                    "mod" => Some(Instruction::new(MOD_INSTRUCTION, arg1, arg2)),
                    "jmpz" => Some(Instruction::new(JUMP_ZERO_INSTRUCTION, arg1, arg2)),
                    "mov" => Some(Instruction::new(MOVE_INSTRUCTION, arg1, arg2)),
                    "ldb" => Some(Instruction::new(LOAD_BYTE_INSTRUCTION, arg1, arg2)),
                    "popb" => Some(Instruction::new(POP_BYTE_INSTRUCTION, arg1, 0)),
                    "stb" => Some(Instruction::new(STORE_BYTE_INSTRUCTION, arg1, arg2)),
                    _ => None
                }
            }
            _ => None,
        }
    }
}

impl Clone for Instruction{
    fn clone(&self) -> Instruction{
        Instruction{task: self.task, arg0: self.arg0, arg1: self.arg1}
    }
}

