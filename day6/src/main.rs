use std::collections::HashSet;
use std::iter::FromIterator;

fn read() -> Vec<String> {
    include_str!("input.txt")
        .split("\n\n")
        .map(|s| s.trim().to_string())
        .collect()
}

fn part1(input: &Vec<String>) -> usize {
    input.iter()
         .map(|group| HashSet::<char>::from_iter(
                 group
                 .replace("\n", "")
                 .chars())
             .len())
         .sum()
}

fn part2(input: &Vec<String>) -> usize {
    let mut c = 0;
    for group in input {
        let mut qs: Vec<char>  = group.split("\n").next().unwrap().chars().collect();
        for p in group.split("\n") {
            qs = qs
                .iter()
                .filter(|q| p.contains(&q.to_string()))
                .map(|q| *q)
                .collect();
        }
        c += qs.len();
    }
    c
}

fn main() {
    let input = read();
    println!("===== Part 1 ====");
    println!("num is {}", part1(&input));
    println!("===== Part 2 ====");
    println!("num is {}", part2(&input));
}
