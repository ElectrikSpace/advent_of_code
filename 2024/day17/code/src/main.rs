use std::fs;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    pc: i64
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Instruction {
    Adv,
    Bdv,
    Cdv,
    Bxl,
    Bxc,
    Bst,
    Jnz,
    Out
}

fn decode_instruction(opcode: &i64) -> Instruction {
    return match opcode {
        0 => Instruction::Adv,
        1 => Instruction::Bxl,
        2 => Instruction::Bst,
        3 => Instruction::Jnz,
        4 => Instruction::Bxc,
        5 => Instruction::Out,
        6 => Instruction::Bdv,
        7 => Instruction::Cdv,
        _ => unreachable!()
    };
}

fn decode_operand(operand: &i64, state: &State) -> i64 {
    if *operand < 4 {
        return *operand;
    }
    return match operand {
        4 => state.reg_a.clone(),
        5 => state.reg_b.clone(),
        6 => state.reg_c.clone(),
        _ => unreachable!()
    };
}

fn execute_one(state: &mut State, code: &Vec<i64>) -> Option<i64> {
    let opcode = decode_instruction(code.get(state.pc as usize)
                                        .expect("Segfault"));
    let operand_literal = code.get((state.pc + 1) as usize)
                              .expect("Segfault");
    let operand_combo = decode_operand(operand_literal, state);
    match opcode {
        Instruction::Adv => {
            state.reg_a = state.reg_a / 2i64.pow(operand_combo as u32);
        },
        Instruction::Bdv => {
            state.reg_b = state.reg_a / 2i64.pow(operand_combo as u32);
        },
        Instruction::Cdv => {
            state.reg_c = state.reg_a / 2i64.pow(operand_combo as u32);
        },
        Instruction::Bxl => {
            state.reg_b = state.reg_b ^ operand_literal;
        },
        Instruction::Bst => {
            state.reg_b = operand_combo % 8;
        },
        Instruction::Jnz => {
            if state.reg_a != 0 {
                state.pc = operand_literal - 2;
            }
        },
        Instruction::Bxc => {
            state.reg_b = state.reg_b ^ state.reg_c;
        },
        Instruction::Out => {}
        _ => unreachable!()
    }
    state.pc += 2;
    if opcode == Instruction::Out {
        return Some(operand_combo % 8);
    }
    return None;
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let mut lines_iter = contents.lines();
    let reg_a = lines_iter
        .next().expect("Expected a line")
        .trim()
        .split(" ")
        .last().expect("Error while parsing reg")
        .parse::<i64>().expect("Unable to Parse reg");
    let reg_b = lines_iter
        .next().expect("Expected a line")
        .trim()
        .split(" ")
        .last().expect("Error while parsing reg")
        .parse::<i64>().expect("Unable to Parse reg");
    let reg_c = lines_iter
        .next().expect("Expected a line")
        .trim()
        .split(" ")
        .last().expect("Error while parsing reg")
        .parse::<i64>().expect("Unable to Parse reg");
    lines_iter.next();
    let code = lines_iter
        .next().expect("Expected a line")
        .trim()
        .split(" ")
        .last().expect("Error while parsing code")
        .split(",")
        .map(|x| x.parse::<i64>().expect("Unable to parse code"))
        .collect::<Vec<i64>>();

    let mut output = Vec::<i64>::new();
    let mut state = State {
        reg_a: reg_a,
        reg_b: reg_b,
        reg_c: reg_c,
        pc: 0
    };
    
    // Part1
    let mut pcc = 0;
    while state.pc < (code.len() as i64) {
        if let Some(o) = execute_one(&mut state, &code) {
            output.push(o);
        }
        pcc += 1;
    }
    println!("Cycles: {pcc}");
    println!("output is: {:?}", output);
    output.clear();

    // Part2
    let mut reg_a_start = 0;
    let mut index = 0;
    while index != code.len() {
        let refs: Vec<i64> = vec![code[code.len()-1 - index-1], code[code.len()-1-index]];
        let mut part_reg_a_start = 0;
        while true {
            state.reg_a = part_reg_a_start | (reg_a_start << 6);
            //state.reg_a = part_reg_a_start;
            state.reg_b = reg_b;
            state.reg_c = reg_c;
            state.pc = 0;
            pcc = 0;
            let mut outs = Vec::<i64>::new();
            while (state.pc < (code.len() as i64)) {
                if let Some(o) = execute_one(&mut state, &code) {
                    outs.push(o);
                    if outs.len() == 2 {
                        break;
                    }
                }
                pcc += 1;
            }
            if outs.len() == 2 && outs[0] == refs[0] && outs[1] == refs[1] {
                break;
            }
            part_reg_a_start += 1;
            pcc = 0;
        }
        reg_a_start = part_reg_a_start | (reg_a_start << 6); 
        index += 2;
    }
    println!(" -> Success with reg_a = {} !", reg_a_start);

    // Check
    state.reg_a = reg_a_start;
    state.reg_b = reg_b;
    state.reg_c = reg_c;
    state.pc = 0;
    pcc = 0;
    output.clear();
    while state.pc < (code.len() as i64) {
        if let Some(o) = execute_one(&mut state, &code) {
            output.push(o);
        }
        pcc += 1;
    }
    println!("Cycles: {pcc}");
    println!("output is: {:?}", output);
}
