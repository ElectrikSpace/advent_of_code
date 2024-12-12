use std::fs;

fn find_result_part1(result: &i64, operands: &Vec<i64>, current: &i64) -> bool {
    let next_add = current + operands[0];
    let next_mul = current * operands[0];
    if (next_add == *result) || (next_mul == *result) {
        return true;
    }
    else if (next_add > *result) && (next_mul > *result) {
        return false;
    }
    else if operands.len() == 1 {
        return false;
    }
    else {
        let next_operands = operands[1..].to_vec();
        return find_result_part1(result, &next_operands, &next_add) ||
               find_result_part1(result, &next_operands, &next_mul);
    }
}

fn number_digits(x: i64) -> u32 {
    let mut n = 0;
    let mut v = x;
    while (v / 10) > 0 {
        v = v / 10;
        n += 1;
    }
    if (v % 10) > 0 {
        n += 1;
    }
    return n;
}

fn find_result_part2(result: &i64, operands: &Vec<i64>, current: &i64) -> bool {
    let next_add = current + operands[0];
    let next_mul = current * operands[0];
    let next_concat = current * 10_i64.pow(number_digits(operands[0])) + operands[0];
    if (operands.len() == 1) && 
       ((next_add == *result) || (next_mul == *result) || (next_concat == *result)) {
        return true;
    }
    else if (next_add > *result) && (next_mul > *result) && (next_concat > *result){
        return false;
    }
    else if operands.len() == 1 {
        return false;
    }
    else {
        let next_operands = operands[1..].to_vec();
        return find_result_part2(result, &next_operands, &next_add) ||
               find_result_part2(result, &next_operands, &next_mul) ||
               find_result_part2(result, &next_operands, &next_concat);
    }
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    let mut acc_part1 = 0;
    let mut acc_part2 = 0;
    for line in contents.trim().lines() {
        let mut split = line.split(":");
        let result = split.next()
            .expect("Unable to parse result")
            .parse::<i64>()
            .expect("Result is not a number");

        let operands = split.next()
            .expect("Unable to parse operands")
            .trim()
            .split(" ")
            .map(|x| x.parse::<i64>()
                      .expect("Operands is not a number"))
            .collect::<Vec<i64>>();

        if find_result_part1(&result, &operands, &0) {
            acc_part1 += result;
        }
        else if find_result_part2(&result, &operands, &0) {
            acc_part2 += result;
        }
    }
    println!("acc part 1 = {acc_part1}");
    println!("new found = {acc_part2}");
    println!("acc part 2 = {}", acc_part1 + acc_part2)
}
