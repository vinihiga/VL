//
//  main.cpp
//  VL
//
//  Created by Vinicius Higa on 12/05/22.
//

#include <iostream>
#include <fstream>

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
}
