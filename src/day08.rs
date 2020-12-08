use std::convert::TryInto;
#[derive(Debug)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Instruction {
    op: OpCode,
    val: i32,
    runs: i32,
}
pub fn part1(input: String) -> Option<String> {
    let mut instructions: Vec<Instruction> = input
        .lines()
        .map(|line| parse_instruction(line).unwrap())
        .collect();

    let mut acc: i32 = 0;
    let mut i : i32 = 0;
    while i < instructions.len() as i32{
        if let Some(mut inst) = instructions.get_mut(i as usize) {
            if inst.runs > 0 {
                break;
            }
            inst.runs += 1;
            i += 1;
            match inst.op {
                OpCode::Nop => continue,
                OpCode::Acc => acc += inst.val,
                OpCode::Jmp => i += inst.val-1,
            }
            if i < 0 {
                panic!("Program counter went below zero");
            }
        }
    }
    println!("Accumulator: {}", acc);
    Some(acc.to_string())
}

fn parse_instruction(s: &str) -> Option<Instruction> {
    let parts: Vec<&str> = s.split(" ").collect();

    match &*parts {
        ["nop", _] => {
            let instruction = Instruction {
                op: OpCode::Nop,
                val: 0,
                runs: 0,
            };
            Some(instruction)
        }
        ["acc", val] => {
            let instruction = Instruction {
                op: OpCode::Acc,
                val: val.parse::<i32>().ok()?,
                runs: 0,
            };
            Some(instruction)
        }
        ["jmp", val] => {
            let instruction = Instruction {
                op: OpCode::Jmp,
                val: val.parse::<i32>().ok()?,
                runs: 0,
            };
            Some(instruction)
        }
        _ => None,
    }
}
