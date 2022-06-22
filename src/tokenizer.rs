pub fn tokenize(line_content: &str) {
    let tokens: Vec<String> = line_content.split(" ").map(|s| s.to_string()).collect();
    let operation_queue = ["*", "/", "+", "-"];
    let mut currentIndex = 0;

    for operation in operation_queue {
        for index in 0 .. tokens.len() {
            print!("{}", index);
        }
    }

    // std::deque<std::string> operationQueue = { "*", "/", "+", "-" }
    // int previousIndex = 0;

    // for (std::string token : tokens) {    

    //     std::find(operationQueue.begin(), operationQueue.end(), token)

    //     std::cout << "Output: " << token.c_str() << std::endl;
    // }

    // std::cout << "Loaded symbols" << std::endl;
}