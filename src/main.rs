use aocf::{Aoc, Level};
use getopts::Options;
use std::env;
use std::fs;
use std::io;
use std::time::{Duration, Instant};

use advent_of_code::{get_day, noop};

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}

fn main() {
    // Get day string
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }

    let mut opts = Options::new();
    opts.optflag("s", "submit", "Submit the current puzzle");
    opts.optopt("f", "file", "", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!(e.to_string());
        }
    };

    // Parse day as number
    day = day.trim().to_string();
    let day_num: u32 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };
    // Get corresponding function

    let to_run = get_day(day_num);
    if let Ok(mut aoc) = Aoc::new().year(Some(2020)).day(Some(day_num)).init() {
        if let Ok(mut input) = aoc.get_input(false) {
            if matches.opt_present("f") {
                if let Some(filename) = matches.opt_str("f") {
                    input = fs::read_to_string(filename).expect("Unable to open file");
                }
            }
            let (output1, output2) = run_funcs(to_run, input);
            if matches.opt_present("s") {
                match aoc.level {
                    Level::First => {
                        if let Some(output) = output1 {
                            println!("Submitting {} for Day {}, Level {}", output, day_num, 1);
                            match aoc.submit(&output) {
                                Ok(resp) => println!("{}", resp),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        } else {
                            println!("Output for Day {}, Level {} was empty", day_num, 1);
                        }
                    }
                    Level::Second => {
                        if let Some(output) = output2 {
                            println!("Submitting {} for Day {}, Level {}", output, day_num, 2);
                            match aoc.submit(&output) {
                                Ok(resp) => println!("{}", resp),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        } else {
                            println!("Output for Day {}, Level {} was empty", day_num, 2);
                        }
                    }
                }
            }
        } else {
            println!("Unable to get Input. Make sure session cookie is valid")
        }
    } else {
        if matches.opt_present("f") {
            if let Some(filename) = matches.opt_str("f") {
                if let Ok(input) = fs::read_to_string(filename) {
                    run_funcs(to_run, input);
                }
            }
        } else {
            eprintln!("Please specify an input file with '--file|-f FILE'");
        }
    }
}
fn run_funcs(
    to_run: (advent_of_code::DayFn, advent_of_code::DayFn),
    input: String,
) -> (Option<String>, Option<String>) {
    let mut res1: Option<String> = None;
    let mut res2: Option<String> = None;
    if to_run.0 != noop {
        println!("Running Part 1");
        let part1_start = Instant::now();
        res1 = to_run.0(input.clone());
        let part1_dur = part1_start.elapsed();
        println!("Took {}", fmt_dur(part1_dur));
    }

    if to_run.1 != noop {
        println!("Running Part 2");
        let part2_start = Instant::now();
        res2 = to_run.1(input.clone());
        let part2_dur = part2_start.elapsed();
        println!("Took {}", fmt_dur(part2_dur));
    }
    (res1, res2)
}
