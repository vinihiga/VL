pub fn tokenize(line_content: &str) {
    let mut tokens: Vec<String> = line_content.split(" ").map(|s| s.to_string()).collect();
    let operation_queue = ["*", "/", "+", "-"];

    for operation in operation_queue {
        if tokens.len() >= 3 && tokens[1] == operation {
            while tokens.len() >= 3 {
                let new_token = join_variables(
                    tokens[0].clone(),
                    tokens[2].clone(),
                    tokens[1].clone());

                tokens.remove(0);
                tokens.remove(0);
                tokens.remove(0);

                if tokens.len() > 0 {
                    tokens.remove(0);

                    if new_token.parse::<i32>().unwrap() >= 0 {
                        tokens.push("+".to_string());
                    } else {
                        tokens.push("-".to_string());
                    }
                }

                tokens.push(new_token);

                println!("{}", tokens.join(" "));
            }
        }
    }
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