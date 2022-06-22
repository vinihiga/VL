#ifndef LINKEDLIST_H
#define LINKEDLIST_H

template <typename T>
class LinkedList {
private:
    T value;
    LinkedList<T>* next;
public:
    LinkedList(T newValue);
    ~LinkedList();
    T& operator[](int);
    void append(T newValue);
};

#endif