fn read() -> Vec<Vec<char>> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|n| n.trim().chars().collect())
        .collect()
}

fn round(state: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new = state.clone();
    for i in 0..state.len() {
        for j in 0..state[i].len() {

            let mut count = 0;
            let bi = if i == 0 {i} else {i-1};
            let ei = if i == state.len()-1 {i+1} else {i+2};
            let bj = if j == 0 {j} else {j-1};
            let ej = if j == state[i].len()-1 {j+1} else {j+2};
            for li in bi..ei {
                for lj in bj..ej {
                    if li == i && lj == j {
                        continue;
                    }
                    if state[li][lj] == '#' {
                        count += 1;
                    }
                }
            }
            
            match state[i][j] {
                'L' => new[i][j] = if count == 0 {'#'} else {'L'},
                '#' => new[i][j] = if count >= 4 {'L'} else {'#'},
                _ => (),
            }
        }

    }
    new
}

fn display(state: &Vec<Vec<char>>) {
    for v in state {
        println!("{}", v.iter().collect::<String>());
    }

}

fn eq(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn part1(mut v: Vec<Vec<char>>) -> usize {
    let mut prev = v.clone();
    v = round(v);
    while ! eq(&v, &prev) {
        prev = v.clone();
        v = round(v);
    }
    v
        .iter()
        .map(|w| w.iter().filter(|c| **c == '#').count())
        .sum()
}

fn main() {
    let input = read();
//    display(&round(round(round(round(input)))));
    println!("===== Part 1 ====");
    println!("The first number is {}", part1(input.clone()));
    println!("===== Part 2 ====");
    //println!("The second number is {}", part2(input.clone()));
}
