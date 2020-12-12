fn read() -> Vec<(char, f64)> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|n| (n.chars().next().unwrap(), n[1..].parse().unwrap()))
        .collect()
}

fn part1(input: &Vec<(char, f64)>) -> f64 {
    let mut dir = (1.0, 0.0);
    let mut pos = (0.0, 0.0);
    for cmd in input {
        match cmd.0 {
            'N' => {
                pos.1 += cmd.1;
            },
            'E' => {
                pos.0 += cmd.1;
            },
            'S' => {
                pos.1 -= cmd.1;
            },
            'W' => {
                pos.0 -= cmd.1;
            },
            'L' => {
                dir = (cmd.1.to_radians().cos() * dir.0 - cmd.1.to_radians().sin() * dir.1, 
                       cmd.1.to_radians().sin() * dir.0 + cmd.1.to_radians().cos() * dir.1);
            },
            'R' => {
                dir = ((-cmd.1).to_radians().cos() * dir.0 - (-cmd.1).to_radians().sin() * dir.1, 
                       (-cmd.1).to_radians().sin() * dir.0 + (-cmd.1).to_radians().cos() * dir.1);
            },
            'F' => {
                pos.0 += dir.0 * cmd.1;
                pos.1 += dir.1 * cmd.1;
            },
            _ => panic!("wtf?"),
        }
    }
    (pos.0.abs() + pos.1.abs()).round()
}

fn part2(input: &Vec<(char, f64)>) -> f64 {
    let mut pos = (0.0, 0.0);
    let mut way = (10.0, 1.0);
    for cmd in input {
        match cmd.0 {
            'N' => {
                way.1 += cmd.1;
            },
            'E' => {
                way.0 += cmd.1;
            },
            'S' => {
                way.1 -= cmd.1;
            },
            'W' => {
                way.0 -= cmd.1;
            },
            'L' => {
                way = (cmd.1.to_radians().cos() * way.0 - cmd.1.to_radians().sin() * way.1, 
                       cmd.1.to_radians().sin() * way.0 + cmd.1.to_radians().cos() * way.1);
            },
            'R' => {
                way = ((-cmd.1).to_radians().cos() * way.0 - (-cmd.1).to_radians().sin() * way.1, 
                       (-cmd.1).to_radians().sin() * way.0 + (-cmd.1).to_radians().cos() * way.1);
            },
            'F' => {
                pos.0 += way.0 * cmd.1;
                pos.1 += way.1 * cmd.1;
            },
            _ => panic!("wtf?"),
        }
    }
    (pos.0.abs() + pos.1.abs()).round()
}

fn main() {
    let input = read();
    println!("===== Part 1 ====");
    println!("The first number is {}", part1(&input));
    println!("===== Part 2 ====");
    println!("The second number is {}", part2(&input));
}
