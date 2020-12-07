use std::collections::HashMap;
#[derive(Debug)]
struct Bag {
    color: String,
    contents: HashMap<String, u32>,
}

pub fn part1(input: String) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();

    let mut bags: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for line in lines.iter() {
        if let Some(bag) = parse_line(line) {
            bags.insert(bag.color, bag.contents);
        }
    }
    let count = count_bags(&"shiny gold", &mut bags);
    println!("{}", count);

    Some(count.to_string())
}

fn count_bags(color: &str, map: &mut HashMap<String, HashMap<String, u32>>) -> u32 {
    let mut count = 0;
    let map_clone = map.clone();
    let map_iter = map_clone.iter();
    for (key, val) in map_iter {
        if val.contains_key(color) {
            count += 1;
            map.remove(key);
            count += count_bags(key, map);
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
        let num = content[..num].parse::<u32>().ok()?;
        bag.contents.insert(String::from(color), num);
    }

    Some(bag)
}
