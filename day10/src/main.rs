fn read() -> Vec<usize> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1(mut input: Vec<usize>) -> usize {
    input.push(0);
    input.sort();
    
    let mut j1 = 0;
    let mut j3 = 1; // final gap will be 3, so add 1 here
    for w in input.windows(2) {
        match w[1] - w[0] {
            1 => j1 += 1,
            3 => j3 += 1,
            _ =>  (),
        }
    }
    j1 * j3
}

fn part2(mut v: Vec<usize>) -> usize { 
    // Some data massaging
    v.push(0);
    v.sort();
    v.push(v[v.len()-1] + 3);
    // cache[i] ==> number of valid chains that include v[i]
    let mut cache: Vec<usize> = vec![0; v.len()];
    cache[0] = 1;

    for i in 1..v.len() {
        let s = i32::max(i as i32 - 3, 0) as usize;
        cache[i] = (s..i)
            .filter(|k| v[i] - v[*k] <= 3)
            .map(|k| cache[k]).sum();
    }
    cache[cache.len()-1]
}

fn main() {
    let input = read();
    println!("===== Part 1 ====");
    println!("The first number is {}", part1(input.clone()));
    println!("===== Part 2 ====");
    println!("The second number is {}", part2(input.clone()));
}
