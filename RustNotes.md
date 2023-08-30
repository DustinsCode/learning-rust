# Intro

I am learning how to program in Rust, following their [book](https://rust-book.cs.brown.edu)!

# Installation (Ch 1)

## Linux/Mac

1. Install `rustup`

```zsh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Notes:

-   `rustup` is for managing the version of Rust on machine
-   'RUSTUP_HOME' env variable for rustup metadata/toolchains/etc
    -   defaults to `~/.rustup`
-   `CARGO_HOME` env var for cargo
    -   defaults to `~/.cargo`
    -   cargo, rustc, and rustup commands will be in `$CARGO_HOME/bin`

### To Update

`rustup update`

### To Uninstall

`rustup self uninstall`

### docs

`rustup docs`

# Project Creation with Cargo (Ch 1.3)

`cargo new <project name>`

`Cargo.toml` file -> Basically `package.json`

-   `[package]` name, version, etc.
-   `[dependencies]`: project dependencies ("crates")

## Build

`cargo build`

-   automatically builds to `target/debug`
-   Can run with `./target/debug/<project name>`

`cargo run`

-   builds and runs

`cargo check`

-   make sure it'll compile first

## Release

`cargo build --release`

-   use for releases and benchmarking

# First Program (Guessing Game, Ch 2)

-   imports section is called the _prelude_.

```rust
use std::io;
```

-   uses `let` to declare variable:
    -   variables are immutable by default
    -   use `mut` to make it mutable

```rust
let apples = 5;
//or
let mut guess = String::new();
```

-   Uses references:
    -   references are also immutable by default

```rust
io::stdin()
    .read_line(&mut guess);
```

-   String interpolation

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

-   range inclusive: `start..=end`
-   docs:

    -   `cargo docs --open`
    -   WSL workaround: `explorer.exe .` -> navigate to target/docs/\<package\> open `index.html` in browser

-   `::<>` is called turbofish lol

# Core Concepts (Ch 3)

## 3.1 Variables and Mutability

-   variables are immutable by default
-   make variable mutable by: `let mut x = 5`
-   `const`s cannot use `mut`
    -   have to define type
    -   can be declared in any scope, including global
    -   Cannot use a variable to assign value
    -   all uppercase, snake_cased
-   `const` can be used in the global scope, while `let` can only be used in a function

You can declare a new variable with the same name as a previous variable in a process called "shadowing". The first variable is _shadowed_ by the second.

```rust
fn main() {
  let x = 5;

  let x = x + 1;

  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x is: {x}");

}
```

-   When shadowing, we're effectively creating a new variable, so we don't have to adhere to the same type as we would have to do using `mut`

```rust
let mut spaces = "    "; //string
let spaces = spaces.len(); // int
```

## 3.2 Data Types

Rust is statically typed.

Scalar Types:

1. integers
2. floats
3. bools
4. chars

### Integers

-   signed and unsigned
-   Neat table from book:

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

-   Signed vs Unsigned refers to if number can be negative
-   Signed numbers are stored using two's compliment representation
-   Each signed variant can store numbers from `-2^(n-1)` to `2^(n-1) - 1`
    -   `i8` can store from -128 to 127
-   Unsigned variants can store from 0 to `2^n - 1`
    -   `u8` can store from 0 to 255
-   `isize` and `usize` depend on the arch type of the computer (64 if on 64-bit architecture, 32 if on 32-bit)

> Literals:

Number literals can use `_` as a visual separator

-   Decimal: 1_000 = 1000
-   Hex: 0xff
-   Octal: 0o77
-   Binary: 0b1111_0000
-   Byte (u8 only): b'A'

> Which integer type to use?

If unsure, Rust defaults to i32. The primary place you'd see `isize` and `usize` is when indexing some sort of collection.

> Overflow

Rust compiler will _not_ check for overflow that cause panics when compiling in `--release` mode, only in debug mode. Instead, Rust performs two's complement wrapping. Values greater than the maximum value the type can hold "wrap around" to the minimum of the values the type can hold. The program won't panic, but the variable will have a value that isn't what we expected.

To explicitly handle overflow, use these methods in the standard lib:

-   Wrap in all modes: `wrapping_*` methods, like `wrapping_add`
-   Return `None` if there is overflow with `checked_*`
-   Return the value and a boolean indicating whether there was overflow with `overflowing_*`
-   Saturate at the value's minimum or maximum values with `saturating_*`

### Floats

`f32` and `f64`. Default type is `f64` because on modern CPUs it's about the same speed as `f32` but is capable of more precision.

All floats are signed.

-   `f32` is single-precision
-   `f64` had double-precision

### Numeric Operations

```Rust
fn main() {
  let sum = 5 + 10;
  let diff = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1
  let remainder = 43 % 5;
}
```

### Boolean

`bool`.  Yup.

### Char

Notated with single quotes. `char`.

```Rust
fn main() {
  let c = 'z';
  let z: char = 'Z'; //with explicit type annotation
  let emoji = '🤠';
}
```

- `char` is 4 bytes
- represents a Unicode Scalar Value
  - ASCII and beyond

### Compound Types

Can group multiple values into one type.  Tuples and Arrays

#### Tuples:

- fixed length

```Rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
The variable `tup` above, bind to the entire tuple because a tuple is considered a single compound element.  To get the individual values out, we can use pattern matching to destructure:

```Rust
fn main() {
  let tup = (500, 6.4, 1);

  let (x, y , z) = tup;

  println!("The value of y is: {y}");
}
```

We can also access indexes of the tuple directly like so: `tup.0`

A tuple without any values is called a `unit`, written as `()`. Represents an empty value or an empty return type. Expressions implicitly return the unit value if they don't return any other value.

#### Arrays

- Every element must be the same type
- fixed length
- allocated on the stack
- not as flexible as the `vector` type
- typed using the following pattern: `[type; size]`

```Rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- accessed with `a[0]`
- program will panic if array goes out of bounds
- The syntax [x; y] declares an array with y copies of x

## 3.3 Functions

- uses snake_case for function and variable names
- must declare parameter type
- calling a function is an expression
- calling a macro is an expression
- a new scope block created with curlies is an expression

```Rust
fn main() {
  let y = {
    let x = 3;
    x + 1 // No semicolon?
  };

  println!("The value of y is: {y}");
}
```

From the book:
> If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.


- annotate return values with `-> TYPE`
- most functions return the last expression implicitly


## 3.4 Comments

```Rust
// ...yup
```

- There are also doc comments `///` i guess

## 3.5 Control Flow

Most common
- if expressions
- lööps

### `if`

- condition must be a boolean, no js magicianry 
- using if in a let statement:

```Rust
// feels like a ternary tbh
let condition = true;
let number = if condition { 5 } else { 6 };
```

### Lööps

3 kinds of loops: 
- `loop`
- `while`
- `for`

LOOP LABELS?!?!?
- disambiguate between multiple loops!!!!!
  - ARE YOU KIDDING ME????
- must begin with a single quote
  - see `/loops/main.rs`

While loops, pretty standard. 

For loops:

```rs
fn main() {
  let a = [10,20,30,40,50]

  for element in a {
    println!("the value is: {element}");
  }
}
```

see also:

```rs
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```