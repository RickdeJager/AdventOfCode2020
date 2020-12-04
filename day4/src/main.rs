fn read() -> Vec<String>{
    include_str!("input.txt")
        .trim()
        .split("\n\n")
        .map(|l| l.to_string())
        .collect()
}

fn check(passport: &String) -> bool {
    let reqs = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for req in reqs {
        if ! passport.contains(req) {
            return false;
        }
    }
    true
}

fn part1(passports: &Vec<String>) -> usize {
    let mut i = 0;
    for passport in passports {
        if check(passport) {
            i += 1;
        }
    }
    i
}

fn main() {
    let passports = read();
    println!("===== Part 1 ====");
    println!("Found {} valid passports", part1(&passports));
}
