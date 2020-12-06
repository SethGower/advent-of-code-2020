use std::cmp::Ordering;
use std::u32;

use std::collections::BTreeMap;
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd)]
struct Seat {
    row: u32,
    col: u32,
}
impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.row == other.row {
            self.col.cmp(&other.col)
        } else {
            self.row.cmp(&other.row)
        }
    }
}

pub fn part1(input: String) -> Option<String> {
    let mut highest_id: u32 = 0;

    for seat in input.split("\n") {
        let id: u32 = match get_seat(seat) {
            Some(seat) => seat.row * 8 + seat.col,
            None => 0,
        };
        if id > highest_id {
            highest_id = id;
        }
    }
    println!("Highest: {}", highest_id);
    Some(highest_id.to_string())
}
pub fn part2(input: String) -> Option<String> {
    let mut occupied_seats: BTreeMap<u32, Seat> = BTreeMap::new();
    let mut empty_seats: BTreeMap<u32, Seat> = BTreeMap::new();
    let mut highest_id: u32 = 0;

    for seat in input.split("\n") {
        if let Some(seat) = get_seat(seat) {
            let id = seat.row * 8 + seat.col;
            occupied_seats.insert(id, seat);
            if id > highest_id {
                highest_id = id;
            }
        }
    }

    for i in 0..=127 {
        for j in 0..=7 {
            let seat = Seat { row: i, col: j };
            let id = seat.row * 8 + seat.col;
            if !occupied_seats.contains_key(&id) {
                empty_seats.insert(id, seat);
            }
        }
    }

    for (id, _) in &empty_seats {
        if *id == 0 || *id == highest_id {
            continue;
        }
        if occupied_seats.contains_key(&(id - 1)) && occupied_seats.contains_key(&(id + 1)) {
            return Some(id.to_string());
        }
    }

    None
}

// Finds a seat given a string. String must be composed of 7 characters that are F or B (front or
// back) and 3 R or L (Right or Left). These describe which row and column, respectively. This
// performs a binary search
fn get_seat(s: &str) -> Option<Seat> {
    let mut seat = Seat { row: 0, col: 0 };

    if s.len() == 0 {
        return None;
    }
    let s = String::from(s)
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    seat.row = u32::from_str_radix(&s[..7], 2).ok()?;
    seat.col = u32::from_str_radix(&s[7..], 2).ok()?;
    Some(seat)
}
