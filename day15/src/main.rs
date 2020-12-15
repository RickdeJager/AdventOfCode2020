use std::collections::HashMap;

fn solve(input: &Vec<usize>, n: usize) -> usize {
    let mut mem = HashMap::<usize, usize>::new();
    for c in input {
        mem.insert(*c, 0);
    }

    let mut last = 0;
    for i in 0..n {
        if i < input.len() {
            last = input[i];
            mem.insert(input[i], i+1);
        } else {
            last = i - mem.insert(last, i+1).unwrap_or(i);
        }
    }
    last
}

fn main() {
    let input: Vec<usize> = vec![7,12,1,0,16,2];
    println!("===== Part 1 ====");
    println!("The first number is {}", solve(&input, 2020));
    println!("===== Part 2 ====");
    println!("The second number is {}", solve(&input, 30000000));
}
