# simple-compiler

![workflow](https://github.com/LionsWrath/simple-compiler/actions/workflows/rust.yml/badge.svg)

Simple TINY BASIC transpiler to C written in rust.

## Tiny Basic Grammar

```
program ::= {statement}
statement ::= "PRINT" (expression | string) nl
    | "IF" comparison "THEN" nl {statement} "ENDIF" nl
    | "WHILE" comparison "REPEAT" nl {statement} "ENDWHILE" nl
    | "LABEL" ident nl
    | "GOTO" ident nl
    | "LET" ident "=" expression nl
    | "INPUT" ident nl
comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
expression ::= term {( "-" | "+" ) term}
term ::= unary {( "/" | "*" ) unary}
unary ::= ["+" | "-"] primary
primary ::= number | ident
nl ::= '\n'+
```

## How to run

Running only the compiler:

```
cargo run --release
target/release/simple-compiler -i <input filename> -o <output filename>
```

If you have gcc installed, you can:

```
./build.sh <input filename>
```
