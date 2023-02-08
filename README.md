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
- snake_case as conventional style for function and variable names

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
- `//` for comments, use `//` for each line of multi-line comments
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
---
# Common Programming Concepts: Rust Edition (ch. 3)
### Variables
- Variables are immutable by default, add `mut` in front of var to make it mutable => `let mut x = 123;`
  - Can't use `mut` with constants => `const x = 5;`
  - Constants can only be set to a constant expression, not a result from a value at runtime `const SOME_CONST: u32 = 14`
- Shadowing variables is allowed, and encouraged for transforming types, mutating in inner scopes.
  - use `let variable = "some new value"` to declare a new variable with the same name for transformations, while keeping
    the "original" variable immutable
    - using `mut` instead will not allow you to change the type, and actually mutates the value
  - "Shadowed" value is used by compiler until scope ends or a new shadowing overrides it

### Data Types
- Rust is a statically typed langauge => `some_var: u32 = //...` for type annotations (here, unsigned 32 bit integer)
- Scalar type represents a single value. Rust has four primary scalar types:
  - Integer
  - Floating-point number
  - Booleans
  - Characters

Integer Type: Number without a fractional component
  - signed int types start with `i` instead of `u` (signed means it's possible for the int to have a negative value)
  - the num like `u32` refers to the number of bits of storage it uses
  - can use `_` as a visual separator => `10_000` equals `10000`
  - If value exceeds typed range => Integer Overflow
    - Int overflow results in panic in debug mode, but wrapping in release mode (should be handled, reliance is considered an error)

Floating-Point Types
- Two types => `f32` and `f64`, refers to bits of size for each float
  - `f32` is single precision
  - `f64` is double precision
- All floats are signed

Numeric Operations
 - `+`, `-`, `*`, `/` (division), `%` (remainder) all operate as expected and evaluate to a single value that is bound to a variable


Boolean Type
- `true` and `false`, 1 byte in size
- e.g. `let f: bool = false;`

Character Type `char`
- Rusts' must primitive alphabetic type, four bytes in size (a unicode scalar value)
- Use `' '` single quotes (compared to string literals which use `" "` double quotes)
- e.g. `let my_string: char = 'abc';`

#### Compound Types: Two primitive compound types: Tuples and Arrays
- Group multiple values into a single type

Tuple Type
  - Group a variety of types into one compound type
  - Fixed length
  - Create using a comma separated list of values in parenthesis e.g.
  - a "unit" is a tuple without any values, written as `()` to represent an empty value or return type
  ```rust
  fn main() {
      let tup: (i32, f64, u8) = (100, 6.9, 9)

      // can be destructured using pattern matching
      let (x, y, z) = tup
      println!({y}) => 6.9

      // or zero-based index access
      let nine = tup.2 // => 9
  }
  ```

Array Type
- Unlike a tuple, every element of an array must have the same type
- Fixed length (use a Vector type instead for collection that can grow or shrink in size)
- Used for data to allocated to stack rather than heap
- Value accessed by index in brackets `let first = a[0]`
- A value for an index out of the defined bounds will cause panic at runtime (memory safety)

```rust
fn main() {
    let a: [i32; 3] = [1,2,3]; //  3 element array of i32 ints
    let b = [3; 5]; // 3 element array, each with the value 5 => [5, 5, 5]
}
```

### Functions
- `fn main() {}` => entrypoint to most rust apps
- `fn` keyword to declare new functions
- Rust doesn't care about order that functions are defined, only that they're in scope
- **Must** declare type of each parameter
- Func return values are not named, but must be declared after an arrow `->`

### Statements and Expresssions
Function bodies are comprised of a series of statements optionally ending in an expr
- **Statements**: Instructions that perform some action and do not return a value
  - e.g. creating and assigning a var, func definitions
- **Expressions**: Evaluate to a resultant value
  - e.g. calling a func or a macro, a new scope block using `{}`
  - expressions **do not** include ending semicolons
    - putting an ending semicolon at the end of an expr transforms it into a statement (no return value)

### Control Flow
- `if ` and loops are the most common control flows
- `if` expr
  - Blocks of code associated with the conditions are called *arms*
  - Will **not** automatically type convert non-boolean types to a bool

- Rust has three kinds of loops: `loop`, `while`, `for`

#### Loop
- `loop` endless loop until explicit use of `break` to stop
  - use `continue` to skip over rest of code in block for this iteration and move to next iteration
  - `break` and `continue` by default apply to the innermost of nested loops
    - Can use loop labels to disambiguate multiple loops: `'some_loop_label: loop {}` (notice the `'` prefix)
      - Can affect labelled loops `break 'some_loop_label;` (notice the `'` prefix)
- `while` loop automatically `break`s when condition is no longer `true`
- `for` loop to execute some code for each item in a collection
  - e.g. `for element in some_array { println!("Value is: {element}"); }`

---
# Ch 4: Understanding Ownership [Link to Chapter](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- Ownership is Rust's most unique feature, providing memory safety guarantees without a garbage collector
  - A set of rules that govern how a Rust program manages memory

- Stack: Values are stored (pushed) in order received and removed (popped) in reverse (last in first out)
  - Good for data with a known size
- Heap: Less organized, need to request space from memory allocator to get a pointer to the memory address of the location
  - Slower for read and write, because have to allocate memory, also have to follow a pointer for reads

- **Ownership Rules**:
  - Each value in Rust has an owner
  - There can only be one owner at a time (assigning a value to another variable *moves* it)
  - When the owner goes out of scope, the value will be dropped

- `String` type (not string literal) is mutable: `let s = String::from("hello world");`
- `drop` function is called automatically at the closing curly brace to drop memory allocation for vars going out of scope
- reassignment of variables results in a "move" not a shallow copy because first variable is invalidated:


```rust
let s1 = String::from("some string");
let s2 = s1; // invalidates s1
println!("{}", s1) // throws error
```

### References and Borrowing

- Prepending `&` in front of a type signifies a reference is passed
  - Allows referring to a value without taking ownership of it (called *borrowing*)
  - References are immutable by default
  - Can create a mutable reference by declaring the initial var with `mut` and `&mut` as the reference
    - Can't mix both `&s` immutable and `&mut s` mutable references
  - Can either have 1 mutable reference, or limitless immutable references, but can't mix them
  - Refernces must always be valid, or compiler will err (if reference is to a value that has been dropped)
```rust
fn calculate_length(s: &String) -> usize { // note &String type is immutable ref
  s.len()
}

let mut s = String::from("hi"); // var declared as mutable

change(&mut s); // mut reference passed

fn change(some_string: &mut String) { // mut param declared
  some_string.push_str(", other string");
}
```

### The Slice Type

- A slice is a kind of reference that lets you refer to a contiguous sequence of elements in a collection
- `&str` is the type for "string slice" which is an immutable reference
  - `fn returns_string_slice(s: &String) -> &str { ...` takes String referrence, returns a string slice
  - Generally more flexible to use `(s: &str)` as a param, becuase can be used for `&String` and `&str` values
- Can slice other collections like arrays more generally too

```rust
// String slice
let s = String::from("hack days!");

let slice = &s[0..4]; // "hack" => slice var s from index 0 to 3 (start index to one more than end index)
let slice = &s[..4]; // also "hack"

let slice = &s[6..11] // "days!"
let slice = &s[6..] // also "days!"

let slice = &s[..] // "hack days!" => from start index to one more than end index

// Array slice
let a = [1, 2, 3];

let arr_slice = &a[..3]; // [1, 2]
```
---
# Using Structs to Structure Related Data Ch.5
- Struct is a custom data type that allows you to package and organize related values as named *fields* that comprise a meaningful group
  - Methods are associated functions
  - Can instantiate structs, use dot notation to retrieve values
  - Use `mut` to declare mutable instance => entire struct must be mutable, can't allow mutating only specific fields
  - Can use factory functions to build structs with default values

```rust
// Define struct with field names and types
struct User {
  active: bool,
  first_name: String, // Instance owns this value
  email: String,
  sign_in_count: u64,
}

// Instantiation of mutable instance with concrete values
let mut user1 = User {
  active: true,
  first_name: String::from("Will"),
  email: String::from("abc@example.com"),
  sign_in_count: 1,
};

user1.email = String::from("someotheremail@place.com"); // dot notation for field access

// Factory to build user with default values
fn build_user(email: String, first_name: String) -> User {
  User {
    active: true,
    first_name, // Field init shorthand notation to use parameter value with same name
    email,
    sign_in_count: 1,
  }
}

// Using struct update syntax to create a new instance spreading values from other struct (for values not explicitly set)
let user2 = User {
  email: String::from("newemail@example.com"),
  ..user1 // spread must come last (will not use the value for email because it was explicitly defined above)
};

```

- Tuple Structs Without Named Fields behave like tuples
```rust
struct Color(i32, i32, i32); // unnamed fields because would be redundant

let black = Color(0, 0, 0); //
```

## Methods
- Similar to functions, but defined within a struct (or enum/trait object) and pirst parameter is always `self`
  - `self` is the instance of the struct the method is being called on
- Define a method within the context of a struct by starting an `impl` (Implement) block for the struct
- Can define a method with the same name as one of the struct's fields which can be invoked by using `()` parenthesis
  - e.g. `rect1.width()` for method, `rect1.width` for field value
  - Generally used to create getter methods (not implemented automatically) => helpful for access control

```rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // within impl block, `Self` is an alias for the type the block is for
  fn area(&self) -> u32 {  // `&self` is short for `self: &Self`
    self.width * self.height
  }
}
```

- All functions defined within an `impl` block are *associated functions* (associated with the type named in the `impl`)
  - Can define non-method associated functions (don't have `self` as first param)
  - Used like `String::from`
```rust
impl Rectangle {
  fn square(size: u32) -> Self { // -> returns an instance of Rectangle with matching width and height
    Self {
      width: size,
      height: size,
    }
  }
}

let new_square = Rectangle::square(10);
```
---
# Enums and Pattern Matching Ch.6
- Enums allow defining a type by enumerating its possible variants
  - An enum value can only be one of its variants



- `Option` enum expresses that a value can be something or nothing
- Pattern matching in `match` expr for running code based on the value of an enum
- Rust does not have a null type, instead has `Option`
  - To use an `Option<T>`, need to handle both the `None` and `Some` cases or compiler will fail
```rust
enum Option<T> { // `<T>` is a Generic type, can hold any type
  None,
  Some(t),
}
```

## `match` for Control Flow
- `if` requires condition to evaluate to a boolean, but `match` can take any type
- `match` has 'arms' that have a pattern and some code
- Matches must be exhaustive
- Use `other` as a catch all pattern matcher where value is used (must be last) e.g. `other => func_with_param(other)`
  - Use `_` as a special pattern that matches any value and does not bind to it e.g  `_ => no_param_func()`
    - Use `_ => ()` to match any value and do nothing (expr is an empty tuple)

```rust
fn plus_five(x: Option<i32>) -> Option<i32> { // Return value could be None, or a 32 bit integer
  match x {
    None => None, // If value is `None`, return `None`
    Some(i) => Some(i + 5), // the `i` binds to the value contained in `Some`
  }
}

let six = Some(6); // 6
let eleven = plus_five(six); // 11
let none = plus_five(None); // None
```

## `if let` Control Flow
- Can avoid some of the boilerplate code of a match when only interested in handling code that matches one pattern and ignoring the rest:
```rust
let config_max = Some(3u8);
if let Some(max) = config_max { // Only does something if matches the `Some(max) = config_max` expr
  // do something
}
```

---
