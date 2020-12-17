use std::collections::HashSet;

fn read() -> HashSet<Vec<i32>> {
    include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(y, l)|
            l
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(x, _)| vec![x as i32, y as i32, 0i32, 0i32])
            .collect::<HashSet<_>>())
        .flatten()
        .collect()
}

fn solve(mut cubes: HashSet<Vec<i32>>, ds: usize)
    -> usize {

    for _ in 0..6 {
        let mut next = cubes.clone();
        // Keep a set of all "non-cubes" we encounter, since those
        // have neighbours
        let mut not_cubes :HashSet::<Vec<i32>> = HashSet::new();

        for cube in &cubes {
            let mut count = 0;
            // A grid/box around the center box has 3^dimension elements.
            //  < -3- >
            // [0, 1, 2] ^
            // [3, 4, 5] 3
            // [6, 7, 8] v
            // One "row" for every dimension.
            //
            // Now we can get the coordinate number belonging to axis "i"
            // by computing (3^ds) div i mod 3
            // 
            // Note that the origin is not placed on the center element.
            // We can shift the entire box by subtracting 1 from every coord
            for alpha in 0..usize::pow(3, ds as u32) {
                let mut neighbour = cube.clone();
                for d in 0..ds {
                    neighbour[d] 
                        += (alpha as i32 / i32::pow(3, d as u32) % 3 as i32) - 1;
                }
                // If a neighbour is in our big list of cubes, it must exist
                if cubes.contains(&neighbour) && neighbour != *cube {
                    count += 1;
                } else {
                    // If there is no cube at this position, we add it to
                    // the "non-cube" set. It may have enough neighbours
                    // to turn active.
                    not_cubes.insert(neighbour);
                }
            }

            // Apply the Conway cube rules as stated in the prompt.
            if count != 2 && count != 3 {
                next.remove(cube);
            }
        }

        // Repeat the same process for "non-cubes"
        for not_cube in &not_cubes {
            let mut count = 0;
            for alpha in 0..usize::pow(3, ds as u32) {
                let mut neighbour = not_cube.clone();
                for d in 0..ds {
                    neighbour[d] 
                        += (alpha as i32 / i32::pow(3, d as u32) % 3 as i32) - 1;
                }
                if cubes.contains(&neighbour) && neighbour != *not_cube {
                    count += 1;
                }
            }

            // Apply the Conway cube rules as stated in the prompt.
            if count == 3 {
                next.insert(not_cube.clone());
            }
        }
        // Save the newly computed state
        cubes = next.clone();
    }
    cubes.len()
}

fn main() {
    let cubes = read();
    println!("===== Part 1 ====");
    println!("The first number is {}", solve(cubes.clone(), 3));
    println!("===== Part 2 ====");
    println!("The second number is {}", solve(cubes.clone(), 4));
}
