#[derive(Debug, Clone)]
struct Opcode {
    command  : String,
    argument: i32,
    dirty: bool,

}

#[derive(Debug, Clone)]
struct Pc {
    mem: Vec<Opcode>,
    acc: i32,
    ip : usize,
}

fn read() -> Vec<Opcode> {
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| Opcode{
            command: s[..3].to_string(), 
            argument: s[4..].parse().unwrap(),
            dirty: false
        })
        .collect()
}

fn noop(mut pc: Pc) -> Result<Pc, i32> {
    if pc.mem[pc.ip].dirty {
        return Err(pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.ip += 1;
    }
    Ok(pc)
}

fn acc(mut pc: Pc) -> Result<Pc, i32> {
    if pc.mem[pc.ip].dirty {
        return Err(pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.acc += pc.mem[pc.ip].argument;
        pc.ip += 1;
    }
    Ok(pc)
}

fn jmp(mut pc: Pc) -> Result<Pc, i32> {
    if pc.mem[pc.ip].dirty {
        return Err(pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.ip = (pc.ip as i32 + pc.mem[pc.ip].argument) as usize;
    }
    Ok(pc)
}

fn run(mut pc: Pc) -> Result<i32, i32> {
    while pc.ip < pc.mem.len() {
        let c = pc.mem[pc.ip].command.as_str();
        let maybe_pc: Result<Pc, i32>;
        match c {
            "nop" => maybe_pc = noop(pc),
            "acc" => maybe_pc = acc(pc),
            "jmp" => maybe_pc = jmp(pc),
            _ => panic!("wtf: {}", c),
        }
        match maybe_pc {
            Ok(p)  => pc = p,
            Err(p) => return Err(p),
        }
    }
    Ok(pc.acc)
}

fn part1(pc: Pc) -> i32{
    if let Err(acc) = run(pc) {
        return acc;
    }
    unreachable!();
}

fn part2(pc: Pc) -> i32 {
    for i in 0..pc.mem.len() {
        let mut clone = pc.clone();
        if clone.mem[i].command == "nop" {
            clone.mem[i].command = "jmp".to_string();
        } else if clone.mem[i].command == "jmp" {
            clone.mem[i].command = "nop".to_string();
        }
        if let Ok(acc) = run(clone) {
            return acc;
        }
    } 
    unreachable!();
}

fn main() {
    let pc = Pc{mem: read(), acc: 0, ip: 0};
    println!("===== Part 1 ====");
    println!("The machine hangs with acc value {}", part1(pc.clone()));
    println!("===== Part 2 ====");
    println!("The machine should cleanly exit with {}", part2(pc.clone()));
}
