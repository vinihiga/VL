//
//  main.cpp
//  VL
//
//  Created by Vinicius Higa on 12/05/22.
//

#include <iostream>
#include <fstream>
#include "commons/algorithms.hpp"
#include "commons/linkedlist.hpp"

void checkParameters(char *argv[]);
char** tokenize();

int main(int argc, char *argv[]) {
    checkParameters(argv);
    return 0;
}

void checkParameters(char *argv[]) {
    std::ifstream indata;
    indata.open("../samples/addition.vl");

    if (!indata) {
        return;
    }

    std::string word;
    Stack* stack = new Stack();

    while (indata >> word) {
        stack->push(word.c_str()[0]);
    }

    indata.close();
    std::cout << "Loaded " << stack->count() << " symbols" << std::endl;
}
