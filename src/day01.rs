pub fn part1(inp: String) {
    let v: Vec<&str> = inp.split('\n').collect();
    let mut int: Vec<i32> = Vec::new();
    for number in v.iter() {
        let temp = match number.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        int.push(temp);
    }

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
        if done {
            break;
        }
    }
}
