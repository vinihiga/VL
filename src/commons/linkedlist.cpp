#include "linkedlist.hpp"

template<typename T>
LinkedList<T>::LinkedList(T newValue) {
    this->value = newValue;
    this->next = nullptr;
}

template<typename T>
LinkedList<T>::~LinkedList() {

}

template<typename T>
T& LinkedList<T>::operator[](int) {
}

template<typename T>
void append(T newValue) {
    LinkedList<T> *cursor = this;

    while (cursor->next != nullptr) {
        cursor = cursor->next;
    }

    this->next = new LinkedList<T>(newValue);
}