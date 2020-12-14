use itertools::izip;
use rayon::prelude::*;
use std::cmp::Ordering;
#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct Bus {
    id: isize,
    time: isize,
}
impl Ord for Bus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
impl Bus {
    // Implements the Chinese Remainder Theorem to compute the first timestamp with
    // contiguous departures
    fn find_contiguous_crt(schedule: &[Bus]) -> Option<isize> {
        #[inline]
        // find value with inverse modulus of a with mod value of m
        fn inverse_mod(a: isize, m: isize) -> Option<isize> {
            (0..m).into_iter().find(|x| a * x % m == 1)
        }
        let prod: isize = schedule.iter().map(|x| x.id).product(); // product of all of the id's
        let factors: Vec<isize> = schedule.iter().map(|x| prod / x.id).collect(); // the factors for the product for each of the id's
        let y: Vec<isize> = schedule // shows all of the inverse modulus values for the factors
            .iter()
            .zip(factors.clone())
            .map(|(b, f)| inverse_mod(f, b.id).unwrap())
            .collect();
        let mod_prods: isize = izip!(schedule, &factors, &y) // sum of the inverse mod values * factors and id-offset
            .map(|(b, f, i)| {
                let x = (b.id - b.time) % b.id;
                x * f * i
            })
            .sum();

        let result = mod_prods % prod;
        Some(result)
    }
    #[allow(dead_code)]
    // this doesn't work. I ran it on the server for 204 hours (of CPU time, overnight on 16 threads)
    fn find_contiguous_brut(schedule: &[Bus]) -> Option<isize> {
        let min_bus = schedule.iter().max()?;
        let start: isize = min_bus.id - min_bus.time;
        let step: isize = min_bus.id;
        let curr_time: isize = (0..=100_000_000_000_000 as isize)
            .into_par_iter()
            .find_first(|iter| {
                let answer = iter * step + start;
                schedule.iter().all(|b| (b.time + answer) % b.id == 0)
            })?
            * step
            + start;
        Some(curr_time)
    }
    fn print_table(schedule: &[Bus], time: usize, entries: usize) -> Option<String> {
        let time_width = (time + entries).to_string().len();
        let bus_width = schedule.iter().max()?.id.to_string().len() + 7;

        let mut string = String::new();
        let mut header = format!("{:<width$}", "time", width = time_width + 4);
        for bus in schedule.iter() {
            let col = format!("bus {}", bus.id); // adds the column header for each bus
            header += &format!("{:^width$}", col, width = bus_width); //formats the width of the column
        }
        string += &header;
        // Iterates through each row and adds the rows to the string
        for num in time..time + entries {
            // adds the row with the time stamp in the first column
            let mut row = format!("\n{:<width$}", num, width = time_width + 4);
            // iterates through each bus in the schedule.
            for bus in schedule.iter() {
                if num % bus.id as usize == 0 {
                    // if the bus departs at that time stamp, write a 'D' in the column
                    row += &format!("{:^width$}", "D", width = bus_width);
                } else {
                    // if the bus doesn't depart at that time stamp, write a '-' in the column
                    row += &format!("{:^width$}", "-", width = bus_width);
                }
            }
            string += &row;
        }
        Some(string)
    }
}

pub fn part1(input: String) -> Option<String> {
    let mut lines = input.lines();
    let timestamp: isize = lines.next()?.parse().ok()?;
    let earliest_departure: Bus = lines
        .next()?
        .split(',')
        .filter_map(|x| x.parse::<isize>().ok())
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
    let offsets: Vec<Bus> = lines
        .nth(1)?
        .split(',')
        .enumerate()
        .filter_map(|x| {
            if let Ok(num) = x.1.parse::<isize>() {
                Some(Bus {
                    id: num,
                    time: x.0 as isize,
                })
            } else {
                None
            }
        })
        .collect();
    let result = Bus::find_contiguous_crt(&offsets)?;
    println!("First Timestamp with contiguous departures: {}", result);
    println!(
        "Departures:\n{}",
        Bus::print_table(
            &offsets,
            result as usize - 2,
            offsets.iter().map(|x| x.time).max()? as usize + offsets.len()
        )?
    );

    Some(result.to_string())
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
