# Absolute value

A CLI program built on Rust to determine absolute values of a set of numbers and expressions.

![Screenshot of program](assets/Screenshot%202023-01-24%20at%2015.26.21.png)

## Compile

Install Rust here: https://www.rust-lang.org/learn/get-started

```bash
cargo build --release
```

The command will generate an executable `absolute_value` inside `target/release/`.

## Run

```bash
./target/release/absolute_value '-3 + 3' '-5'
```

If any expression is not valid it will not be shown on final result.
