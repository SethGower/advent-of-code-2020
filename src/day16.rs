use std::ops::RangeInclusive;
#[derive(Debug)]
struct Rule {
    name: String,
    valid: Vec<RangeInclusive<i32>>,
}

impl Rule {
    fn new(s: &str) -> Option<Rule> {
        let name = s.splitn(2, ":").nth(0)?;

        let parts = s.splitn(2, ':').map(|x| x.trim()).nth(1)?;
        let valid: Vec<RangeInclusive<i32>> = parts
            .split("or")
            .map(|s| s.trim())
            .map(|s| {
                let s: Vec<&str> = s.split('-').collect();
                s[0].parse().unwrap()..=s[1].parse().unwrap()
            })
            .collect();
        Some(Rule {
            name: String::from(name),
            valid,
        })
    }
}
impl std::str::FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Rule::new(s).ok_or(())
    }
}
#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}
impl Rules {
    fn check_line(&self, line: &str) -> Option<&str> {
        line.split(',').all(|s| {
            let num = s.parse::<i32>().unwrap();
            self.rules.iter().any(|x| {
                let valid = &x.valid;
                valid.iter().any(|r| r.contains(&num))
            })
        });
        None
    }
}
pub fn part1(input: String) -> Option<String> {
    let mut sections = input.split("\n\n");

    let rules = sections.nth(0)?.lines();
    let rules = Rules {
        rules: rules.filter_map(|s| s.parse::<Rule>().ok()).collect(),
    };

    let other_tickets: Vec<&str> = sections.nth(1)?.lines().collect();

    Some(input)
}
pub fn part2(input: String) -> Option<String> {
    Some(input)
}
