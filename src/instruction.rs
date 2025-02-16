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
pub const ADD_INSTRUCTION: u8 = 0b0100_0000;
pub const SUB_INSTRUCTION: u8 = 0b0100_0001;
pub const MUL_INSTRUCTION: u8 = 0b0100_0010;
pub const DIV_INSTRUCTION: u8 = 0b0100_0011;
pub const MOD_INSTRUCTION: u8 = 0b0100_0100;
pub const HALT_INSTRUCTION: u8 = 0b0110_0000;
pub const MOVE_INSTRUCTION: u8 = 0b0110_0001;
pub const JUMP_INSTRUCTION: u8 = 0b0110_0010;
// Jumps to arg2 if arg1 is 0
pub const JUMP_ZERO_INSTRUCTION: u8 = 0b0110_0011;
pub const LOAD_BYTE_INSTRUCTION: u8 = 0b0110_0100;
pub const STORE_BYTE_INSTRUCTION: u8 = 0b0111_0100;



pub const FLAGS_REGISTER: u8 = 12 + 128;
pub const EXEC_PTR_REGISTER: u8 = 15 + 128;
pub const EMPTY_ARGUMENT: u8 = 0;
impl Instruction{
    pub fn new(task: u8, arg0: u8, arg1: u8) -> Instruction{
        Instruction{task, arg0, arg1}
    }

    pub fn to_binary(&self) -> Vec<u8>{
        [self.task, self.arg0, self.arg1].to_vec()
    }

    pub fn from_string(instruction: String) -> Option<Instruction>{



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
                let arg1_lower = splitted[1].to_ascii_lowercase();
                let arg1_string = arg1_lower.split_at(1);
                let mut arg1: u8 = arg1_string.1.parse().ok()?;
                if arg1_string.0 == "r" { arg1 |= 0b1000_0000; }

                match task_string.as_ref() {
                    "jmp" => Some(Instruction::new(JUMP_INSTRUCTION, EXEC_PTR_REGISTER, arg1)),
                    _ => None
                }
            }
            3 => {
                let arg1_lower = splitted[1].to_ascii_lowercase();
                // Store the modified string
                let arg2_lower = splitted[2].to_ascii_lowercase();

                let arg1_string = arg1_lower.split_at(1);
                let arg2_string = arg2_lower.split_at(1);

                let mut arg1: u8 = arg1_string.1.parse().ok()?;  // Use `ok()?` to avoid unwrap panic
                let mut arg2: u8 = arg2_string.1.parse().ok()?;


                if arg1_string.0 == "r" { arg1 |= 0b1000_0000; }
                if arg2_string.0 == "r" { arg2 |= 0b1000_0000; }

                match task_string.as_ref() {
                    "add" => Some(Instruction::new(ADD_INSTRUCTION, arg1, arg2)),
                    "sub" => Some(Instruction::new(SUB_INSTRUCTION, arg1, arg2)),
                    "mul" => Some(Instruction::new(MUL_INSTRUCTION, arg1, arg2)),
                    "div" => Some(Instruction::new(DIV_INSTRUCTION, arg1, arg2)),
                    "mod" => Some(Instruction::new(MOD_INSTRUCTION, arg1, arg2)),
                    "jmpz" => Some(Instruction::new(JUMP_ZERO_INSTRUCTION, arg1, arg2)),
                    "mov" => Some(Instruction::new(MOVE_INSTRUCTION, arg1, arg2)),
                    "ldb" => Some(Instruction::new(LOAD_BYTE_INSTRUCTION, arg1, arg2)),
                    "stb" => Some(Instruction::new(STORE_BYTE_INSTRUCTION, arg1, arg2)),
                    _ => None
                }
            }
            _ => None,
        }



    }


    // Returns number of arguments and the name of the instruction
    pub fn name_of_instruction(task: u8, arg_1: u8, arg_2: u8) -> Option<String>{
        let arg_1 = if arg_1 >= 128 { format!("R{:02} ", arg_1 - 128) } else { format!("N{:02} ", arg_1) };
        let arg_1_text = arg_1.as_str();
        let arg_2 = if arg_2 >= 128 { format!("R{:02}", arg_2 - 128) } else { format!("N{:02}", arg_2) };
        let arg_2_text = arg_2.as_ref();
        match task{
            ADD_INSTRUCTION => Some("add ".to_string() + arg_1_text + arg_2_text),
            SUB_INSTRUCTION => Some("sub ".to_string() + arg_1_text + arg_2_text),
            MUL_INSTRUCTION => Some("mul ".to_string() + arg_1_text + arg_2_text),
            DIV_INSTRUCTION => Some("div ".to_string() + arg_1_text + arg_2_text),
            MOD_INSTRUCTION => Some("mod ".to_string() + arg_1_text + arg_2_text),
            HALT_INSTRUCTION => Some("halt ".to_string()),
            MOVE_INSTRUCTION => Some("mov ".to_string() + arg_1_text + arg_2_text),
            JUMP_INSTRUCTION => Some("jmp ".to_string() + arg_2_text),
            JUMP_ZERO_INSTRUCTION => Some("jmpz ".to_string() + arg_1_text + arg_2_text),
            LOAD_BYTE_INSTRUCTION => Some("ldb ".to_string() + arg_1_text + arg_2_text),
            STORE_BYTE_INSTRUCTION => Some("stb ".to_string() + arg_1_text + arg_2_text),
            _ => None
        }
    }
}

impl Clone for Instruction{
    fn clone(&self) -> Instruction{
        Instruction{task: self.task, arg0: self.arg0, arg1: self.arg1}
    }
}

