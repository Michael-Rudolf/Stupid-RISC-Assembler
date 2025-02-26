use colored::Colorize;
use crate::replacement::Replacement;

const CHARACTERS: [&str; 36] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

pub struct ArgumentParser {}

impl ArgumentParser {
    pub fn argument_to_8_bit_binary(argument: &str, line: u32) -> u8 {
        if let Some(arg_decimal_interpratation) = argument.parse::<i64>().ok() {
            // The number was a decimal number. Look if it is within range (0...127)
            if arg_decimal_interpratation < 0 {
                let error = format!("Argument {} in line {} should be within range 0...127 but is negative. Please define it in the data section instead.", argument, line).red().to_string();
                panic!("{}", error);
            }
            if arg_decimal_interpratation > 127 {
                let error = format!("Argument {} in line {} should be within range 127 but is too high. Please define it in the data section instead.", argument, line).red().to_string();
                panic!("{}", error);
            }
            // Fits constraints
            return arg_decimal_interpratation as u8;
        }
        if argument.len() < 2 {
            let error = format!("Argument {} in line {} is not decimal and needs type and value.", argument, line).red().to_string();
            panic!("{}", error);
        }
        // It's not plain ol' decimal, so get the encoding
        let splitted_argument = argument.split_at(1);
        let encoding = splitted_argument.0;
        let number = splitted_argument.1;
        match encoding.to_ascii_lowercase().as_str() {
            "x" => { return Self::convert(number, 16, 10).parse::<u8>().unwrap() },
            "o" => { return Self::convert(number, 8, 10).parse::<u8>().unwrap() },
            "b" => { return Self::convert(number, 2, 10).parse::<u8>().unwrap() },
            "r" => { return number.parse::<u8>().unwrap() | 0b1000_0000 },
            _ => {
                let error = format!("Number system {} in argument {} (line {}) isn't available.", encoding, argument, line).red().to_string();
                panic!("{}", error);
            }
        }
    }

    pub fn remove_comments(code: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for line in code.iter() {
            if let Some(first_character) = line.chars().nth(0) {
                if first_character != '#' && line != "" {
                    let part_without_comment = line.split("#").nth(0).unwrap();
                    result.push(part_without_comment.trim().to_string());
                }
            }
        }

        result
    }


    /*fn resolve_macro(macro_n: &str, arg1: Option<u32>, arg2: Option<u32>, line: u32) -> Vec<String> {
        // Remove leading !
        let macro_characters: Vec<char> = Ok(macro_n.chars().collect()).unwrap();
        let macro_internal_name = macro_characters[1..].to_vec().iter().collect().as_ref();

        match macro_internal_name{
            "jmp" => {
                if arg1.is_none(){
                    let error = format!(" Macro '{}' at line {} requires at least 1 argument but 0 were found.", macro_internal_name, line).red().to_string();
                    panic!("{}", error);
                }
                if arg1.unwrap() < 128{
                    return vec![format!("jmp {}", arg1.unwrap())];
                }
                return vec![
                    format!("ldii {}", arg1.unwrap()), // Load immediate to internal register
                    format!("jmp R{}", RESERVED_REGISTER),
                ];
            }
            _ => {
                let error = format!("Unknown macro '{}' at line {}.", macro_internal_name, line).red().to_string();
                panic!("{}", error);
            }
        }

        return vec![]
    }*/

    pub fn get_replacements_from_code(code: Vec<String>) -> Vec<Replacement> {
        let mut replacements: Vec<Replacement> = Vec::new();
        let mut i: u16 = 0;
        let mut current_line_number: u16 = 0;
        for line in code.iter() {
            current_line_number += 1;
            // Ensure line has at least one char
            if line.is_empty() { continue; }
            if line.starts_with('.') {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 2{
                    let error = format!("Two arguments required for constant declaration, but {} where found at line {}.", parts.len(), current_line_number).red().to_string();
                    panic!("{}", error);
                }
                let constant_name = parts.get(0).unwrap().chars().collect::<Vec<char>>()[1..].iter().collect::<String>();
                let constant_value = parts.get(1).unwrap().chars().collect::<Vec<char>>().iter().collect();
                replacements.push(Replacement::new(constant_name, constant_value, false));
                continue;
            }
            if line.ends_with(":"){
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 1{
                    let error = format!("Function declaration requires function name (and nothing more or less). This wasn't followed at line {}.", current_line_number).red().to_string();
                    panic!("{}", error);
                }
                let characters = parts.get(0).unwrap().chars().collect::<Vec<char>>();
                let function_name = characters[0..characters.len() - 1].iter().collect::<String>();

                let replacement = Replacement::new(function_name, i.to_string(), true);
                println!("i: {}", i);
                println!("{}", replacement.make_description());
                replacements.push(replacement);

                continue;
            }
            i += 1;
        }

        replacements = Self::replace_replacements(replacements);

        replacements
    }

    fn replace_replacements(replacements: Vec<Replacement>) -> Vec<Replacement> {
        let mut output: Vec<Replacement> = replacements.to_vec();
        for _ in 0..replacements.len() {
            for i in 0..output.len() {
                let mut origin = output[i].clone();
                for replacement in replacements.iter() {
                    if replacement.get_name() == output[i].get_value() {

                        output[i].set_value(replacement.get_value(), origin.get_is_function())
                    }
                }
            }
        }

        output
    }

