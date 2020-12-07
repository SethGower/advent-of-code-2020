use std::cmp::Ordering;
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
    seat.row = find_row(&s[..7], (0, 127));
    seat.col = find_col(&s[7..], (0, 7));
    Some(seat)
}
// Binary search using the a string of F's and B's
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
// Binary search using the a string of R's and L's
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

#[cfg(test)]
mod tests {
    use super::*;
    const SINGLE: &str = "FBFBBFFRLR";
    const MULTI: &str = "BFFFBFBLRR\nBFFFBFBRLR";

    #[test]
    fn test_part1() -> Result<(), String> {
        assert_eq!(&part1(String::from(SINGLE)).ok_or("Part1 Failed")?, "357");
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<(), String> {
        assert_eq!(&part1(String::from(MULTI)).ok_or("Part1 Failed")?, "557");
        Ok(())
    }
}
