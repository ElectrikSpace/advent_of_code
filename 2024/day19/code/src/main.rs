use std::fs;
//use std::collections::HashMap;
//use std::collections::HashSet;

//#[derive(Clone, PartialEq, Eq, Hash, Debug)]
//enum Item {
//    Wall,
//    Start,
//    End,
//    Nice,
//    Empty
//}

fn is_possible(design: &str, towels: &Vec<&str>) -> bool {
    let mut tries = Vec::<usize>::new();
    tries.push(0);
    while !tries.is_empty() {
        let index = tries.pop().unwrap();
        println!("index is {}", index);
        if index == design.len() {
            return true;
        }
        towels
            .iter()
            .filter(|x| {
                let len = x.len();
                if len == 0 {
                    return false;
                }
                if (index + len) > design.len() {
                    return false;
                }
                //println!("trying {:?} with on {:?}", x, design[index..max]);
                //println!("trying {:?}", x);
                return design[index..(index+len)] == ***x;
            })
            .for_each(|m| {
                tries.push(index + m.len());
            });
    }
    return false;
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // Parse
    let mut contents_lines = contents.lines();
    let towels_str = contents_lines.next().unwrap();
    let towels = towels_str
        .trim()
        .split(", ")
        .collect::<Vec<_>>();
    contents_lines.next();
    let designs = contents_lines.collect::<Vec<_>>();

    // Debug
    println!("Towels: {:?}", towels);
    println!("Designs: {:?}", designs);

    // Part1
    for design in designs {
        println!("Checking design {}", design);
        println!(" -> {}", is_possible(&design, &towels));
    }
}