    pub fn remove_declaration_lines(code: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for line in code.iter() {
            if line.is_empty() { continue; }
            if line.starts_with('.') { continue; }
            if line.ends_with(':') { continue; }

            result.push(line.to_string());
        }

        result
    }



    pub fn get_start_function(replacements: Vec<Replacement>) -> FunctionMetadata {
        // Fetch start function name
        let mut start_function_name: Option<String> = None;
        for replacement in replacements.iter() {
            println!("Replacemeasfdasdfnt: {}", replacement.get_name());
            if replacement.get_name() != "global_start" { continue }
            println!("#Replacemeasfdasdfnt: {}", replacement.get_value());
            start_function_name = Some(replacement.get_value().to_string().clone());
            break;
        }

        if start_function_name.clone().is_none() {
            let error = "Needs start function declaration (insert .global_start <main/start>).".red().to_string();
            panic!("{}", error);
        }

        // Fetch start & end position
        let mut start_function_start_pos: Option<u16> = None;
        let mut start_function_end_pos: Option<u16> = None;
        let mut i: u16 = 0;
        for replacement in replacements.iter() {
            i += 1;
            if replacement.get_value() != start_function_name.clone().unwrap() { continue }

            if !replacement.get_is_function(){
                //let error = format!("Invalid constant name '{}'. No constant can be named {:?}", replacement.get_name(), start_function_name.clone().unwrap()).red().to_string();
                //panic!("{}", error);
                continue;
            }

            start_function_start_pos = Some(replacement.get_value().parse::<u16>().unwrap());

            // The end position is just the next function / the last number
            if replacements.len() > i as usize {
                let end_replacement = replacements.get(i as usize).unwrap();

                if !end_replacement.get_is_function(){
                    /*let error = format!("Constants can't be defined in global_start function. Please move {} to the top instead", end_replacement.get_name()).red().to_string();
                    panic!("{}", error);*/
                    continue;
                }

                let end_function_value = end_replacement.get_value().parse::<u16>().unwrap();
                start_function_end_pos = Some(end_function_value)
            }

            break;
        }

        if start_function_start_pos.is_none() {
            let error = format!("Start function name ({}) defined but not implemented.", start_function_name.unwrap().red().to_string()).red().to_string();
            panic!("{}", error);
        }

        FunctionMetadata{ name: start_function_name.unwrap().to_string(), start: start_function_start_pos.unwrap(), end: start_function_end_pos }
    }

    pub fn apply_replacements_in_code(replacements: Vec<Replacement>, code: &mut Vec<String>){
        for i in 0..code.iter().len(){
            for replacement in replacements.iter() {
                code[i] = code[i].replace(replacement.get_name().as_str(), replacement.get_value().as_str()).as_str().to_string();
            }
        }
    }

    pub fn function_lines_to_function_bytes(replacements: &mut Vec<Replacement>){
        for i in 0..replacements.len(){
            if !replacements[i].get_is_function() { continue; }
            let replacement = replacements[i].clone();
            let new_value = (replacement.get_value().parse::<u16>().unwrap() * 3).to_string();
            let is_function = replacement.get_is_function();
            replacements[i].set_value(new_value, is_function);
        }
    }

    pub fn move_replacements_after_end_function(start_function_length_lines: u16, old_replacements: Vec<Replacement>) -> Vec<Replacement>{
        let mut stop_updating = false;
        let mut replacements: Vec<Replacement> = old_replacements.to_vec();
        // The problem:
        // Start function name is global_start, so it'll always think the start function starts at .global_start main.
        // Retrieve the value of .global_start instead and wait for a function to equal it instead.
        let mut global_start_value : u16 = 0;
        for old_replacement in replacements.iter() {
            if old_replacement.get_name() == "global_start" { global_start_value = old_replacement.get_value().parse::<u16>().unwrap(); break; }
        }
        for i in 0..replacements.len(){
            if !replacements[i].get_is_function(){ continue; }
            println!("::replacing xaysdf {} (repl: {}", replacements[i].get_name(), !stop_updating);
            if replacements[i].get_value() == global_start_value.to_string().as_str() {
                replacements[i].set_value(0.to_string(), true);
                stop_updating = true;
            }
            if !stop_updating {
                let new_replacement = replacements[i].get_value().parse::<u16>().unwrap() + start_function_length_lines;
                replacements[i].set_value(new_replacement.to_string(), true);
                println!("doing something named {} with {}", replacements[i].get_name(), new_replacement);
                continue;
            }
        }

        replacements
    }


    pub fn convert(a: &str, a_sys: i8, b_sys: i8, ) -> String {
        // Convert to int
        let mut total = 0;
        let mut i = 0;
        for character in a.to_ascii_uppercase().chars().rev() {
            let corresponding_number = CHARACTERS.iter().position(|p| *p.to_string() == character.to_string());
            let number_in_sum = corresponding_number.unwrap() as i32 * (a_sys as i32).pow(i);
            total += number_in_sum;
            i += 1;
        }

        println!("{}", total);

        // Convert to new type
        let mut b: String = "".to_string();

        while total > 0{
            b = b + CHARACTERS[(total % b_sys as i32) as usize];
            total = total / b_sys as i32;
        }

        b.chars().rev().collect::<String>()
    }
}
pub struct FunctionMetadata{
    pub name: String,
    pub start: u16,
    pub end: Option<u16>,
}