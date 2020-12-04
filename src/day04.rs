use std::u32;
#[derive(Debug)]
struct Passport {
    birth_year: u32,
    issue_year: u32,
    exp_year: u32,
    height: (u32, String),
    hair: String,
    eye: String,
    passport_id: u32,
    country_id: u32,
}

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
pub fn part1(input: String) {
    let vec: Vec<&str> = input.split("\n\n").collect();

    let mut invalid: usize = 0;
    for passport in vec.iter() {
        let s = String::from(*passport).replace("\n", " ");
        for pattern in FIELDS.iter() {
            if let None = s.find(pattern) {
                invalid += 1;
                break;
            }
        }
    }

    println!("Valid Passports: {}", vec.len() - invalid);
}

pub fn part2(input: String) {
    let vec: Vec<&str> = input.split("\n\n").collect();
    let mut valid: u32 = 0;
    for entry in vec.iter() {
        if let Some(_) = parse_passport(String::from(*entry).replace("\n", " ")) {
            valid += 1;
        }
    }
    println!("Valid Passports: {}", valid);
}

fn parse_passport(pass: String) -> Option<Passport> {
    for pattern in FIELDS.iter() {
        if let None = pass.find(pattern) {
            return None;
        }
    }
    // at this point, the passport has been verified to have all of the necessary fields, whether
    // they have valid data is yet to be determined
    let fields: Vec<&str> = pass.split_whitespace().collect();
    let mut passport = Passport {
        birth_year: 0,
        issue_year: 0,
        exp_year: 0,
        height: (0, String::with_capacity(0)),
        hair: String::with_capacity(0),
        eye: String::with_capacity(0),
        passport_id: 0,
        country_id: 0,
    };
    for field in fields.iter() {
        let s: Vec<&str> = field.split(":").collect();

        match &*s {
            ["byr", num] if num.len() == 4 => {
                let num: u32 = num
                    .parse::<u32>()
                    .ok()
                    .filter(|&it| it >= 1920 && it <= 2002)?;
                passport.birth_year = num;
            }
            ["iyr", num] if num.len() == 4 => {
                let num: u32 = num
                    .parse::<u32>()
                    .ok()
                    .filter(|&it| it >= 2010 && it <= 2020)?;
                passport.issue_year = num;
            }
            ["eyr", num] if num.len() == 4 => {
                let num: u32 = num
                    .parse::<u32>()
                    .ok()
                    .filter(|&it| it >= 2020 && it <= 2030)?;
                passport.exp_year = num;
            }
            ["hgt", height] => {
                if let Some(index) = height.find("in") {
                    let num: u32 = height[..index]
                        .parse::<u32>()
                        .ok()
                        .filter(|&it| it >= 59 && it <= 76)?;
                    passport.height = (num, String::from("in"));
                }else if let Some(index) = height.find("cm") {
                    let num: u32 = height[..index]
                        .parse::<u32>()
                        .ok()
                        .filter(|&it| it >= 150 && it <= 193)?;
                    passport.height = (num, String::from("cm"));
                }else {
                    return None;
                }
            },
            ["hcl", hair] if hair.len() == 7 && hair[0..1] == *"#" => {
                let _ : u32 = u32::from_str_radix(&hair[1..], 16).ok()?;
                passport.hair = String::from(*hair);
            },
            ["ecl", eye] => {
                let _ = COLORS.iter().find(|&x| x == eye)?;
                passport.eye = String::from(*eye);
            }
            ["pid", pid] if pid.len() == 9 => {
                passport.passport_id = pid.parse::<u32>().ok()?;
            }
            _ => continue,
        }
    }
    Some(passport)
}
