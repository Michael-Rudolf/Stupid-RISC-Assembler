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

        let lines_without_comments: Vec<String> = argument_parser::ArgumentParser::remove_comments(code_seperated_by_lines.map(|x| x.chars().collect()).collect());

        // Go throw every line and set all the values defined in the file.
        let mut lines_except_values: Vec<String> = argument_parser::ArgumentParser::remove_declaration_lines(lines_without_comments.clone());
        let mut replacements: Vec<Replacement> = argument_parser::ArgumentParser::get_replacements_from_code(lines_without_comments);
        let start_function_meta_data = argument_parser::ArgumentParser::get_start_function(replacements.to_vec());
        let start_function_start: u16 = start_function_meta_data.start; // In lines
        let mut start_function_end: Option<u16> = start_function_meta_data.end; // In lines

        // Set the start function end to the file end in case the start function is the last function (in which case start_function_end hasn't been set).
        if start_function_end.is_none(){
            start_function_end = Some(lines_except_values.len() as u16);
        }
        // Swap the lines now
        let mut indices: Vec<usize> = vec![];
        // Make a list of the items to move
        for i in start_function_start..start_function_end.unwrap() {
            indices.push(i as usize);
        }

        // Move them
        Self::move_items_by_index(&mut lines_except_values, &indices, 0);


        // Replace the positions of the functions
        replacements = argument_parser::ArgumentParser::move_replacements_after_end_function(start_function_end.unwrap() - start_function_start, replacements);
        // Convert function positions in lines to function positions in bytes
        argument_parser::ArgumentParser::function_lines_to_function_bytes(&mut replacements);
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

    fn move_items_by_index<T: Clone>(vec: &mut Vec<T>, indices: &Vec<usize>, target_index: usize) {
        // Collect elements by index while preserving order
        /*let extracted: Vec<T> = indices.iter().map(|&i| vec[i].clone()).collect();

        // Sort indices in descending order to remove them without shifting remaining indices
        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort_unstable_by(|a, b| b.cmp(a));

        // Remove elements at the given indices
        for i in sorted_indices {
            vec.remove(i);
        }

        // Insert extracted items at the target position
        let insert_pos = target_index.min(vec.len());
        vec.splice(insert_pos..insert_pos, extracted);*/
    }

}


/*
for line in lines_without_comments {
    let characters: Vec<char> = line.chars().collect();
    if let Some(&last_character) = characters.last(){
        if last_character == ':' {
            // This indicates a function afterward, therefore, store the current line number
            let name = characters[0..characters.len() - 1].iter().collect::<String>();
            replacements.push((name.clone(), (current_line * 3).to_string()));
            if start_function_start.is_some() && start_function_end.is_none(){
                start_function_end = Some(current_line);
            }
            if let Some(start_function_name_copy) = start_function_name.clone() {
                if start_function_name_copy.as_str() == name {
                    start_function_start = Some(current_line);
                }
            }
            if start_function_start.is_none(){
                // This function occurs before the start function, so it'll be moved later
                // Add it to the update list to move it
                update_list.push(replacements.len() - 1);
            }
            continue;
        } else if let Some(&first_character) = characters.first(){
            if first_character == '.' {
                let selected_chars = &characters[1..characters.len()];
                let selected_string = selected_chars.iter().collect::<String>();
                let selected_string_clone = selected_string.clone();
                let mut selected_string_split = selected_string_clone.split(' ');
                println!("{:?}", selected_string_split);

                let name = selected_string_split.next().unwrap();
                if let Some(replacement) = selected_string_split.next() {
                    replacements.push((name.to_string(), format!("{}", replacement.to_string())));

                    if start_function_name.is_none() && name == "global_start"{
                        start_function_name = Some(replacement.to_string());
                    }
                }else {
                    let error = format!("Identifier {} at line {} misses an argument.", name, current_line).red().to_string();
                    panic!("{}", error);
                }
                continue;
            }
        }
        current_line += 1;
        lines_except_values.push(line.to_string());
    }
}*/
