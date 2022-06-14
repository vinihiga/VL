#include "stack.hpp"
#include "linkedlist.hpp"
#include <iostream>

Stack::Stack() {
    this->list = nullptr;
}

Stack::~Stack() {
}

void Stack::push(std::string element) {
    LinkedList<std::string>* currentElement = this->list;

    // If this is the 1st time. We initialize the list.
    if (currentElement == nullptr) {
        this->list = new LinkedList<std::string>();
        currentElement = this->list;
        currentElement->value = element;
        currentElement->next = nullptr;
        return;
    }

    while (currentElement->next != nullptr) {
        currentElement = currentElement->next;
    }

    currentElement->next = new LinkedList<std::string>();
    currentElement = currentElement->next;
    currentElement->value = element;
    currentElement->next = nullptr;
}

std::string Stack::pop() {
    std::string value = "\0";

    if (this->list != nullptr) {
        LinkedList<std::string>* currentElement = this->list;
        LinkedList<std::string>* previousElement = nullptr;

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
    LinkedList<std::string>* currentElement = this->list;

    if (currentElement == nullptr) {
        return size;
    }

    while (currentElement->next != nullptr) {
        currentElement = currentElement->next;
        size++;
    }

    return size + 1;
}