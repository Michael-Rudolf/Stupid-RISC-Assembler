use crate::utility::math;
use crate::tests::test::Test;
use crate::utility::replacement::Replacement;
use crate::argument_parser::ArgumentParser;
use colored::*;

pub struct MathTest;

impl Test for MathTest {
    fn execute() -> bool{
        let start_message = "Starting math tests.".green();
        println!("{}\n", start_message);
        // Test resolving
        let resolving_tests = ["1 + 5", "3 - 2", "8 / 4", "8 * 3", "x % 2", "y % 2"];
        let resolving_replacements = vec![Replacement::new("x".to_string(), "11".to_string(), false), Replacement::new("y".to_string(), "10".to_string(), true)];
        let resolving_results = ["6", "1", "2", "24", "1", "0"];

        for test in resolving_tests.iter().enumerate(){
            let calculated_result = math::resolve_string(test.1.to_string(), resolving_replacements.clone());
            if calculated_result == resolving_results[test.0]{
                let message = format!("{} is {}", test.1, calculated_result).green();
                println!("{}", message);
            }else{
                let message = format!("{} shouldn't be {}", test.1, calculated_result).red();
                println!("{}", message);
                return false;
            }
        }

        // Test resolving in strings via argument parser
        let string_resolving_tests = ["add R1 [1 + 2]", "add R1 [x + 5]", "asdf[5 - x]", "sto [x * y]"];
        let string_resolving_replacements = vec![Replacement::new("x".to_string(), "11".to_string(), false), Replacement::new("y".to_string(), "10".to_string(), true)];
        let string_resolving_solutions = ["add R1 3", "add R1 16", "asdf-6", "sto 110"];

        for test in string_resolving_tests.iter().enumerate(){
            let calculated_result = ArgumentParser::resolve_all_math_ops_in_line(test.1.to_string(), string_resolving_replacements.clone());
            let solution = string_resolving_solutions[test.0];
            if calculated_result == solution{
                let message = format!("{:?} is {}", test.1, calculated_result).green();
                println!("{}", message);
            }else {
                let message = format!("{:?} shouldn't be {}", test.1, calculated_result).red();
                println!("{}", message);
            }
        }
        true
    }
}
