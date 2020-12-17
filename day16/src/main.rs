use regex::Regex;
use std::collections::HashSet;

fn read() -> (
    Vec<(String, usize, usize, usize, usize)>,  // Rules
    Vec<usize>,                                 // Our ticket
    Vec<Vec<usize>>                             // All tickets
) {
    let re = Regex::new(r"^(?P<name>\w+( \w+)?): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)").unwrap();
    let parts: Vec<&str> = include_str!("input.txt").split("\n\n").collect();
    let rules: Vec<(String, usize, usize, usize, usize)> = 
        parts[0].lines().map(|rule| {
                let cap = re.captures(rule).unwrap();
                let name = cap.name("name").unwrap().as_str().to_string();
                let mi1 = cap.name("min1").unwrap().as_str().parse().unwrap();
                let ma1 = cap.name("max1").unwrap().as_str().parse().unwrap();
                let mi2 = cap.name("min2").unwrap().as_str().parse().unwrap();
                let ma2 = cap.name("max2").unwrap().as_str().parse().unwrap();
                (name, mi1, ma1, mi2, ma2)
            }).collect();

    let our_ticket: Vec<usize> = parts[1][13..].split(",").map(|c| c.parse().unwrap()).collect();

    let tickets: Vec<Vec<usize>> = parts[2][16..].lines()
        .map(|line| line.split(",").map(|c| c.parse().unwrap()).collect())
        .collect();


    (rules, our_ticket, tickets)
}

fn conforms(t: usize, r: &(String, usize, usize, usize, usize)) -> bool {
    (r.1 <= t && t <= r.2) || (r.3 <= t && t <= r.4)
}

fn part1(tickets: &Vec<Vec<usize>>, 
         rules: &Vec<(String, usize, usize, usize, usize)>) -> usize {
    tickets
        .iter()
        .map(|ticket| 
                ticket.iter()
                .filter(|t| ! rules.iter().any(|r| conforms(**t, r)))
                .map(|c| *c).sum::<usize>())
        .sum()
}

fn part2(mut tickets: Vec<Vec<usize>>, 
         our_ticket: &Vec<usize>,
         rules: &Vec<(String, usize, usize, usize, usize)>) -> usize {

    tickets = tickets
        .iter()
        .filter(|ticket| 
                ticket.iter()
                .all(|t| rules.iter().any(|r| conforms(*t, r))))
        .map(|c| c.clone())
        .collect();

    let rule_set: HashSet<usize> = (0..rules.len()).collect();
    let mut possible_rules = vec![rule_set.clone(); our_ticket.len()];

    for i in 0..rules.len() {
        for (j, rule) in rules.iter().enumerate() {
            for ticket in &tickets {
                if !conforms(ticket[i], rule) {
                    possible_rules[i].remove(&j);
                }
            }
        }
    }

    let mut burned = HashSet::<usize>::new();
    let mut remap = vec![0; rules.len()];

    let mut matching_rules: Vec<(usize, &HashSet<_>)> = possible_rules
        .iter()
        .enumerate()
        .collect();

    matching_rules.sort_by_key(|(_, a)| a.len());

    for (field, cur_rule_set) in matching_rules {
        for rule in cur_rule_set {
            if ! burned.contains(rule) {
                remap[field] = *rule;
                burned.insert(*rule);
                break;
            }
        }
    }

    our_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| rules[remap[*i]].0.contains("departure"))
        .map(|(_, f)| *f)
        .product()
}
    

fn main() {
    let (rules, our_t, ts) = read();
    println!("===== Part 1 ====");
    println!("The first number is {}", part1(&ts, &rules));
    println!("===== Part 2 ====");
    println!("The second number is {}", part2(ts, &our_t, &rules));
}
