#include "tokenizer.hpp"

void tokenize(std::ifstream& file) {
    std::string word;
    LinkedList<std::string> tokens;

    while (file >> word) {

        tokens.value = word;
        tokens.next = nullptr;

        std::cout << word.c_str() << std::endl;
    }

    std::cout << "Loaded symbols" << std::endl;
}