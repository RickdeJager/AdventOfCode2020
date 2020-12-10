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

fn helper2(group: &String) -> usize {
    let mut set: HashSet<char> =  group.split("\n").next().unwrap().chars().collect();
    for p in group.split("\n") {
        let qs = HashSet::<char>::from_iter(p.chars());
        set = set.intersection(&qs).copied().collect();
    }
    set.len()
}

fn part2(groups: &Vec<String>) -> usize {
    groups
        .iter()
        .map(|group| helper2(group))
        .sum()
}

fn main() {
    let input = read();
    println!("===== Part 1 ====");
    println!("num is {}", part1(&input));
    println!("===== Part 2 ====");
    println!("num is {}", part2(&input));
}
