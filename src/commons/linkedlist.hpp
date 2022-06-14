#ifndef LINKEDLIST_H
#define LINKEDLIST_H

template <typename T>
struct LinkedList {
    T value;
    LinkedList<T>* next;
};

#endif