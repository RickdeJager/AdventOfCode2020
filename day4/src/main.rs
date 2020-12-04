use regex::Regex;
use lazy_static::lazy_static;

// Prevent compiling the regex for each .parse::PwdEntry
lazy_static! {
    static ref HCLRE: Regex = Regex::new(r"\bhcl:#[0-9a-f]{6}\b").unwrap();
    static ref PIDRE: Regex = Regex::new(r"\bpid:\d{9}\b").unwrap();
}


fn read() -> Vec<String>{
    include_str!("input.txt")
        .trim()
        .split("\n\n")
        .map(|l| l.to_string().replace("\n", " "))
        .collect()
}

fn check_presence(passport: &String) -> bool {
    let reqs = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for req in reqs {
        if ! passport.contains(req) {
            return false;
        }
    }
    true
}

fn check_byr(passport: &String) -> bool {
    let byr: &str = passport.split("byr:")
        .nth(1).unwrap().split(' ').next().unwrap();
    let x = byr.parse().unwrap_or(-1);
    1920 <= x && x <= 2002
}

fn check_iyr(passport: &String) -> bool {
    let iyr: &str = passport.split("iyr:")
        .nth(1).unwrap().split(' ').next().unwrap();
    let x = iyr.parse().unwrap_or(-1);
    2010 <= x && x <= 2020
}

fn check_eyr(passport: &String) -> bool {
    let eyr: &str = passport.split("eyr:")
        .nth(1).unwrap().split(' ').next().unwrap();
    let x = eyr.parse().unwrap_or(-1);
    2020 <= x && x <= 2030
}

fn check_hgt(passport: &String) -> bool {
    let hgt: &str = passport.split("hgt:")
        .nth(1).unwrap().split(' ').next().unwrap();
    let unit = &hgt[hgt.len()-2..];
    let val  = &hgt[..hgt.len()-2];
    let x = val.parse().unwrap_or(-1);
    if unit == "in" {
        return 59 <= x && x <= 76;
    } else if unit == "cm" {
        return 150 <= x && x <= 193;
    }
    false
}

fn check_hcl(passport: &String) -> bool {
    HCLRE.is_match(passport)
}

fn check_pid(passport: &String) -> bool {
    PIDRE.is_match(passport)
}


fn check_ecl(passport: &String) -> bool {
    let cols = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let col: &str = passport.split("ecl:")
        .nth(1).unwrap().split(' ').next().unwrap();
    cols.contains(&col)
}


fn part1(passports: &Vec<String>) -> usize {
    let mut i = 0;
    for passport in passports {
        if check_presence(passport) {
            i += 1;
        }
    }
    i
}

fn part2(passports: &Vec<String>) -> usize {
    let mut i = 0;
    for passport in passports {
        if check_presence(passport) &&
            check_byr(passport)     &&
            check_iyr(passport)     &&
            check_eyr(passport)     &&
            check_hgt(passport)     &&
            check_hcl(passport)     &&
            check_ecl(passport)     &&
            check_pid(passport)     {
            i += 1;
        }
    }
    i
}

fn main() {
    let passports = read();
    println!("===== Part 1 ====");
    println!("Found {} valid passports", part1(&passports));
    println!("===== Part 2 ====");
    println!("Found {} valid passports", part2(&passports));
}
