use rayon::prelude::*;
use std::cmp::Ordering;
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
    let min_id: usize = offsets.iter().min()?.id;
    let curr_time: usize = (100000000000000..std::usize::MAX / 2 as usize)
        .into_par_iter()
        .find_first(|x| {
            let answer = min_id * x;
            offsets.iter().all(|b| (b.time + answer) & b.id == 0)
        })?;
    println!("{}", curr_time);
    Some(curr_time.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "939\n7,13,x,x,59,x,31,19";
    const INPUT2: &str = "\n17,x,13,19";
    const INPUT3: &str = "\n67,7,59,61";
    const INPUT4: &str = "\n67,x,7,59,61";
    const INPUT5: &str = "\n67,7,x,59,61";
    const INPUT6: &str = "\n1789,37,47,1889";
    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from(INPUT1)), Some(String::from("295")));
    }
    #[test]
    fn test_part2() {
        assert_eq!(&part2(String::from(INPUT1)).unwrap(), "1068781");
        assert_eq!(&part2(String::from(INPUT2)).unwrap(), "3417");
        assert_eq!(&part2(String::from(INPUT3)).unwrap(), "754018");
        assert_eq!(&part2(String::from(INPUT4)).unwrap(), "779210");
        assert_eq!(&part2(String::from(INPUT5)).unwrap(), "1261476");
        assert_eq!(&part2(String::from(INPUT6)).unwrap(), "1202161486");
    }
}
