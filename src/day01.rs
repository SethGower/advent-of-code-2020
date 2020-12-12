pub fn part1(inp: String) -> Option<String> {
    let v: Vec<&str> = inp.split('\n').collect(); // adds string of each num to vector
    let mut int: Vec<i32> = Vec::new(); // vector for the integers
    for number in v.iter() {
        let temp = match number.parse::<i32>() {
            // Converts number to integer
            Ok(num) => num,
            Err(_) => continue, // if conversion failed, just go to next entry
        };
        int.push(temp);
    }

    // This isn't the best way, since it is O(n^2).
    for num in int.iter() {
        for num1 in int.iter() {
            if num + num1 == 2020 {
                println!("The numbers were: {} and {}", num, num1);
                let prod: i32 = num * num1;
                println!("{}", prod);
                let s: String = prod.to_string();
                return Some(s);
            }
        }
    }
    Some(String::from("0"))
}

pub fn part2(input: String) -> Option<String> {
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
    for num in int.iter() {
        for num1 in int.iter() {
            for num2 in int.iter() {
                if num + num1 + num2 == 2020 {
                    println!("The numbers were: {}, {}, and {}", num, num1, num2);
                    let prod: i32 = num * num1 * num2;
                    println!("{}", prod);
                    let s: String = prod.to_string();
                    return Some(s);
                }
            }
        }
    }
    Some(String::from("0"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1721\n979\n366\n299\n675\n1456";
    #[test]
    fn test_part1() {
        assert_eq!(&part1(String::from(INPUT)).unwrap(), "514579");
    }
    #[test]
    fn test_part2() {
        assert_eq!(&part2(String::from(INPUT)).unwrap(), "241861950");
    }
}
