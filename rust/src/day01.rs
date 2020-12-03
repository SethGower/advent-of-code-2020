pub fn part1(inp: String) {
    let v: Vec<&str> = inp.split('\n').collect(); // adds string of each num to vector
    let mut int: Vec<i32> = Vec::new(); // vector for the integers
    for number in v.iter() {
        let temp = match number.parse::<i32>() { // Converts number to integer
            Ok(num) => num,
            Err(_) => continue, // if conversion failed, just go to next entry
        };
        int.push(temp);
    }

    // This isn't the best way, since it is O(n^2).
    let mut done = false;
    for num in int.iter() {
        for num1 in int.iter() {
            if num + num1 == 2020 {
                println!("The numbers were: {} and {}", num, num1);
                println!("{}", num * num1);
                done = true;
                break;
            }
        }
        if done { // breaks out of the outer loop
            break;
        }
    }
}

pub fn part2(input: String) {
    let v: Vec<&str> = input.split('\n').collect();
    let mut int: Vec<i32> = Vec::new();
    for number in v.iter() {
        let temp = match number.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        int.push(temp);
    }

    // This isn't the best way, since it is O(n^3). this was the first thing I thought of
    let mut done = false;
    for num in int.iter() {
        for num1 in int.iter() {
            for num2 in int.iter() {
                if num + num1 + num2 == 2020 {
                    println!("The numbers were: {}, {}, and {}", num, num1, num2);
                    println!("{}", num * num1 * num2);
                    done = true;
                    break;
                }
            }
            if done {
                break;
            }
        }
        if done {
            break;
        }
    }
}
