use itertools::Itertools;
const PREAMBLE: usize = 25;
pub fn part1(input: String) -> Option<String> {
    let mut nums: Vec<u64> = vec![];
    for num in input.lines() {
        match num.parse::<u64>() {
            Ok(n) => nums.push(n),
            Err(e) => println!("Errored on {} with error {}", num, e.to_string()),
        }
    }

    let mut bad_num = 0;
    for i in PREAMBLE..nums.len() {
        if !sum_valid(&nums[i - PREAMBLE..i], nums[i]) {
            bad_num = nums[i];
            break;
        }
    }
    println!("{}", bad_num);

    Some(bad_num.to_string())
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

pub fn part2(input: String) -> Option<String> {
    let mut nums: Vec<u64> = vec![];
    for num in input.lines() {
        match num.parse::<u64>() {
            Ok(n) => nums.push(n),
            Err(e) => println!("Errored on {} with error {}", num, e.to_string()),
        }
    }

    let mut bad_num = 0;
    for i in PREAMBLE..nums.len() {
        if !sum_valid(&nums[i - PREAMBLE..i], nums[i]) {
            bad_num = nums[i];
            break;
        }
    }

    println!("{}", bad_num);
    let (min, max) = checksum(&nums, bad_num);
    let sum = min + max;

    Some(sum.to_string())
}
fn checksum(vec: &Vec<u64>, num: u64) -> (u64, u64) {
    for i in 0..vec.len() {
        for j in i..vec.len() {
            let slice = &vec[i..j];
            let sum : u64 = slice.iter().sum();
            if sum == num {
                return (*slice.iter().min().unwrap(), *slice.iter().max().unwrap());
            }
        }
    }
    (0, 0)
}
