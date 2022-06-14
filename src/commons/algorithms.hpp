#ifndef ALGORITHMS_H
#define ALGORITHMS_H
#include "linkedlist.hpp"

//template <typename T>
class Stack {
private:
    LinkedList<char>* list;
public:
    Stack();
    ~Stack();
    void push(char element);
    char pop();
    int count();
};

#endif