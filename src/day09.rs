use itertools::Itertools;
use std::time::Instant;

use super::fmt_dur;

const PREAMBLE: usize = 25;

pub fn part1(input: String) -> Option<String> {
    let start = Instant::now();
    let nums = parse(input);
    let elapsed = start.elapsed();
    println!("Parsing took {}", fmt_dur(elapsed));

    let start = Instant::now();
    let bad_num = find_bad_num(&nums)?;
    let elapsed = start.elapsed();
    println!("Calculation took {}", fmt_dur(elapsed));
    println!("{}", bad_num);

    Some(bad_num.to_string())
}
pub fn part2(input: String) -> Option<String> {
    let start = Instant::now();
    let nums = parse(input);
    let elapsed = start.elapsed();
    println!("Parsing took {}", fmt_dur(elapsed));

    let start = Instant::now();
    let bad_num = find_bad_num(&nums)?;
    let elapsed = start.elapsed();
    println!("Finding Bad Number took {}", fmt_dur(elapsed));
    println!("{}", bad_num);

    let start = Instant::now();
    let (min, max) = checksum(&nums, bad_num);
    let elapsed = start.elapsed();
    let sum = min + max;
    println!("{}", sum);
    println!("Checksum Calculation took {}", fmt_dur(elapsed));

    Some(sum.to_string())
}

fn parse(input: String) -> Vec<u64> {
    let mut nums: Vec<u64> = vec![];
    for num in input.lines() {
        match num.parse::<u64>() {
            Ok(n) => nums.push(n),
            Err(e) => println!("Errored on {} with error {}", num, e.to_string()),
        }
    }
    nums
}
fn sum_valid(chunk: &[u64], num: u64) -> bool {
    let combinations = chunk.iter().combinations(2);
    for combination in combinations {
        if combination[0] + combination[1] == num {
            return true;
        }
    }
    false
}
fn find_bad_num(nums: &Vec<u64>) -> Option<u64> {
    for (i, window) in nums.windows(PREAMBLE).enumerate() {
        if !sum_valid(window, nums[i + PREAMBLE]) {
            return Some(nums[i + PREAMBLE]);
        }
    }
    None
}
fn checksum(vec: &Vec<u64>, num: u64) -> (u64, u64) {
    for i in 2..PREAMBLE {
        let windows = vec.windows(i);
        for window in windows {
            let sum: u64 = window.iter().sum();
            if sum == num {
                return (*window.iter().min().unwrap(), *window.iter().max().unwrap());
            }
        }
    }
    (0, 0)
}
