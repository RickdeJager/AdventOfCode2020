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

fn main() {
    let input = read();
    println!("===== Part 1 ====");
    println!("num is {}", part1(&input));
}
