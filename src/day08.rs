#[derive(Debug, PartialEq, Clone)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}
#[derive(Debug, Clone)]
struct Instruction {
    op: OpCode,
    val: i32,
}

pub fn part1(input: String) -> Option<String> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| parse_instruction(line).unwrap())
        .collect();

    let (finished, acc) = run_instructions(instructions);
    println!("Successfully Finished? : {:?}", finished);
    println!("Accumulator: {}", acc);
    Some(acc.to_string())
}

fn exec_instruction(inst: &Instruction, acc: &mut i32, prog_counter: &mut usize) {
    match inst.op {
        OpCode::Nop => *prog_counter += 1,
        OpCode::Acc => {
            *acc += inst.val;
            *prog_counter += 1;
        }
        OpCode::Jmp => {
            *prog_counter = (*prog_counter as i32 + inst.val) as usize;
        }
    }
}
fn run_instructions(prog: Vec<Instruction>) -> (bool, i32) {
    let mut inst_run = vec![false; prog.len()];
    let mut acc: i32 = 0;
    let mut i: usize = 0;
    while i < prog.len() {
        let inst = &prog[i];
        if inst_run[i] {
            break;
        }
        inst_run[i] = true;
        exec_instruction(inst, &mut acc, &mut i);
    }

    if i == prog.len() {
        (true, acc)
    } else {
        (false, acc)
    }
}

pub fn part2(input: String) -> Option<String> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| parse_instruction(line).unwrap())
        .collect();

    for i in 0..instructions.len() {
        let mut program = instructions.clone();
        match program[i].op {
            OpCode::Nop => program[i].op = OpCode::Jmp,
            OpCode::Jmp => program[i].op = OpCode::Nop,
            _ => continue,
        }
        let (success, acc) = run_instructions(program);
        if success {
            println!("ACC: {}", acc);
            return Some(acc.to_string());
        }

    }
    None
}

fn parse_instruction(s: &str) -> Option<Instruction> {
    let parts: Vec<&str> = s.split(" ").collect();

    match &*parts {
        ["nop", _] => {
            let instruction = Instruction {
                op: OpCode::Nop,
                val: 0,
            };
            Some(instruction)
        }
        ["acc", val] => {
            let instruction = Instruction {
                op: OpCode::Acc,
                val: val.parse::<i32>().ok()?,
            };
            Some(instruction)
        }
        ["jmp", val] => {
            let instruction = Instruction {
                op: OpCode::Jmp,
                val: val.parse::<i32>().ok()?,
            };
            Some(instruction)
        }
        _ => None,
    }
}
