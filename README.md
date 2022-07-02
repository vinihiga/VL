# VL
Vini's Language

## My study of how compilers works

This an idea and exercise of how compilers works. I have followed partially the "Dragon's Book".

## What I've done

- File loader.
- Tokenizer for each line of the file (VL format).
- Semantic analyzer.
- Calculations processor.

## What I haven't done

- AST Tree data structure for optimizing the instructions. Today we are just going through the line by comparing a queue of operations (*, /, +, -), so the order of complexity it's high and unoptimized for another features.
- Conversion of VL instructions to assembly format.
- Functions definitions or any commons functions (ex. print).
