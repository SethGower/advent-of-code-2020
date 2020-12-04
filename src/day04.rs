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

const FIELDS: [&str;7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const COLORS: [&str;7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
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
    let mut valid : u32 = 0;
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
        let mut s = field.split(":");
        match s.next() {
            Some("byr") => {
                if let Some(num) = s.next() {
                    if num.len() == 4 {
                        if let Ok(num) = num.parse::<u32>() {
                            if num >= 1920 && num <= 2002 {
                                passport.birth_year = num;
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
            Some("iyr") => {
                if let Some(num) = s.next() {
                    if num.len() == 4 {
                        if let Ok(num) = num.parse::<u32>() {
                            if num >= 2010 && num <= 2020 {
                                passport.issue_year = num;
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
            Some("eyr") => {
                if let Some(num) = s.next() {
                    if num.len() == 4 {
                        if let Ok(num) = num.parse::<u32>() {
                            if num >= 2020 && num <= 2030 {
                                passport.exp_year = num;
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
            Some("hgt") => {
                if let Some(height) = s.next() {
                    if let Some(index) = height.find("in") {
                        if let Ok(num) = (height[0..index]).parse::<u32>() {
                            if num >= 59 && num <= 76 {
                                passport.height = (num, String::from("in"));
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else if let Some(index) = height.find("cm") {
                        if let Ok(num) = (height[0..index]).parse::<u32>() {
                            if num >= 150 && num <= 193 {
                                passport.height = (num, String::from("cm"));
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
            Some("hcl") => {
                if let Some(hair) = s.next() {
                    if hair[0..1] == *"#" && hair.len() == 7 {
                        // let hex = String::from(hair).replace("#","");
                        if let Ok(_) = u32::from_str_radix(&hair[1..], 16) {
                            passport.hair = String::from(hair);
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Some("ecl") => {
                if let Some(eye) = s.next() {
                    // if the eye color is in the list COLORS
                    if let Some(_) = COLORS.iter().find(|&&x| x == eye) {
                        passport.eye = String::from(eye);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Some("pid") => {
                if let Some(pid) = s.next() {
                    if let Ok(num) = pid.parse::<u32>() {
                        if pid.len() == 9 {
                            passport.passport_id = num;
                        }else{
                            return None;
                        }
                    }else{
                        return None;
                    }
                }else{
                    return None;
                }
            },
            _ => continue
        }
    }
    Some(passport)
}
