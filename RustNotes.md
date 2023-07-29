# Intro

I am learning how to program in Rust, following their [book](https://doc.rust-lang.org/book)!

# Installation

## Linux/Mac

1. Install `rustup`
```zsh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Notes:
- 'RUSTUP_HOME' env variable for rustup metadata/toolchains/etc
    - defaults to `~/.rustup`
- `CARGO_HOME` env var for cargo
    - defaults to `~/.cargo`
    - cargo, rustc, and rustup commands will be in `$CARGO_HOME/bin`

### To Update

`rustup update`

### To Uninstall

`rustup self uninstall`

### docs

`rustup docs`

# Project Creation with Cargo

`cargo new <project name>`

`Cargo.toml` file -> Basically `package.json`
- `[package]` name, version, etc.
- `[dependencies]`: project dependencies ("crates")

## Build

`cargo build`

- automatically builds to `target/debug`
- Can run with `./target/debug/<project name>`

`cargo run`
- builds and runs

`cargo check`
- make sure it'll compile first

## Release

`cargo build --release`
- use for releases and benchmarking

# First Programm (Guessing Game)

- imports section is called the *prelude*.
```rust
use std::io;
```

- uses `let` to declare variable:
  - variables are immutable by default
  - use `mut` to make it mutable
```rust
let apples = 5;
//or
let mut guess = String::new();
```

- Uses references:
  - references are also immutable by default
```rust
io::stdin()
    .read_line(&mut guess);
```

- String interpolation

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

- range inclusive: `start..=end`
- docs: 
  - `cargo docs --open`
  - WSL workaround: `explorer.exe .` -> navigate to target/docs/\<package\> open `index.html` in browser

# Core Concepts

