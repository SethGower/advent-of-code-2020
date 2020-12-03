pub fn part1(input: String){
    let matrix : Vec<&str> = input.split('\n').collect();
    let numCol = match matrix.get(0) {
        Some(row) => row.len(),
        _ => 0
    };

}
