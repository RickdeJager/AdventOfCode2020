struct Opcode {
    command  : String,
    argument: i32,
    dirty: bool,

}

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
        panic!("PC tired to access an instruction for the second time.\n
               Accumalator is at: {}", pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.ip += 1;
    }
    pc
}

fn acc(mut pc: Pc) -> Pc {
    if pc.mem[pc.ip].dirty {
        panic!("PC tired to access an instruction for the second time.\n
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
        panic!("PC tired to access an instruction for the second time.\n
               Accumalator is at: {}", pc.acc);
    } else {
        pc.mem[pc.ip].dirty = true;
        pc.ip = (pc.ip as i32 + pc.mem[pc.ip].argument) as usize;
    }
    pc
}

fn main() {
    let mut pc = Pc{mem: read(), acc: 0, ip: 0};
    while true {
        let c = pc.mem[pc.ip].command.as_str();
        match c {
            "nop" => pc = noop(pc),
            "acc" => pc = acc(pc),
            "jmp" => pc = jmp(pc),
            _ => panic!("wtf: {}", c),
        }
    }
}
