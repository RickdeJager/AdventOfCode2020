use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("===== Part 1 ====");
    part1(&input);
    println!("===== Part 2 ====");
    part2(&input);
}

fn read_input(path: &str) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(File::open(path)?);
    br.lines().map(|line| 
                   line.and_then(|entry| entry.parse()
                   .map_err(|e| Error::new(ErrorKind::InvalidData, e))))
              .collect()
}

fn part1(input: &Vec<i64>) {
    for i in 0..input.len() {
        for j in i..input.len() {
            if input[i]+input[j] == 2020 {
                println!("Found number: {}\n", input[i] * input[j]);
            }
        }
    }
}

fn part2(input: &Vec<i64>) {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            for k in j+1..input.len() {
                if input[i]+input[j]+input[k] == 2020 {
                    println!("Found number: {}\n", input[i] * input[j] * input[k]);
                }
            }
        }
    }
}
