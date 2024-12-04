use std::fs;
use regex::Regex;

fn main() {
    let file_path = "./input";

    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let parse_ok = Regex::new(r"(?s)(do\(\).*?don't\(\))|(do\(\).*?$)|(do\(\).*?do\(\))").unwrap();
    let parse_mul = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let parse_number = Regex::new(r"\d+").unwrap();

    let all: Vec<_> = parse_mul.find_iter(&contents).map(|m| m.as_str()).collect();
    let to_process_str: Vec<_> = parse_ok.find_iter(&contents).map(|m| m.as_str()).collect();
    let mut ok: Vec<_> = vec![];
    for t in to_process_str.iter() {
        for p in parse_mul.find_iter(t).map(|m| m.as_str()) {
            ok.push(p);
        }
    }
    println!("Total count is {}", all.len());
    println!("Ok count is {}", ok.len());

    let mut acc = 0;
    for capture in all.iter() {
        acc += capture.split(",")
                         .map(|x| parse_number.find(x).unwrap().as_str().parse::<i32>().unwrap())
                         .fold(1, |a, b| a*b);
    }
    let mut acc2 = 0;
    println!("total part1 is {acc}");
    for capture in ok.iter() {
        acc2 += capture.split(",")
                         .map(|x| parse_number.find(x).unwrap().as_str().parse::<i32>().unwrap())
                         .fold(1, |a, b| a*b);
    }
    println!("total part2 is {acc2}");
}
