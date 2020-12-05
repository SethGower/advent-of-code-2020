#[derive(Debug)]
struct Seat {
    row: u32,
    col: u32,
}
pub fn part1(input: String) -> Option<String> {
    // let mut seats: Vec<Seat> = Vec::new();
    let mut highest_id: u32 = 0;

    for seat in input.split("\n") {
        let id: u32 = match get_seat(seat) {
            Some(seat) => {
                seat.row * 8 + seat.col
            }
            None => 0,
        };
        if id > highest_id {
            highest_id = id;
        }
    }
    println!("Highest: {}", highest_id);
    Some(highest_id.to_string())
}

fn get_seat(s: &str) -> Option<Seat> {
    let mut seat = Seat { row: 0, col: 0 };

    if s.len() == 0 {
        return None
    }
    seat.row = find_row(&s[..7], (0, 127));
    seat.col = find_col(&s[7..], (0, 7));
    Some(seat)
}
fn find_row(s: &str, range: (u32, u32)) -> u32 {
    let mut lower: u32 = range.0;
    let mut upper: u32 = range.1;
    if s.len() > 1 {
        if s[0..1] == *"F" {
            upper = (upper + lower) / 2;
            find_row(&s[1..], (lower, upper))
        } else if s[0..1] == *"B" {
            lower = (upper + lower) / 2 + 1;
            find_row(&s[1..], (lower, upper))
        } else {
            0
        }
    } else {
        if s[0..1] == *"F" {
            lower
        } else if s[0..1] == *"B" {
            upper
        } else {
            0
        }
    }
}

fn find_col(s: &str, range: (u32, u32)) -> u32 {
    let mut lower: u32 = range.0;
    let mut upper: u32 = range.1;
    if s.len() > 1 {
        if s[0..1] == *"L" {
            upper = (upper + lower) / 2;
            find_col(&s[1..], (lower, upper))
        } else if s[0..1] == *"R" {
            lower = (upper + lower) / 2 + 1;
            find_col(&s[1..], (lower, upper))
        } else {
            0
        }
    } else {
        if s[0..1] == *"L" {
            lower
        } else if s[0..1] == *"R" {
            upper
        } else {
            0
        }
    }
}
