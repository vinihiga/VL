pub fn tokenize(line_content: &str) {
    let mut tokens: Vec<String> = line_content.split(" ").map(|s| s.to_string()).collect();
    let operation_queue = ["*", "/", "+", "-"];

    // Iterating for each operator. We follow the Mathmatics priority rule.
    for operation in operation_queue {
        println!("time: {}", operation.to_string());

        // Verifying and solving each sentence.
        while tokens.contains(&operation.to_string()) {

            // Iterating for token to find the matching operator time.
            for index in 0 .. tokens.len() {
                if index < tokens.len() && tokens[index] == operation {
                    println!("{}", tokens.join(" "));

                    let new_token = join_variables(
                        tokens[index - 1].clone(),
                        tokens[index + 1].clone(),
                        tokens[index].clone());

                    tokens.remove(index - 1);
                    tokens.remove(index - 1);
                    tokens.remove(index - 1);

                    tokens.insert(index - 1, new_token);
                }
            }
        }
    }

    println!("The result is {}", tokens.join(" "));
}

fn join_variables(x1: String, x2: String, operator: String) -> String {
    let mut result: String = "\0".to_string();

    if operator == "+" {
        let value1 = x1.parse::<i32>().unwrap();
        let value2 = x2.parse::<i32>().unwrap();
        result = (value1 + value2).to_string();
    } else if operator == "-" {
        let value1 = x1.parse::<i32>().unwrap();
        let value2 = x2.parse::<i32>().unwrap();
        result = (value1 - value2).to_string();
    } else if operator == "*" {
        let value1 = x1.parse::<i32>().unwrap();
        let value2 = x2.parse::<i32>().unwrap();
        result = (value1 * value2).to_string();
    } else if operator == "/" {
        let value1 = x1.parse::<i32>().unwrap();
        let value2 = x2.parse::<i32>().unwrap();

        if value2 == 0 {
            panic!("Division by zero!");
        }

        result = (value1 / value2).to_string();
    }

    return result;
}

fn rearrange_token(new_token: &String, operator_index: usize, tokens_list: &Vec<String>) {
    
}