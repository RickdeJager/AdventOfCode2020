fn read_field() -> Vec<Vec<char>>{
    include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|s| s.trim().chars().collect())
        .collect()
}

fn part1(field: &Vec<Vec<char>>, dx: usize, dy: usize) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut hits = 0;
    while y < field.len() {
        if field[y][x%field[y].len()] == '#' {
            hits += 1;
        }
        x += dx;
        y += dy;
    }
    hits
}

fn part2(field: &Vec<Vec<char>>) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| part1(field, *dx, *dy))
        .product()
}

fn main() {
    let field = read_field();
    println!("===== Part 1 ====");
    println!("Hit {} trees!", part1(&field, 3, 1));
    println!("===== Part 2 ====");
    println!("Hit {} combined trees!", part2(&field));
}
