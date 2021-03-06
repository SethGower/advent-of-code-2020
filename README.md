# Advent of Code 2020

This is my repository for my solutions for [Advent of
Code](https://adventofcode.com/) 2020. This is the first time I am doing this
competition, and have decided to take this opportunity to advance my rust
skills. I am using a template created by [Scoder12](https://github.com/Scoder12)
on [repl.it](https://repl.it/@Scoder12/aoc-rust-template).

## Usage

[aocf](https://github.com/nuxeh/aocf) is required to use run these. This is a
wrapper that accesses the AoC website in order to get the inputs. I found this
project and thought it was cool. The reason aocf is used is to make sure the
most up to date input data is being used. A fall back (instead of `aocf`) is to
specify a filename of the input data. To do this, use the command line options
of either `--file <FILE>` or `-f <FILE>`

In order to submit, run the program with the flag `--submit` or `-s`.
This will automatically submit the puzzle output, if `aocf` has been properly
configured. 

The main program takes an argument for the day number. So for example, to submit
the puzzle solution for Day 5, you could run. 

```
cargo run -- 5 --submit
```

A template for this project is located [here](https://github.com/SethGower/rust-aoc-template)
