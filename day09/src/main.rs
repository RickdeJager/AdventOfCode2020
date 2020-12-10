fn read() -> Vec<usize> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1(input: &Vec<usize>) -> usize {
    const WINDOW: usize = 25;
    for (i, num) in input.iter().enumerate() {
        if i < WINDOW {
            continue;
        }
        let mut flag = false;
        for j in i-WINDOW..i {
            for k in j..i {
                if input[j] + input[k] == *num {
                    flag = true;
                }
            }
        }
        if !flag {
            return *num;
        }
    }
    unreachable!();
}

fn part2(input: &Vec<usize>, goal: usize) -> usize {
    let mut first = 0;
    let mut last  = 0;
    let mut acc   = input[0];
    while last < input.len() {
        // We've overshot the goal :O.
        // Since the set is continuous, we can only kick out earlier elements
        while acc > goal {
            acc   -= input[first];
            first += 1;
        }
        // First could be last+1, in case we just kicked out an element > goal
        if last < first {
            last = first
        }
        // Now we can start adding elements
        while acc < goal {
            last += 1;
            acc  += input[last];
        }
        // If we've summed to the exact goal, return here
        if acc == goal {
            return input[first..last+1].iter().min().unwrap() +
                   input[first..last+1].iter().max().unwrap();
        }
    }
    unreachable!();
}

fn main() {
    let input = read();
    println!("===== Part 1 ====");
    let p1 = part1(&input);
    println!("The first number is {}", p1);
    println!("===== Part 2 ====");
    println!("The second number is {}", part2(&input, p1));
}
