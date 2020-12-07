use regex::Regex;
use std::collections::HashMap;

fn read() -> Vec<String> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect()
}

struct Bag {
    color: String,
    amount: usize,
}

fn to_bags(input: &Vec<String>) -> HashMap<String, Vec<Bag>> {
    let main    = Regex::new(r"(\w+ \w+) bags contain").unwrap();
    let contain = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();

    input
        .iter()
        .map(|line| {
             let parent = main.captures(line).unwrap()[1].to_string();
             let children: Vec<Bag> = contain.captures_iter(line)
                 .map(|cap| Bag {color: cap[2].to_string(), amount: cap[1].parse().unwrap()})
                 .collect();
             (parent, children)
        })
    .collect()
}

fn has_gold(bags: &HashMap<String, Vec<Bag>>, bag: &String) -> bool {
    match bag.as_str() {
        "shiny gold" => true,
        _ => bags.get(bag).unwrap().iter()
            .any(|child_bag| has_gold(bags, &child_bag.color)),
    }
}

fn part1(bags: &HashMap<String, Vec<Bag>>) -> usize {
    bags
        .iter()
        .filter(|(bag, _)| has_gold(bags, bag))
        .count() - 1
}

fn get_content(bags: &HashMap<String, Vec<Bag>>, bag: &String) -> usize {
    let children = bags.get(bag).unwrap();
    match children[..] {
        [] => 0,
         _ => children.iter()
             .map(|child_bag| child_bag.amount * (1 + get_content(bags, &child_bag.color)))
             .sum()
    }
}

fn part2(bags: &HashMap<String, Vec<Bag>>) -> usize {
    get_content(bags, &"shiny gold".to_string())
}

fn main() {
    let input = read();
    let bags = to_bags(&input);
    println!("===== Part 1 ====");
    println!("num is {}", part1(&bags));
    println!("===== Part 2 ====");
    println!("num is {}", part2(&bags));
}
