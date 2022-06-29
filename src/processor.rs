pub fn calculate_sentence(line_content: &str) {
    let mut tokens: Vec<String> = line_content.split(" ").map(|s| s.to_string()).collect();
    let operation_queue = ["*", "/", "+", "-"];

    // Iterating for each operator. We follow the Mathmatics priority rule.
    for operation in operation_queue {

        // Verifying and solving each sentence.
        while tokens.contains(&operation.to_string()) {

            // Iterating for token to find the matching operator time.
            for index in 0 .. tokens.len() {
                if index < tokens.len() && tokens[index] == operation {
                    rearrange_token_and_calculate(index, &mut tokens);
                }
            }
        }
    }
}

fn rearrange_token_and_calculate(operator_index: usize, tokens: &mut Vec<String>) {
    let new_token = calculate(
        tokens[operator_index - 1].clone(),
        tokens[operator_index + 1].clone(),
        tokens[operator_index].clone());

    tokens.remove(operator_index - 1);
    tokens.remove(operator_index - 1);
    tokens.remove(operator_index - 1);

    tokens.insert(operator_index - 1, new_token);
}

fn calculate(x1: String, x2: String, operator: String) -> String {
    let mut result: String = "\0".to_string();
    let value1 = x1.parse::<f64>().unwrap();
    let value2 = x2.parse::<f64>().unwrap();

    if operator == "+" {
        result = (value1 + value2).to_string();
    } else if operator == "-" {
        result = (value1 - value2).to_string();
    } else if operator == "*" {
        result = (value1 * value2).to_string();
    } else if operator == "/" {
        if value2 == 0.0 {
            panic!("Division by zero!");
        }

        result = (value1 / value2).to_string();
    }

    return result;
}