#include "algorithms.hpp"
#include "linkedlist.hpp"
#include <iostream>

Stack::Stack() {
    this->list == nullptr;
}

Stack::~Stack() {
}

void Stack::push(char element) {
    LinkedList<char>* currentElement = this->list;

    // If this is the 1st time. We initialize the list.
    if (currentElement == nullptr) {
        this->list = new LinkedList<char>();
        currentElement = this->list;
        currentElement->next = nullptr;
    }

    while (currentElement->next != nullptr) {
        currentElement = currentElement->next;
    }

    this->list->value = element;
    this->list->next = nullptr;
}

char Stack::pop() {
    char value = '\0';

    if (this->list != nullptr) {
        LinkedList<char>* currentElement = this->list;

        while (currentElement->next != nullptr) {
            currentElement = currentElement->next;
        }

        value = currentElement->value;
        delete currentElement;
    }

    return value;
}