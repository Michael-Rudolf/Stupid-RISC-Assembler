use std::cmp::PartialEq;
use colored::Colorize;
use crate::instruction;
use crate::utility::replacement::Replacement;
use crate::utility::math;

const CHARACTERS: [&str; 36] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

pub struct ArgumentParser {

}

enum DataType{
    Int8,
    Int16,
    Int32,

}

impl DataType {
    fn from_string(d_type: &str) -> DataType {
        match d_type {
            "8b" | "str" | "char" => DataType::Int8,
            "16b" => DataType::Int16,
            "32b" => DataType::Int32,
            _ => { let error = format!("Unknown data type: {}", d_type).red(); panic!("{}", error); }
        }
    }
}

enum SectionType{
    TEXT,
    DATA,
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {

        match (self, other) {
            (DataType::Int8, DataType::Int8) => true,
            (DataType::Int16, DataType::Int16) => true,
            (DataType::Int32, DataType::Int32) => true,
            _ => false,
        }
    }
}

impl ArgumentParser {
    pub fn argument_to_8_bit_binary(argument: &str, line: i32) -> u8 {
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
            "x" => { Self::convert(number, 16, 10).parse::<u8>().unwrap() },
            "o" => { Self::convert(number, 8, 10).parse::<u8>().unwrap() },
            "b" => { Self::convert(number, 2, 10).parse::<u8>().unwrap() },
            "r" => { number.parse::<u8>().unwrap() | 0b1000_0000 },
            "\'" => { number.chars().nth(0).unwrap() as u8 },
            _ => {
                let error = format!("Number system {} in argument {} (line {}) isn't available.", encoding, argument, line).red().to_string();
                panic!("{}", error);
            }
        }
    }

    pub fn split_sections(code: Vec<String>) -> (/*data: */ Vec<String>, /*text*/Vec<String>) {
        let mut section_data: Vec<String> = Vec::new();
        let mut section_text: Vec<String> = Vec::new();
        let mut current_section: Option<SectionType> = None;

        for line in code {
            match line.as_str() {
                "<text>" => {current_section = Some(SectionType::TEXT); continue;},
                "<data>" => {current_section = Some(SectionType::DATA); continue;},
                _ => {},
            }

            match current_section {
                Some(SectionType::TEXT) => {section_text.push(line);},
                Some(SectionType::DATA) => {section_data.push(line);},
                None => {},
            }
        }

        (section_data, section_text)
    }

    pub fn compile_data_section(lines: Vec<String>) -> (Vec<u8>, Vec<Replacement>) {
        let mut data: Vec<u8> = Vec::new();
        let mut replacements: Vec<Replacement> = Vec::new();

        for line in lines {
            let parts = Self::line_to_argument_parts(line.as_str());//line.split(" ").collect::<Vec<&str>>();
            let name = &parts[0];
            let data_type = DataType::from_string(&*parts[1]);
            let data_def = parts[2..].to_vec();
            let mut bytes = Self::data_to_bytes(data_def.clone(), data_type);
            let start_position = data.len();

            data.append(&mut bytes);

            let replacement = Replacement::new(name.to_string(), start_position.to_string(), false);

            replacements.push(replacement);
        }
        (data, replacements)
    }

    fn data_to_bytes(data: Vec<String>, data_type: DataType) -> Vec<u8> {
        let data = data;
        let mut data_bytes: Vec<u8> = Vec::new();
        if data_type == DataType::Int8 {
            let mut data0chars = data[0].chars().collect::<Vec<char>>();
            if data0chars[0] == '"' {
                // Decode the chars
                //data0chars.remove(0);
                data0chars.remove(0);
                data0chars.remove(data0chars.len() - 1);
                for date in data0chars{
                    let replacement = Self::argument_to_8_bit_binary(&("\'".to_owned() + date.to_string().as_str() + "\'"), -1);
                    data_bytes.push(replacement);
                }
            }else{
                // Decode the numbers
                for date in data{
                    data_bytes.push(Self::argument_to_8_bit_binary(&date, -1));
                }
            }
        }

        data_bytes
    }
    
   

    // ZKW
    pub fn get_replacements_from_code(code: Vec<String>) -> Vec<Replacement> {
        let mut replacements: Vec<Replacement> = Vec::new();
        let mut passed_bytes: u32 = 0;
        let mut current_line_number: u16 = 0;
        for line in code.iter() {
            current_line_number += 1;
            // Ensure line has at least one char
            if line.is_empty() { continue; }
            if line.starts_with('.') {
                let parts: Vec<String> = Self::line_to_argument_parts(line);//line.split_whitespace().collect();
                if parts.len() != 2{
                    let error = format!("Two arguments required for constant declaration, but {} were found at line {} ({}).", parts.len(), current_line_number, line).red().to_string();
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
                let replacement = Replacement::new(function_name, passed_bytes.to_string(), true);
                replacements.push(replacement);

                continue;
            }
            if let Some(instruction_name) = line.split_whitespace().nth(0) {
                passed_bytes += instruction::Instruction::bytes_required_by_instruction_by_name(instruction_name.to_string()) as u32;
            }
        }

        replacements = Self::replace_replacements(replacements);
        replacements.push(Replacement::new("data_offset".to_string(), passed_bytes.to_string(), true));

        replacements
    }

    pub fn line_to_argument_parts(lines: &str) -> Vec<String> {
        let characters: Vec<char> = lines.chars().collect();
        let mut arguments: Vec<String> = vec![];
        let mut current_argument = "".to_string();
        let mut next_character_escaped = false;
        let mut whitespaces_escaped = false;

        for character in characters {
            if (character == ' ' || character == '\t') && !(next_character_escaped || whitespaces_escaped) {
                // add current argument
                if current_argument != "" {
                    arguments.push(current_argument.parse().unwrap());
                    current_argument = "".to_string();
                }
                continue;
            }

            if character == '\'' || character == '"' {
                whitespaces_escaped = !whitespaces_escaped;
            }

            if character == '\\' && !next_character_escaped {
                next_character_escaped = true;
                continue;
            }else{
                next_character_escaped = false;
            }

            current_argument = current_argument.to_string() + character.to_string().as_str();
        }

        if current_argument != "" {
            arguments.push(current_argument.parse().unwrap());
        }

        arguments

    }

    fn replace_replacements(replacements: Vec<Replacement>) -> Vec<Replacement> {
        let mut output: Vec<Replacement> = replacements.to_vec();
        for _ in 0..replacements.len() {
            for i in 0..output.len() {
                let  origin = output[i].clone();
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
            if line.starts_with('#') { continue; }

            let mut line_without_comments = "".to_string();

            for char in line.chars(){
                if char == '#' { break; }
                line_without_comments += char.to_string().as_str();
            }

            result.push(line_without_comments);
        }

        result
    }

    pub fn apply_replacements_in_code(replacements: Vec<Replacement>, code: &mut Vec<String>){
        for i in 0..code.iter().len(){
            for replacement in replacements.clone().iter() {
                code[i] = code[i].replace(replacement.get_name().as_str(), replacement.get_value().as_str()).as_str().to_string();
                code[i] = Self::resolve_all_math_ops_in_line(code[i].clone(), replacements.clone());
            }
        }
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

        // Convert to new type
        let mut b: String = "".to_string();

        while total > 0{
            b = b + CHARACTERS[(total % b_sys as i32) as usize];
            total = total / b_sys as i32;
        }

        b.chars().rev().collect::<String>()
    }

    pub fn resolve_all_math_ops_in_line(line: String, replacements: Vec<Replacement>) -> String {
        let mut next_replacement_data = "".to_string();
        let mut output: String = "".to_string();
        let mut write_to_repl_data = false;

        let characters: Vec<char> = line.chars().collect();

        for character in characters.iter() {
            if *character == '[' {
                write_to_repl_data = true;
                continue;
            }
            if *character == ']' {
                write_to_repl_data = false;
                output += math::resolve_string(next_replacement_data.clone().parse().unwrap(), replacements.clone()).as_str();
                next_replacement_data = "".to_string();
                continue;
            }
            if write_to_repl_data {
                next_replacement_data += character.to_string().as_str();
            }else {
                output = (&*(output.to_owned() + &*character.to_string())).parse().unwrap();
            }
        }



        output
    }
}
pub struct FunctionMetadata{
    pub name: String,
    pub start: u16,
    pub end: Option<u16>,
}