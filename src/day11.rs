#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Spot {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Eq)]
struct Matrix {
    rows: usize,
    cols: usize,
    payload: Vec<Vec<Spot>>,
}
impl Matrix {
    fn new(vec: Vec<&str>) -> Matrix {
        let mut mat = Matrix {
            rows: 0,
            cols: 0,
            payload: Vec::new(),
        };

        for row in vec.iter() {
            let mut r: Vec<Spot> = Vec::new();
            for c in (*row).chars() {
                match c {
                    '.' => r.push(Spot::Floor),
                    'L' => r.push(Spot::Empty),
                    '#' => r.push(Spot::Occupied),
                    _ => continue,
                }
            }
            mat.payload.push(r);
        }
        mat.rows = vec.len();
        mat.cols = vec[0].len();
        mat
    }
    fn get(&self, row: usize, col: usize) -> Option<&Spot> {
        if row < self.rows && col < self.cols {
            Some(self.payload.get(row)?.get(col)?)
        } else {
            None
        }
    }
    fn set(&mut self, s: Spot, row: usize, col: usize) -> Option<Spot> {
        let old = *self.get(row, col)?;
        if row < self.rows && col < self.cols {
            let r = self.payload.get_mut(row)?;
            *r.get_mut(col)? = s;
        }
        Some(old)
    }
    fn process(&mut self) -> bool {
        let mut new_payload = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                match self.get(i, j) {
                    Some(Spot::Floor) => {
                        new_payload.set(Spot::Floor, i, j);
                    }
                    Some(Spot::Empty) => {
                        if self.count_neighors(i, j) == 0 {
                            new_payload.set(Spot::Occupied, i, j);
                        }
                    }
                    Some(Spot::Occupied) => {
                        if self.count_neighors(i, j) >= 4 {
                            new_payload.set(Spot::Empty, i, j);
                        }
                    }
                    _ => continue,
                }
            }
        }
        let same = new_payload == *self;
        self.payload = new_payload.payload;
        same
    }
    fn count_neighors(&self, row: usize, col: usize) -> usize {
        let mut neighbors = 0;
        for i in -1..=1 as i32 {
            for j in -1..=1 as i32 {
                let x = i + row as i32;
                let y = j + col as i32;
                if x < 0
                    || y < 0
                    || x >= self.rows as i32
                    || y >= self.cols as i32
                    || (i == 0 && j == 0)
                {
                    continue;
                }
                // at this point, the coodinates are guaranteed to be valid
                if *self.get(x as usize, y as usize).unwrap() == Spot::Occupied {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }
    fn get_occupied(&self) -> usize {
        self.count_type(Spot::Occupied)
    }
    fn count_type(&self, s: Spot) -> usize {
        let mut sum = 0;
        for row in self.payload.iter() {
            for col in row.iter() {
                if *col == s {
                    sum += 1;
                }
            }
        }
        sum
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.payload == other.payload
    }
}

pub fn part1(input: String) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut mat = Matrix::new(lines);
    while !mat.process() {}
    println!("{}", mat.get_occupied());
    Some(mat.get_occupied().to_string())
}
