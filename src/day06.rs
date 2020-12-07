use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use std::thread;

const NUM_THREADS: usize = 4;

pub fn part1(input: String) -> Option<String> {
    fn count_answers(group: String) -> Option<usize> {
        let mut set: HashSet<char> = HashSet::new();

        for c in group.chars() {
            if c != '\n' {
                set.insert(c);
            }
        }
        Some(set.len())
    }
    let groups: Vec<&str> = input.split("\n\n").collect();
    let groups: Vec<String> = groups.into_iter().map(|x| x.to_string()).collect();
    let sum = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for chunk in groups.chunks(groups.len() / NUM_THREADS) {
        let sum = sum.clone();
        let groups = Vec::from(chunk);
        let handle = thread::spawn(move || {
            for group in groups {
                if let Some(num) = count_answers(group) {
                    if let Ok(mut sum) = sum.lock() {
                        *sum += num;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().ok()?;
    }

    let sum = *sum.lock().ok()?;
    println!("Sum: {}", sum);
    Some(sum.to_string())
}
pub fn part2(input: String) -> Option<String> {
    fn count_answers(s: String) -> Option<usize> {
        let people: Vec<&str> = s.split("\n").collect();
        let mut sets: Vec<HashSet<char>> = Vec::new();

        for person in people {
            if person.len() == 0 {
                continue;
            }
            let set: HashSet<char> = HashSet::from_iter(person.chars());
            sets.push(set);
        }

        let mut int: HashSet<char> = sets.get(0)?.clone();

        for set in sets {
            int = int.intersection(&set).cloned().collect();
        }

        Some(int.len())
    }
    let groups: Vec<&str> = input.split("\n\n").collect();
    let groups: Vec<String> = groups.into_iter().map(|x| x.to_string()).collect();
    let sum = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for chunk in groups.chunks(groups.len() / NUM_THREADS) {
        let sum = sum.clone();
        let groups = Vec::from(chunk);
        let handle = thread::spawn(move || {
            for group in groups {
                if let Some(num) = count_answers(group) {
                    if let Ok(mut sum) = sum.lock() {
                        *sum += num;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().ok()?;
    }

    let sum = *sum.lock().ok()?;
    println!("Sum: {}", sum);
    Some(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "abcx\nabcy\nabcz\n\n";
    const EXAMPLE_2: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n\n";
    #[test]
    #[ignore]
    fn test_part1() -> Result<(), String> {
        assert_eq!(
            &part1(String::from(EXAMPLE_1)).ok_or(String::from(""))?,
            "6"
        );
        assert_eq!(
            &part1(String::from(EXAMPLE_2)).ok_or(String::from(""))?,
            "11"
        );
        assert_eq!(
            &part2(String::from(EXAMPLE_1)).ok_or(String::from(""))?,
            "3"
        );
        assert_eq!(
            &part2(String::from(EXAMPLE_2)).ok_or(String::from(""))?,
            "6"
        );

        Ok(())
    }
}
