#ifndef SYMBOLSTABLE_H_
#define SYMBOLSTABLE_H_

class SymbolsTable {
private:
    static SymbolsTable* instance;
    SymbolsTable() {}
public:
    ~SymbolsTable() {}
    static SymbolsTable* getInstance();
    void parse();
};

#endif