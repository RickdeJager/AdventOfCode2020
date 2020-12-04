use regex::Regex;
use lazy_static::lazy_static;

// Prevent compiling the regex for each check
lazy_static! {
    static ref BYRRE: Regex = Regex::new(r"\bbyr:(19[2-9]\d|200[0-2])\b").unwrap();
    static ref IYRRE: Regex = Regex::new(r"\biyr:20(1\d|20)\b").unwrap();
    static ref EYRRE: Regex = Regex::new(r"\beyr:20(2\d|30)\b").unwrap();
    static ref HGTRE: Regex = Regex::new(r"\bhgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\b").unwrap();
    static ref HCLRE: Regex = Regex::new(r"\bhcl:#[0-9a-f]{6}\b").unwrap();
    static ref ECLRE: Regex = Regex::new(r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
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
        if  BYRRE.is_match(passport)     &&
            IYRRE.is_match(passport)     &&
            EYRRE.is_match(passport)     &&
            HGTRE.is_match(passport)     &&
            HCLRE.is_match(passport)     &&
            ECLRE.is_match(passport)     &&
            PIDRE.is_match(passport)     {
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
