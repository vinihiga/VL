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

    while (indata >> word) {
        std::cout << word << std::endl;
    }

    indata.close();

    Stack* stack = new Stack();
    stack->push('i');
    stack->push('o');

    std::cout << stack->pop();
}
