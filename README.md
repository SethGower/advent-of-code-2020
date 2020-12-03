# Advent of Code 2020

This is my repository for my solutions for [Advent of
Code](https://adventofcode.com/) 2020. This is the first time I am doing this
competition, and have decided to take this opportunity to advance my rust
skills. I am using a template created by [Scoder12](https://github.com/Scoder12)
on [repl.it](https://repl.it/@Scoder12/aoc-rust-template).

## Usage

[aocf](https://github.com/nuxeh/aocf) is required to use run these. This is a
wrapper that accesses the AoC website in order to get the inputs. I found this
project and thought it was cool. I may modify the `main.rs` to have a fallback,
but this assures updated inputs are being used.

## Puzzles

1. Day 1
    - Part 1: Find two numbers who sum to 2020. Print the product of these two
    - Part 2: Find three numbers who sum to 2020. Print the product of these
      three
2. Day 2: Check if a "password" follows a certain password scheme. Example
password entry: `1-3 a: abcde`
    - Part 1: Numbers at the beginning give a range, describing the frequency
      at which the character after the range appears in the password. In the
      example above, the letter 'a' must appear at least once, and at most
      three times. The example above is valid since 'a' appears once, which
      falls in the range [1,3]
    - Part 2: Instead of the frequency being described, the numbers describe
      indices of the password. If the character appears at one or the other
      (XOR operation) indices then the password is valid. The passwords use 1
      based indexing. The example is valid since the letter 'a' appears at
      index 1, but not index 3.
