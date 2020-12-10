fn read() -> Vec<String> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|l| l.trim().to_string())
        .collect()
}

fn helper1(bp: &str) -> u32 {
    u32::from_str_radix(
        &bp.replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1")
        , 2).unwrap()
}

fn part1(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|bp| helper1(bp))
        .max().unwrap()
}

fn part2(input: &Vec<String>) -> u32 {
    let mut bps: Vec<u32> = input
        .iter()
        .map(|bp| helper1(bp))
        .collect();
    bps.sort();

    let mut last: u32 = bps[0];
    for bp in bps {
        if bp-last > 1 {
            return last + 1;
        }
        last = bp;
    }
    0
}

fn main() {
    let passes = read();
    println!("===== Part 1 ====");
    println!("Max is {}", part1(&passes));
    println!("===== Part 2 ====");
    println!("Our pass is {}", part2(&passes));
}
