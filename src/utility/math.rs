use colored::Colorize;
use crate::utility::operation::Operation;
use crate::utility::replacement::Replacement;

/// Turns a string like "1 + 2" to "3"
pub(crate) fn resolve_string(string: String, replacements: Vec<Replacement>) -> String {
    // Tokenize
    let tokens: Vec<&str> = string.split(' ').collect();
    let mut operand_1: Option<i64> = None;
    let mut operand_2: Option<i64> = None;
    let mut operation: Option<Operation> = None;

    for token in tokens {
        let mut token = token.to_string();
        if let Some(op) = Operation::from_string(&token) {
            operation = Some(op);
            continue;
        }
        // Apply all replacements
        for replacement in replacements.clone() {
            let replacement = token.replace(&replacement.get_name(), &replacement.get_value());
            token = replacement;
        }

        if operand_1.is_none() {
            operand_1 = Some(token.parse::<i64>().unwrap());
            continue;
        }
        operand_2 = Some(token.parse::<i64>().unwrap());
        break;
    }

    if let Some(operation) = operation {
        if let Some(operand_1) = operand_1 {
            if let Some(operand_2) = operand_2 {
                let result = operation.perform(operand_1, operand_2).to_string();
                return result;
            }
            let error = format!("Only 1/2 operands specified to resolve ({})", string).red();
            panic!("{}", error);
        }
        let error = format!("No operand specified to resolve ({})", string).red();
        panic!("{}", error);
    }else{
        let error = format!("No operation specified to resolve ({})", string).red();
        panic!("{}", error);
    }
}

