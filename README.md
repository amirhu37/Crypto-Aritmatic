# Crypto-Arithmetic Solver

A Rust-based tool for solving crypto-arithmetic puzzles, where each letter represents a unique digit.

## Features

- Convert words to numerical values based on character-to-digit mappings.
- Check the validity of solutions for given words and result.
- Solve crypto-arithmetic puzzles with up to 10 unique letters.

## Usage

### Running the Program

To run the program, use the following commands:

```sh
cargo build
cargo run
```
### Input :

```sh
Two Words as Input, Separated with Whitespace? SEND MORE
Result String? MONEY
```
### Output :

```sh
SEND + MORE = MONEY
Solution found: SENDMORE
S = 9
E = 5
N = 6
D = 7
M = 1
O = 0
R = 8
Y = 2
```