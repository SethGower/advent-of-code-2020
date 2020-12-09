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
