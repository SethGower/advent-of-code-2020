use std::thread;
// use std::sync::Mutex;
use std::sync::Arc;

pub fn part1(input: String) {
    let matrix: Vec<&str> = input.split('\n').collect();
    let matrix : Vec<String> = matrix.into_iter().map(|it| it.to_string()).collect();
    let num_trees: usize = check_paths(&matrix, 3, 1);
    println!("Trees: {}", num_trees);
}

pub fn part2(input: String) {
    let matrix: Vec<&str> = input.split('\n').collect();
    let cloned : Vec<String> = matrix.into_iter().map(|it| it.to_string()).collect();
    let atomic_matrix = Arc::new(cloned);
    let mut handles = vec![];
    {
        let tup: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        for range in tup.iter() {
            let atomic_matrix = Arc::clone(&atomic_matrix);
            let lower = range.0;
            let upper = range.1;
            let handle = thread::spawn(move || {
                let trees = check_paths(&(*atomic_matrix), lower, upper);
                println!("Range: ({},{}) had {} trees", lower, upper, trees);
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
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
