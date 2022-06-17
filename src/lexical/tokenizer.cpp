#include "tokenizer.hpp"

void tokenize(std::ifstream& file) {
    std::string word;

    while (file >> word) {        
        std::cout << word.c_str() << std::endl;
    }

    std::cout << "Loaded symbols" << std::endl;
}

/*

2 + 3 * 5 + 4 = 21

2, 3, 5, 4
previous = 3
next = 5

2, 15, 4

+, +

15 + 4 = 19
19 + 2 = 21

END OF INSTRUCTIONS

MOV AL, 3
MUL AL, 5

*/