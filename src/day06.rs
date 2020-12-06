use std::collections::HashSet;
use std::iter::FromIterator;
pub fn part1(input: String) -> Option<String> {
    fn count_answers(group: &str) -> usize {
        let mut set: HashSet<char> = HashSet::new();

        for c in group.chars() {
            if c != '\n' {
                set.insert(c);
            }
        }
        set.len()
    }
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut sum = 0;
    for group in groups.iter() {
        sum += count_answers(group);
    }

    println!("Sum: {}", sum);
    Some(sum.to_string())
}
pub fn part2(input : String) -> Option<String> {
    fn count_answers(s : &str) -> Option<usize> {
        let people : Vec<&str> = s.split("\n").collect();
        let mut sets : Vec<HashSet<char>> = Vec::new();

        for person in people {
            if person.len() == 0 {
                continue;
            }
            let set : HashSet<char> = HashSet::from_iter(person.chars());
            sets.push(set);
        }

        let mut int : HashSet<char> = sets.get(0)?.clone();

        for set in sets {
            int = int.intersection(&set).cloned().collect();
        }
        
        Some(int.len())
    }
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut sum = 0;
    for group in groups.iter() {
        if let Some(num) = count_answers(group){
            sum += num;
        }
    }

    println!("Sum: {}", sum);
    Some(sum.to_string())
}
