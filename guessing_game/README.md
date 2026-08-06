# Guessing Game

A simple number guessing game in Rust. The program picks a secret number between 1 and 100 and you keep guessing until you get it right.

## What I learned

- **Importing libraries** — `use std::io;` brings input/output into scope, and `use rand::RngExt;` lets us call methods like `random_range()` on a random number generator (renamed from `Rng`/`gen_range` in rand 0.10).
- **`loop` keyword** — `loop { }` repeats the block forever. `continue` restarts the loop (bad input) and `break` exits it (game won).
- **`match` keyword** — compares one value against patterns and runs the matching arm. Used to compare the guess against the secret number and to parse input with `Ok(...)` / `Err(...)`.

## Run

```sh
cargo run
```