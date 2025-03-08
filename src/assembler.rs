use instruction::Instruction;
use crate::{argument_parser, instruction};
use colored::Colorize;
use crate::replacement::Replacement;

pub struct Assembler {
    pub code: String,
    pub output: Vec<u8>,
}

impl Assembler {
    pub fn new(code: String) -> Assembler {
        Assembler{code, output: Vec::new()}
    }
    pub fn assemble(&mut self) {
        // Remove all comments and empty lines
        let code_seperated_by_lines = self.code.lines();
        let lines = code_seperated_by_lines.clone().map(|x| x.chars().collect()).collect();
        let lines_2 = code_seperated_by_lines.map(|x| x.chars().collect()).collect();

        // Go throw every line and set all the values defined in the file.
        let mut lines_except_values: Vec<String> = argument_parser::ArgumentParser::remove_declaration_lines(lines);
        let replacements: Vec<Replacement> = argument_parser::ArgumentParser::get_replacements_from_code(lines_2);
        argument_parser::ArgumentParser::apply_replacements_in_code(replacements, &mut lines_except_values);

        let mut binary: Vec<u8> = vec![];
        let mut i: u32 = 0;
        for line in lines_except_values.clone() {
            i += 1;
            if let Some(instruction) = Instruction::from_string(line.clone(), i){
                let mut binary_instruction = instruction.to_vec();
                binary.append(&mut binary_instruction);
            }else{
                let error = format!("Couldn't decode line {} at line {}.", line.clone().to_string(), i).red().to_string();
                panic!("{}", error);
            }
        }
        self.output = binary;
    }

}

