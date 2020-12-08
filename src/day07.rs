use std::collections::HashMap;
#[derive(Debug)]
struct Bag {
    color: String,
    contents: HashMap<String, usize>,
}

pub fn part1(input: String) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();

    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for line in lines.iter() {
        if let Some(bag) = parse_line(line) {
            bags.insert(bag.color, bag.contents);
        }
    }
    // println!("{:#?}", bags);
    let count = count_bag_holders("shiny gold", &bags).len();
    println!("{}", count);

    Some(count.to_string())
}
fn count_bag_holders(color: &str, map: &HashMap<String, HashMap<String, usize>>) -> Vec<String> {
    let mut bags: Vec<String> = Vec::with_capacity(0);

    for (k, v) in map {
        if v.contains_key(color) {
            // println!("{} can hold {}", k, color);
            for string in count_bag_holders(k, map) {
                if !bags.contains(&string) {
                    bags.push(string);
                }
            }
            if !bags.contains(k) {
                bags.push(k.to_string());
            }
        }
    }

    bags
}

pub fn part2(input: String) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();

    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for line in lines.iter() {
        if let Some(bag) = parse_line(line) {
            bags.insert(bag.color, bag.contents);
        }
    }
    let count = count_bag_contents("shiny gold", &bags);
    println!("{}", count);

    Some(count.to_string())
}
fn count_bag_contents(color: &str, map: &HashMap<String, HashMap<String, usize>>) -> usize {
    let mut count = 0;
    if let Some(contents) = map.get(color) {
        for (k, v) in contents {
            let content = (1 + count_bag_contents(k, map)) * *v;
            count += content;
        }
    }
    count
}

fn parse_line(line: &str) -> Option<Bag> {
    let top: &str = line
        .splitn(2, "bag")
        .map(|x| x.trim())
        .collect::<Vec<&str>>()[0];
    let contents: &str = line
        .splitn(2, "contain")
        .map(|x| x.trim().trim_matches('.'))
        .collect::<Vec<&str>>()[1];

    let mut bag = Bag {
        color: String::from(top),
        contents: HashMap::new(),
    };
    for content in contents.split(",").map(|x| x.trim()) {
        let num = content.find(" ")?;
        let color = content[num..].splitn(2, " ").collect::<Vec<&str>>()[1]
            .split("bag")
            .map(|x| x.trim())
            .collect::<Vec<&str>>()[0];
        let num = content[..num].parse::<usize>().ok()?;
        bag.contents.insert(String::from(color), num);
    }

    Some(bag)
}
