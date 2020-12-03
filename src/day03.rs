pub fn part1(input: String) {
    let matrix: Vec<&str> = input.split('\n').collect();
    let num_trees: usize = check_paths(&matrix, 3, 1);
    println!("Trees: {}", num_trees);
}

pub fn part2(input: String) {
    let matrix: Vec<&str> = input.split('\n').collect();
    let mut vec : Vec<usize> = Vec::with_capacity(5);

    vec.push(check_paths(&matrix, 1,1));
    vec.push(check_paths(&matrix, 3,1));
    vec.push(check_paths(&matrix, 5,1));
    vec.push(check_paths(&matrix, 7,1));
    vec.push(check_paths(&matrix, 1,2));

    let mut prod : usize = 1;

    for num in vec.iter(){
        prod *= num;
    }
    println!("Product: {}", prod);
}


fn check_paths(matrix: &Vec<&str>, right: usize, down: usize) -> usize {
    let num_col = match matrix.get(0) {
        Some(row) => row.len(),
        _ => 0,
    };
    let mut num_trees: usize = 0;
    let mut curr_col: usize = 0;

    let mut i: usize = 0;
    while i < matrix.len() {
        if let Some(row) = matrix.get(i) {
            let row_s = String::from(*row);
            if let Some(character) = row_s.get(curr_col..curr_col + 1) {
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
