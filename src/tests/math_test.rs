use crate::utility::math;
use crate::tests::test::Test;
use crate::utility::replacement::Replacement;
use colored::*;

pub struct MathTest;

impl Test for MathTest {
    fn execute() -> bool{
        let start_message = "Starting math tests.".green();
        println!("{}\n", start_message);
        // Test resolving
        let tests = ["1 + 5", "3 - 2", "8 / 4", "8 * 3", "x % 2", "y % 2"];
        let replacements = vec![Replacement::new("x".to_string(), "11".to_string(), false), Replacement::new("y".to_string(), "10".to_string(), true)];
        let results = ["6", "1", "2", "24", "1", "0"];

        for test in tests.iter().enumerate(){
            let calculated_result = math::resolve_string(test.1.to_string(), replacements.clone());
            if calculated_result == results[test.0]{
                let message = format!("{} is {}", test.1, calculated_result).green();
                println!("{}", message);
            }else{
                let message = format!("{} shouldn't be {}", test.1, calculated_result).red();
                println!("{}", message);
                return false;
            }
        }
        true
    }
}
