#include "symbolstable.hpp"

static SymbolsTable* instance = nullptr;

SymbolsTable* SymbolsTable::getInstance() {
    if (SymbolsTable::instance == nullptr) {
        SymbolsTable::instance = new SymbolsTable();
    }

    return SymbolsTable::instance;
}

void parse() {
}
