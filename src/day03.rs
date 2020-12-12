use std::sync::{Arc, Mutex};
use std::thread;

pub fn part1(input: String) -> Option<String> {
    let matrix: Vec<&str> = input.split('\n').collect();
    let matrix: Vec<String> = matrix.into_iter().map(|it| it.to_string()).collect();
    let num_trees: usize = check_paths(&matrix, 3, 1);
    println!("Trees: {}", num_trees);
    Some(num_trees.to_string())
}

pub fn part2(input: String) -> Option<String> {
    let matrix: Vec<&str> = input.split('\n').collect();
    let matrix: Vec<String> = matrix.into_iter().map(|it| it.to_string()).collect();
    let matrix = Arc::new(matrix);

    let product = Arc::new(Mutex::new(1));
    let mut handles = vec![];
    {
        let tup: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        for range in tup.iter() {
            let matrix = Arc::clone(&matrix);
            let product = Arc::clone(&product);
            let lower = range.0;
            let upper = range.1;
            let handle = thread::spawn(move || {
                let trees = check_paths(&(*matrix), lower, upper);
                // println!("Range: ({},{}) had {} trees", lower, upper, trees);
                if let Ok(mut prod) = product.lock() {
                    *prod *= trees;
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let prod = *product.lock().unwrap();
    println!("Product: {}", prod);
    Some(prod.to_string())
}

fn check_paths(matrix: &Vec<String>, right: usize, down: usize) -> usize {
    let num_col = match matrix.get(0) {
        Some(row) => row.len(),
        _ => 0,
    };
    let mut num_trees: usize = 0;
    let mut curr_col: usize = 0;

    let mut i: usize = 0;
    while i < matrix.len() {
        if let Some(row) = matrix.get(i) {
            if let Some(character) = row.get(curr_col..curr_col + 1) {
                if character == "#" {
                    num_trees += 1;
                }
            }
            curr_col += right;
            curr_col %= num_col;
            i += down;
        }
    }

    num_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT : &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
    #[test]
    fn test_part1() {
        assert_eq!(&part1(String::from(INPUT)).unwrap(), "7");
    }
    #[test]
    fn test_part2() {
        assert_eq!(&part2(String::from(INPUT)).unwrap(), "336");
    }
}
