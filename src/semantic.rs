use std::borrow::Borrow;

pub fn fix_expressions(tokens: &mut Vec<String>) {
    let mut index: usize = 1;
    let mut has_found_errors: bool = false;
    
    loop {
        if !has_found_errors && index >= tokens.len() {
            break;
        } else if index >= tokens.len() {
            has_found_errors = false;
            index = 1;
        }

        let is_actual_index_number = tokens[index].parse::<f64>();
        let previous_index = index - 1;

        // Handling negative numbers
        if index > 1 {
            let is_pre_operator_number = tokens[index - 2].parse::<f64>();

            if tokens[index - 1] == "-" && (!is_negative_number(tokens[index - 2].borrow()) || is_not_number(tokens[index - 2].borrow())) && is_actual_index_number.is_ok() {
                if !is_pre_operator_number.is_ok() {
                    let previous_content = tokens[previous_index].clone();
                    let current_content = tokens[index].clone();
                    let new_element = previous_content + &current_content;
                    tokens.remove(index);
                    tokens.remove(previous_index);
                    tokens.insert(previous_index, new_element);
                    has_found_errors = true;
                }
            }
        } else if tokens[index - 1] == "-" && is_actual_index_number.is_ok() {
            let previous_content = tokens[previous_index].clone();
            let current_content = tokens[index].clone();
            let new_element = previous_content + &current_content;
            tokens.remove(index);
            tokens.remove(previous_index);
            tokens.insert(previous_index, new_element);
            has_found_errors = true;
        }

        // Handling extra "+" in code
        if index > 1 {
            if tokens[index - 1] == "+" && is_not_number(tokens[index - 2].borrow()) && is_actual_index_number.is_ok() {
                tokens.remove(previous_index);
                has_found_errors = true;
            }
        } else if tokens[index - 1] == "+" && is_actual_index_number.is_ok() {
            tokens.remove(previous_index);
            has_found_errors = true;
        }

        index = index + 1;
    }

}

fn is_not_number(value: &String) -> bool {
    return value.parse::<f64>().is_err()
}

fn is_negative_number(value: &String) -> bool {
    let is_number = value.parse::<f64>();

    if is_number.is_err() {
        return false;
    }

    return value.parse::<f64>().unwrap() < 0.0;
}