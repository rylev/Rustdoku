# Rustdoku

A simple Sudoku solver written in Rust.

Theoretically this solver should be able to solve any "solvable" puzzle.

## Run

To run use `cargo run --` followed by the puzzle as one string starting in the
upper left corner of the puzzle and following each row indicating blank spots
with `*` characters.

For example:

```
cargo run -- \
  "*4*1*5*2*8*76*34*5*2*****8*47*****51*********61*****43*6*****3*9*43*21*8*3*4*9*7*"
```

## Test

Simply run: `cargo test`
