use std::panic;

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

fn noop(mut pc: Pc) -> Pc{
    if pc.mem[pc.ip].dirty {
        panic!("PC tried to access an instruction for the second time.\n
               Accumalator is at: {}", pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.ip += 1;
    }
    pc
}

fn acc(mut pc: Pc) -> Pc {
    if pc.mem[pc.ip].dirty {
        panic!("PC tried to access an instruction for the second time.\n
               Accumalator is at: {}", pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.acc += pc.mem[pc.ip].argument;
        pc.ip += 1;
    }
    pc
}

fn jmp(mut pc: Pc) -> Pc {
    if pc.mem[pc.ip].dirty {
        panic!("PC tried to access an instruction for the second time.\n
               Accumalator is at: {}", pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.ip = (pc.ip as i32 + pc.mem[pc.ip].argument) as usize;
    }
    pc
}

fn run(mut pc: Pc) -> i32 {
    while pc.ip < pc.mem.len() {
        let c = pc.mem[pc.ip].command.as_str();
        match c {
            "nop" => pc = noop(pc),
            "acc" => pc = acc(pc),
            "jmp" => pc = jmp(pc),
            _ => panic!("wtf: {}", c),
        }
    }
    pc.acc
}

fn part1(pc: Pc) {
    panic::catch_unwind(|| run(pc));
}

fn part2(pc: Pc) -> i32 {
    for i in 0..pc.mem.len() {
        let mut clone = pc.clone();
        if clone.mem[i].command == "nop" {
            clone.mem[i].command = "jmp".to_string();
        } else if clone.mem[i].command == "jmp" {
            clone.mem[i].command = "nop".to_string();
        }
        let c2 = clone.clone();
        if ! panic::catch_unwind(|| run(clone)).is_err() {
            return run(c2);
        };
    } 
    unreachable!();
}

fn main() {
    let pc = Pc{mem: read(), acc: 0, ip: 0};
    println!("===== Part 1 ====");
    part1(pc.clone());
    println!("===== Part 2 ====");
    println!("---> {}", part2(pc.clone()));
}
