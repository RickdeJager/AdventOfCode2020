fn read() -> Vec<usize> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1(input: &Vec<usize>) -> usize {
    const Window: usize = 25;
    for (i, num) in input.iter().enumerate() {
        if i < Window {
            continue;
        }
        let mut flag = false;
        for j in i-Window..i {
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

fn main() {
    let input = read();
    println!("===== Part 1 ====");
    println!("The first number is {}", part1(&input));
    println!("===== Part 2 ====");
}
