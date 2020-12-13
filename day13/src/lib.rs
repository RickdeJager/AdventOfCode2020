// Code originally found on 
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
//
// Modified to allow negative residue,
// Removed some safety checks,
// and to work a bit better with the puzzle input


fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> i64 {
    let (g, x, _) = egcd(x, n);
    assert!(g == 1);
    (x % n + n) % n
}

// Pairwise residue/modulii
pub fn crt(v: Vec<(i64, i64)>) -> i64 {
    let prod: i64 = v.iter().map(|s| s.1).product();

    let mut sum = 0;
    for (r, m) in v {
        let p = prod / m;
        sum += ((r+m)%m) * mod_inv(p, m) * p;
    }
    sum % prod
}
