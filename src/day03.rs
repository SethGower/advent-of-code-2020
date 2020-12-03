pub fn part1(input: String) {
    let matrix: Vec<&str> = input.split('\n').collect();
    let num_col = match matrix.get(0) {
        Some(row) => row.len(),
        _ => 0,
    };
    let mut num_trees: usize = 0;
    let mut curr_col: usize = 0;

    for row in matrix.iter() {
        if row.len() > 0 {
            curr_col = curr_col % num_col;
            let row_s = String::from(*row);
            if let Some(character) = row_s.get(curr_col..curr_col+1){
                if character == "#" {
                    num_trees += 1;
                }
            }
            curr_col += 3;
        } else {
            break;
        }
    }
    println!("Trees: {}", num_trees);
}
