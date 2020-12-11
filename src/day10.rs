pub fn part1(input: String) -> Option<String> {
    let mut joltages: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    joltages.sort();

    let mut ones = 0;
    let mut threes = 1;

    let mut prev_adapter : u32 = 0;
    for adapter in joltages.iter(){
        if adapter - prev_adapter == 1 {
            ones += 1;
        }else if adapter - prev_adapter == 3 {
            threes += 1;
        }
        prev_adapter = *adapter;

    }
    println!("Ones: {}\nThrees: {}", ones, threes);

    Some((ones * threes).to_string())
}

pub fn part2(input: String) -> Option<String> {
    Some(input)
}
