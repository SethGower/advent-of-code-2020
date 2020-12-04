pub fn part1(input: String) -> Option<String> {
    // Function to process a password entry
    fn process_entry(input: &str) -> bool {
        // splits the entry on spaces, but only the first space. Yields two items. The first index
        // will be the range
        let tokens: Vec<&str> = input.splitn(2, ' ').collect();

        let mut bounds: [usize; 2] = [0, 0];

        // Finds the two range values
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
        // Parses the rest of the string
        if let Some(string) = tokens.get(1) {
            // Gets rid of internal spaces in the string
            let s = String::from(*string).trim().replace(" ", "");
            // Splits on the colon, the first index will be the character specifier and the second
            // would be the password
            let tokens: Vec<&str> = s.split(':').collect();
            if let Some(string) = tokens.get(0) {
                if let Some(password) = tokens.get(1) {
                    let password = String::from(*password);
                    // Counts how many times the character appears in the password. I couldn't
                    // think of a better way, after perusing the documentation for std::String very
                    // briefly
                    let count: Vec<&str> = password.matches(string).collect();
                    let count = count.len();
                    // If the character frequency falls within the range, return true
                    if count >= bounds[0] && count <= bounds[1] {
                        return true;
                    }
                }
            }
        }

        // if it didn't return, then it isn't valid. This could be because the input string wasn't
        // valid, or the password wasn't valid
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
    Some(count.to_string())
}

pub fn part2(input: String) -> Option<String> {
    fn process_entry(input: &str) -> bool {
        // splits the entry on spaces, but only the first space. Yields two items. The first index
        // will be the range
        let tokens: Vec<&str> = input.splitn(2, ' ').collect();

        let mut bounds: [usize; 2] = [0, 0];

        // Finds the two range values
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
        // Parses the rest of the string
        if let Some(string) = tokens.get(1) {
            // Gets rid of internal spaces in the string
            let s = String::from(*string).trim().replace(" ", "");
            // Splits on the colon, the first index will be the character specifier and the second
            // would be the password
            let tokens: Vec<&str> = s.split(':').collect();
            if let Some(string) = tokens.get(0) {
                if let Some(password) = tokens.get(1) {
                    let string = String::from(*string);
                    let password = String::from(*password);

                    // I used unwrap because I got lazy
                    let first = password.get(bounds[0] - 1..bounds[0]).unwrap();
                    let second = password.get(bounds[1] - 1..bounds[1]).unwrap();

                    // returns the XOR of the characters matching
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
    Some(count.to_string())
}
