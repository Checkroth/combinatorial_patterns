![crates.io](https://img.shields.io/crates/v/combinatorial_patterns.svg)

[Documentation](https://docs.rs/combinatorial_patterns/0.1.0/combinatorial_patterns/)

# Combinatorial Patterns in Rust

This project is meant to maintain rust implementations of [combinatorial patterns](https://en.wikipedia.org/wiki/Combinatorics).

If you have any alternative solutions to any implemented solutions, or want to add a new pattern, please feel free to contribue.

Development may appear stagnant, but it isn't out of lack of interest, but rather out of any need to implement solutions problems in this niche sector of mathematics. If a reason to do so arrises, it will be done.

Currently, only Latin Square generation is implemented.

## Latin Squares

Usage and implementation details can be found in rust docs.

Specifics of latin squares can be found on [wikipedia](https://en.wikipedia.org/wiki/Latin_square).

The implemented solution is not the most efficient method, but it is the easiest to understand.

### Usage

Just add combinatorial_patterns to your `Cargo.toml`.

# Things To Do:

- Improve latin square generation to use more efficient methods
- Improve latin square functionality to allow for latin rectangles, and toggleable requirements for balance.
- Implement unit tests
- Start work on [Graph Theory](https://en.wikipedia.org/wiki/Graph_theory)
