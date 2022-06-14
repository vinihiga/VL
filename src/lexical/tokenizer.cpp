#include "tokenizer.hpp"

void tokenize(std::ifstream& file) {
    std::string word;

    while (file >> word) {        
        std::cout << word.c_str() << std::endl;
    }

    std::cout << "Loaded symbols" << std::endl;
}