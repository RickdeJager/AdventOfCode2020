fn read1() -> (usize, Vec<usize>) {
    let lines: Vec<&str> = include_str!("input.txt").trim().split("\n").collect();
    (lines[0].parse().unwrap(), 
     lines[1].trim().split(",").filter_map(|s| s.parse().ok()).collect())
}

fn part1(input: (usize, Vec<usize>)) -> usize {
   let mut i = input.0;
   loop {
       for m in input.1.clone() {
            if i % m == 0 {
                return (i - input.0) * m;
            }
       }
       i += 1;
   }
}

fn main() {
    println!("===== Part 1 ====");
    println!("The first number is {}", part1(read1()));
    println!("===== Part 2 ====");
//    println!("The second number is {}", part2(&input));
}
