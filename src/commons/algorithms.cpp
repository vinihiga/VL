#include "algorithms.hpp"
#include "linkedlist.hpp"
#include <iostream>

Stack::Stack() {
    this->list = nullptr;
}

Stack::~Stack() {
}

void Stack::push(char element) {
    LinkedList<char>* currentElement = this->list;

    // If this is the 1st time. We initialize the list.
    if (currentElement == nullptr) {
        this->list = new LinkedList<char>();
        currentElement = this->list;
        currentElement->value = element;
        currentElement->next = nullptr;
        return;
    }

    while (currentElement->next != nullptr) {
        currentElement = currentElement->next;
    }

    currentElement->next = new LinkedList<char>();
    currentElement = currentElement->next;
    currentElement->value = element;
    currentElement->next = nullptr;
}

char Stack::pop() {
    char value = '\0';

    if (this->list != nullptr) {
        LinkedList<char>* currentElement = this->list;
        LinkedList<char>* previousElement = nullptr;

        while (currentElement->next != nullptr) {
            previousElement = currentElement;
            currentElement = currentElement->next;
        }

        value = currentElement->value;
        delete currentElement;

        if (previousElement != nullptr) {
            previousElement->next = nullptr;
        } else {
            this->list = nullptr;
        }

    }

    return value;
}

int Stack::count() {
    int size = 0;
    LinkedList<char>* currentElement = this->list;

    if (currentElement == nullptr) {
        return size;
    }

    while (currentElement->next != nullptr) {
        currentElement = currentElement->next;
        size++;
    }

    return size + 1;
}