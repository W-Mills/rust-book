# Learning Rust

Exercises while going through official book from [doc.rust-lang.org](https://doc.rust-lang.org/book/title-page.html)

Notes taken from following along [The Rust Book](https://doc.rust-lang.org/book/title-page.html)

# Basics
- `rustup` is a CLI tool for managing Rust versions
  - `rustc --version` to check if rust is installed correctly
  - `rustup doc` to get documentation in browser
- Cargo is the Rust build system and package manager
  `$ cargo new` to create a new project
  - "crates" => Packages of code
  - [TOML](https://toml.io/en/) is cargo's configuration format
  - Source files for codes live in the `/src` dir
  - `$ cargo check` to only compile without creating an executable
  - `$ cargo run` to execute (and re-compile if files have changed)
  - `$ cargo build` to compile and create an executable
    - `$ cargo build --release` to compile with optimizations => in arget/release`
  - `$ cargo doc --open` to open the documentation for the code/libraries in the project in a browser (very smart!)
  - "binary crates" are executable, "library crates" can't, are intended to be used in other programs


- Compiling and running are separate steps
  - `rustc` runs the rust compiler => outputs a binary executable (prefer `$ cargo build` for more complex projects)
  - Rust is "ahead of time compiled" meaning, a binary executable can be run by a machine without rust installed

## Style basics
- `rustfmt` is a formatting tool
- indent with four spaces
-

## Variable basics
- It is common to shadow existing variable names when converting type to reuse the variable name
```rust
let mut guess = String::new();
// ...
let guess: u32 = guess.trim().parse().expect("Please type a number!"); // note this new var is immutable
// `parse` method on strings converts it to another type
// the colon `:` after the var name tells Rust that the type will be annotated (here an unsigned 32-bit integer)
```


## Function Basics
- `main` function is special, it's always the first code that runs in every executable rust program.
-  Function body is wrapped in `{}` for all function bodies
- `println!` calls a rust macro. `println` without the `!` is a normal function.
  - Macros have some different rules than functions
- `;` indicates the end of an expression


## General
- `use std::io;` to import libraries (called the prelude)
- variables are immutable by default, use `mut` before the variable name
- `//` for comments
- In `String::new()`, `::` refers to an associated function of the string type that makes a new value (a growable UTF-8 string)
- `.read_line(&mut guess)` => `&` means the argument is a reference (immutable by default)
- `Result` values are an enum that can have multiple variants: `Ok` and `Err`. Errors are handled by calling the `expect` method on the result type, if it has an error it gets handled here, otherwise returns the value that `Ok` is holding
- `{}` can be used as placeholders in `println!`
  - `println!("x = {x} and y + 2 = {}", y + 12);` => `x = 5 and y = 14`
- number types default to `i32` 32 bit integer


## Traits
- `use rand::Rng` => `Rng` trait defines methods that are implemented by random number generators. A trait must be in scope in order to use its methods
  - `secret_num = rand::thread_rng().gen_range(1..=100);`
    - => `rand::thread_rng` is the function with the rng to be used local to the current thread of execution
    - `gen_range` method takes a range exp as an arg, is inclusive on lower and upper bounds

## Match expression
```rust
  match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("Spot on!!"),
  }
```
- A `match` expr is comprised of "arms", each arm has a pattern to match against and the code to run if matched
  - expr ends after first successful match
