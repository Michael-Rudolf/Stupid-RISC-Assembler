use colored::Colorize;

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