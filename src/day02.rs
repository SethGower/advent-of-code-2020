pub fn part1(input: String) {
    fn process_entry(input: &str) -> bool {
        let tokens: Vec<&str> = input.splitn(2, ' ').collect();

        let mut bounds: [usize; 2] = [0, 0];

        if let Some(string) = tokens.get(0) {
            let bounds_in: Vec<&str> = string.split('-').collect();
            if let Some(string) = bounds_in.get(0) {
                if let Ok(num) = string.parse::<usize>() {
                    bounds[0] = num;
                }
            }
            if let Some(string) = bounds_in.get(1) {
                if let Ok(num) = string.parse::<usize>() {
                    bounds[1] = num;
                }
            }
        };
        if let Some(string) = tokens.get(1) {
            let s = String::from(*string).trim().replace(" ", "");
            let tokens: Vec<&str> = s.split(':').collect();
            if let Some(string) = tokens.get(0) {
                if let Some(password) = tokens.get(1) {
                    let password = String::from(*password);
                    let count: Vec<&str> = password.matches(string).collect();
                    let count = count.len();
                    if count >= bounds[0] && count <= bounds[1] {
                        return true;
                    }
                }
            }
        }

        return false;
    }
    let lines: Vec<&str> = input.split('\n').collect();

    let mut count: usize = 0;
    for entry in lines {
        if process_entry(entry) {
            count = count + 1;
        }
    }
    println!("Number of Valid Passwords: {}", count);
}

pub fn part2(input: String) {
    fn process_entry(input: &str) -> bool {
        let tokens: Vec<&str> = input.splitn(2, ' ').collect();

        let mut bounds: [usize; 2] = [0, 0];

        if let Some(string) = tokens.get(0) {
            let bounds_in: Vec<&str> = string.split('-').collect();
            if let Some(string) = bounds_in.get(0) {
                if let Ok(num) = string.parse::<usize>() {
                    bounds[0] = num;
                }
            }
            if let Some(string) = bounds_in.get(1) {
                if let Ok(num) = string.parse::<usize>() {
                    bounds[1] = num;
                }
            }
        };
        if let Some(string) = tokens.get(1) {
            let s = String::from(*string).trim().replace(" ", "");
            let tokens: Vec<&str> = s.split(':').collect();
            if let Some(string) = tokens.get(0) {
                if let Some(password) = tokens.get(1) {
                    let string = String::from(*string);
                    let password = String::from(*password);

                    let first = password.get(bounds[0]-1..bounds[0]).unwrap();
                    let second = password.get(bounds[1]-1..bounds[1]).unwrap();

                    return (string == first) ^ (string == second);
                }
            }
        }

        return false;
    }
    let lines: Vec<&str> = input.split('\n').collect();

    let mut count: usize = 0;
    for entry in lines {
        if process_entry(entry) {
            count = count + 1;
        }
    }
    println!("Number of Valid Passwords: {}", count);
}
