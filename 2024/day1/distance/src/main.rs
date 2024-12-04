use std::fs;

fn main() {
    let file_path = "./input";

    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    //println!("With text:\n{contents}");
    
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in contents.split("\n").filter(|line| line.contains("   ")) {
        let mut elts = line.split("   ").enumerate();
        match elts.next() {
            //Some((_,value)) => println!("value is {value}"),
            Some((_,value)) => left.push(value.parse::<i32>().unwrap()),
            None => println!("No more items."),
        }
        match elts.next() {
            //Some((_,value)) => println!("value is {value}"),
            Some((_,value)) => right.push(value.parse::<i32>().unwrap()),
            None => println!("No more items."),
        }
    }
    left.sort();
    right.sort();
    let distance = left.iter()
        .zip(right.iter())
        .map(|(l, r)| i32::abs(l-r))
        .reduce(|a, b| a+b)
        .unwrap();
    println!("distance is {distance}");

    let occurences = left.iter().map(|x| (right.iter().filter(|&y| x==y).collect::<Vec<_>>().len()));
    let similarity = left.iter()
        .zip(occurences)
        .map(|(l, r)| l*(r as i32))
        .reduce(|a, b| a+b)
        .unwrap();
    println!("distance is {similarity}");
}
