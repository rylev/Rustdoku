# Rustdoku

A simple Sudoku solver written in Rust.

Theoretically this solver should be able to solve any "solvable" puzzle.

## Run Natively

To run use `cargo run --` followed by the puzzle as one string starting in the
upper left corner of the puzzle and following each row indicating blank spots
with `*` characters.

For example:

```
cargo run -- \
  "*4*1*5*2*8*76*34*5*2*****8*47*****51*********61*****43*6*****3*9*43*21*8*3*4*9*7*"
```

## Run on Web

To run on the web, you'll need to make sure you have `wasm-pack` installed which you can do by running `cargo install wasm-pack`.

Then, `cd` into the crates/rustdoku-js folder and run `wasm-pack build --target no-modules`.

You'll then need a static server to serve all the files. I like to use `basic-http-server` which you can get by running `cargo install basic-http-server`. You can then run the server like so: `basic-http-server`.

Finally, visit http://localhost:4000/index.html

## Test

Simply run: `cargo test`
