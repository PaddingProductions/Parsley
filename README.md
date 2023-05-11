# Parsley

A parser-combinator and interpreter for a custom scripting syntax, written in Rust.
Written as a hobby project after being inspired by blogs discussing interpreters, parsers, abstract syntax trees.
This implementation employed extensive use of Rust's algebraic type system, different from typical OOP / polymorphic design patterns.
This project allowed me to become fluent in navigating around Rust's safe type system, and the language itself in general.

---
## Usage
Thanks to Cargo's ease of use, simply clone the repository and run `cargo run`.
You should see the following:
```
   ===  [Parsley]  ===
syn > 
```
Type in the script to execute, and it will be parsed and executed upon return.

### Example
```
   ===  [Parsley]  ===
syn > my_var1=1+2
setting 'my_var1' to 3
syn > my_var2=3+2*1
setting 'my_var2' to 5
syn > if:my_var1==my_var2-2{my_var1=10;}
setting 'my_var1' to 10
syn > eval:my_var1+10
Expression Evaluated to => '20'
```
Refer to the `Syntax` section for a ~~complete~~ documentation on Parsley grammar.

---
## Grammar
__Will fill out soon__ 

---

## Resources
- [Learning Parser Combinators With Rust - Bodil Stokke](https://bodil.lol/parser-combinators/)
- [Crafting Interpreters - Robert Nystrom](https://craftinginterpreters.com/)
- [How to Approach Writing an Interpreter From Scratch - Sakib Hadžiavdić](https://www.toptal.com/scala/writing-an-interpreter)

