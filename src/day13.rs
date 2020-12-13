use rayon::prelude::*;
use std::cmp::Ordering;
use std::u8;
#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct Bus {
    id: usize,
    time: usize,
}
impl Ord for Bus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

pub fn part1(input: String) -> Option<String> {
    let mut lines = input.lines();
    let timestamp: usize = lines.next()?.parse().ok()?;
    let earliest_departure: Bus = lines
        .next()?
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .map(|id| {
            let timestamp = ((timestamp / id) + 1) * id;
            Bus {
                id: id,
                time: timestamp,
            }
        })
        .min()?;
    println!("{:?}", earliest_departure);
    let resp = earliest_departure.id * (earliest_departure.time - timestamp);
    println!("{}", resp);
    Some(resp.to_string())
}
pub fn part2(input: String) -> Option<String> {
    let mut lines = input.lines();
    let _ = lines.next()?;
    let offsets: Vec<Bus> = lines
        .next()?
        .split(',')
        .enumerate()
        .filter_map(|x| {
            if let Ok(num) = x.1.parse::<usize>() {
                Some(Bus { id: num, time: x.0 })
            } else {
                None
            }
        })
        .collect();
    let mut curr_time: usize = 0;
    let min_id = offsets.iter().min()?.id;
    loop {
        if offsets.par_iter().all(|b| (b.time + curr_time) % b.id == 0) {
            break;
        }
        curr_time += min_id as usize;
    }
    println!("{}", curr_time);
    Some(input)
}
