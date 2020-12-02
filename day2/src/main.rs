use std::fs::File;
use std::str::FromStr;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use regex::Regex;
use lazy_static::lazy_static;

// Prevent compiling the regex for each .parse::PwdEntry
lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<chr>\w): (?P<pwd>\w+)").unwrap();
}

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("===== Part 1 ====");
    part1(&input);
    println!("===== Part 2 ====");
    part2(&input);
}

struct PwdEntry {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl FromStr for PwdEntry {
    type Err = std::io::Error;
    fn from_str(str_entry: &str) -> Result<Self, Self::Err> {
        let cap = RE.captures(str_entry).unwrap();

        Ok(PwdEntry { 
            min     : cap.name("min").unwrap().as_str().parse().unwrap(), 
            max     : cap.name("max").unwrap().as_str().parse().unwrap(), 
            ch      : cap.name("chr").unwrap().as_str().chars().next().unwrap(),
            password: cap.name("pwd").unwrap().as_str().to_string(),
        })
    }
}

fn read_input(path: &str) -> Result<Vec<PwdEntry>, Error> {
    let br = BufReader::new(File::open(path)?);
    br.lines().map(|line| 
                   line.and_then(|entry| entry.parse::<PwdEntry>()
                   .map_err(|e| Error::new(ErrorKind::InvalidData, e))))
              .collect()
}

fn part1(input: &Vec<PwdEntry>) {
    let mut count = 0;
    for pwd in input {
        let m = pwd.password.matches(pwd.ch).count();
        if pwd.min <= m && m <= pwd.max {
            count += 1;
        }
    }
    println!("Found {} valid passwords.", count);
}

fn part2(input: &Vec<PwdEntry>) {
    let mut count = 0;
    for pwd in input {
        let mut valid = false;
        if let Some(c) = pwd.password.chars().nth(pwd.min - 1) {
            valid ^= c == pwd.ch;
        }
        if let Some(c) = pwd.password.chars().nth(pwd.max - 1) {
            valid ^= c == pwd.ch;
        }
        if valid {
            count += 1;
        }
    }
    println!("Found {} valid passwords.", count);
}
