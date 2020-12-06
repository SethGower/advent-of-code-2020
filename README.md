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

## Puzzles

1. **Report Repair**
    - Part 1: Find two numbers who sum to 2020. Print the product of these two
    - Part 2: Find three numbers who sum to 2020. Print the product of these
      three
2. **Password Philosophy**: Check if a "password" follows a certain password
   scheme. Example password entry: `1-3 a: abcde`
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
3. **Toboggan Trajectory**: Count the amount of "trees" when traversing down a
   "slope". The input is a grid where `.` represents an open spot, and `#`
   represents a tree. The grid tiles infinitely to the right. While traveling
   down, count the amount of trees.
    - Part 1: Travel down 1 and right 3. Return number of trees found
    - Part 2: Travel down multiple times, return the product of the number of
      trees for each This solution was threaded to make the program more
      performant.
        - Right 1, down 1.
        - Right 3, down 1. (This is the slope you already checked.)
        - Right 5, down 1.
        - Right 7, down 1.
        - Right 1, down 2.
4. **Passport Processing**: Count the number of "valid" "passports" in a batch
   of passports. Passports have 8 different fields
    - Part 1: Simply check which passport entries have all necessary fields.
      `cid` could be ignored
    - Part 2: Actually make sure the data is valid

| Key    | Description     | Format                                            |
| ------ | --------------- | ------------------------------------------------- |
| `byr`  | Birth Year      | 4 characters, [1920,2002]                         |
| `iyr`  | Issue Year      | 4 characters, [2010, 2020]                        |
| `eyr`  | Expiration Year | 4 characters, [2020, 2030]                        |
| `hgt`  | Height          | [59,76]in or [150,193]cm                          |
| `hcl`  | Hair Color      | `#<6 digit hex>`                                  |
| `ecl`  | Eye Color       | [`amb`, `blu`, `brn`, `gry`, `grn`, `hzl`, `oth`] |
| `pid`  | Passport ID     | 9 digit number                                    |
| `cid`  | Country ID      | 9 digit number, can be ignored                    |

The table shows the formatting specifiers for the items, where `[x,y]` denotes a
range from `x` to `y` inclusive
