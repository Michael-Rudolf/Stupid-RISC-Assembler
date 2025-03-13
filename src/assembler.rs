use instruction::Instruction;
use crate::{instruction};
use colored::Colorize;
use crate::argument_parser::ArgumentParser;
use crate::utility::replacement::Replacement;

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
        let sections = ArgumentParser::split_sections(lines);
        let data_section = sections.0;
        let text_section = sections.1;
        let data_parsed = ArgumentParser::compile_data_section(data_section);
        let data_replacements = data_parsed.1;
        let data_bytes = data_parsed.0;
        let mut lines_except_values: Vec<String> = ArgumentParser::remove_declaration_lines(text_section.clone());
        let mut replacements: Vec<Replacement> = ArgumentParser::get_replacements_from_code(text_section.clone());
        let data_offset = replacements[replacements.iter().position(|x| x.get_name() == "data_offset").unwrap()].get_value().parse::<u32>().unwrap();

        for replacement in data_replacements{
            replacements.push(Replacement::new(replacement.get_name(), (replacement.get_value().parse::<u32>().unwrap() + data_offset).to_string(), false));
        }


        ArgumentParser::apply_replacements_in_code(replacements, &mut lines_except_values);

        let mut binary: Vec<u8> = vec![];
        let mut i: u32 = 0;
        for line in lines_except_values.clone() {
            i += 1;
            if let Some(instruction) = Instruction::from_string(line.clone(), i){
                let mut binary_instruction = instruction.to_vec();
                binary.append(&mut binary_instruction);
            }else{
                let warning = format!("Couldn't decode line {} at line {}.", line.clone().to_string(), i).yellow().to_string();
                println!("{}", warning);
            }
        }
        binary.append(&mut data_bytes.clone());
        self.output = binary;
    }

}

