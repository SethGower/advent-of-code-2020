use std::collections::HashMap;
use std::u64;
struct Mask {
    set_mask: u64,
    clear_mask: u64,
}

impl Mask {
    fn new(s: &str) -> Option<Mask> {
        Some(Mask {
            set_mask: u64::from_str_radix(&s.replace("X", "0"), 2).ok()?,
            clear_mask: u64::from_str_radix(&s.replace("X", "1"), 2).ok()?,
        })
    }

    fn process_num(&self, num: u64) -> u64 {
        num & self.clear_mask | self.set_mask
    }
}

pub fn part1(input: String) -> Option<String> {
    let mut curr_mask = Mask {
        set_mask: 0,
        clear_mask: 0,
    };
    let mut memory: HashMap<String, u64> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("=").map(|x| x.trim()).collect();
        match &*parts {
            ["mask", val] => {
                curr_mask = Mask::new(val)?;
            }
            [cmd, val] => {
                let num = curr_mask.process_num(val.parse::<u64>().ok()?);
                memory.insert(String::from(*cmd), num);
            }
            _ => continue,
        }
    }
    let sum = memory.values().sum::<u64>();
    println!("{}", sum);
    Some(sum.to_string())
}
pub fn part2(input: String) -> Option<String> {
    Some(input)
}
