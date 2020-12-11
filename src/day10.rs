use itertools::Itertools;

pub fn part1(input: String) -> Option<String> {
    let mut joltages: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    joltages.sort();

    let mut ones = 0;
    let mut threes = 1;

    let mut prev_adapter: u32 = 0;
    for adapter in joltages.iter() {
        if adapter - prev_adapter == 1 {
            ones += 1;
        } else if adapter - prev_adapter == 3 {
            threes += 1;
        }
        prev_adapter = *adapter;
    }
    println!("Ones: {}\nThrees: {}", ones, threes);

    Some((ones * threes).to_string())
}

pub fn part2(input: String) -> Option<String> {
    let joltages: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    let max = joltages.iter().max()? + 3;
    println!("{:#?}", joltages);
    let mut valid: u32 = 0;
    for i in 1..joltages.len() {
        let joltages = joltages.clone();
        for comb in joltages.iter().combinations(i) {
            let mut comb = comb.clone();
            comb.push(&max);
            if is_valid(&mut comb, 0) {
                valid += 1;
            }
        }
    }
    println!("Valid: {}", valid);
    Some(input)
}

fn is_valid(joltages: &mut Vec<&u32>, input: u32) -> bool {
    joltages.sort();
    let mut prev_adapter: u32 = input;
    for adapter in joltages.iter() {
        if **adapter - prev_adapter > 3 {
            return false;
        }
        prev_adapter = **adapter;
    }
    println!("{:?} is valid", joltages);
    true
}
