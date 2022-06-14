//
//  main.cpp
//  VL
//
//  Created by Vinicius Higa on 12/05/22.
//

#include <iostream>
#include <fstream>
#include "lexical/tokenizer.hpp"

void checkParameters(char *argv[]);

int main(int argc, char *argv[]) {
    checkParameters(argv);
    return 0;
}

void checkParameters(char *argv[]) {
    std::ifstream file;
    file.open("../samples/addition.vl");

    if (!file) {
        return;
    }

    tokenize(file);
}
