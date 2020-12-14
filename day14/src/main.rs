use regex::Regex;
use std::collections::HashMap;

fn part1() -> u64 {
    let mut mem = HashMap::new();
    let re = Regex::new(r"mem\[(?P<idx>\d+)\] = (?P<num>\d+)").unwrap();

    let mut mask_or : u64 = 0;
    let mut mask_and: u64 = 0;
    for line in include_str!("input.txt").lines() {
        match &line[..4] {
            "mask" => {
                mask_or  = u64::from_str_radix(&line[7..].replace("X", "0"), 2).unwrap();
                mask_and = u64::from_str_radix(&line[7..].replace("X", "1"), 2).unwrap();
            },
            "mem[" => {
                let cap = re.captures(line).unwrap();
                let idx = cap.name("idx").unwrap().as_str().parse::<u64>().unwrap();
                let num = cap.name("num").unwrap().as_str().parse::<u64>().unwrap();
                mem.insert(
                    idx, 
                    (num | mask_or) & mask_and,
                );
            },
            _ => panic!("wtf?"),
        }
    }
    mem.iter().map(|(_, val)| *val).sum()
}

fn part2() -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let re = Regex::new(r"mem\[(?P<idx>\d+)\] = (?P<num>\d+)").unwrap();

    let mut mask  : u64 = 0;
    let mut floats: Vec<u64> = vec![];
    for line in include_str!("input.txt").lines() {
        match &line[..4] {
            "mask" => {
                mask  = u64::from_str_radix(&line[7..].replace("X", "0"), 2).unwrap();
                floats = line[7..].chars().rev().enumerate().filter(|(_, c)| *c == 'X')
                                  .map(|(i, _)| 1 << i).collect::<Vec<u64>>();
            },
            "mem[" => {
                let cap = re.captures(line).unwrap();
                let idx = cap.name("idx").unwrap().as_str().parse::<u64>().unwrap();
                let num = cap.name("num").unwrap().as_str().parse::<u64>().unwrap();

                let t:u64 = floats.iter().sum();
                for w in 0..(1u64 << floats.len()) {
                    let s: u64 = floats
                        .iter().enumerate()
                        .filter(|(i, _)| w & (1 << i) != 0)
                        .map(|(_, c)| *c).sum();
                    mem.insert(
                        idx^(idx&t) | mask | s,
                        num,
                    );
                }
            },
            _ => panic!("wtf?"),
        }
    }
    mem.iter().map(|(_, val)| *val).sum()
}

fn main() {
    println!("===== Part 1 ====");
    println!("The first number is {}", part1());
    println!("===== Part 2 ====");
    println!("The second number is {}", part2());
}
