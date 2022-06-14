#ifndef ALGORITHMS_H
#define ALGORITHMS_H
#include "linkedlist.hpp"
#include <iostream>

//template <typename T>
class Stack {
private:
    LinkedList<std::string>* list;
public:
    Stack();
    ~Stack();
    void push(std::string element);
    std::string pop();
    int count();
};

#endif