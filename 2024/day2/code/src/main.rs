use std::fs;
use itertools::Itertools;

fn isSafe(elts: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut direction = 0;
    for window in elts.windows(2).collect::<Vec<_>>() {
        let mut window_iter = window.iter();
        let a = window_iter.next().unwrap();
        let b = window_iter.next().unwrap();
        let absDiff = i32::abs(a-b);
        if absDiff < 1 || absDiff > 3 {
            safe = false;
            continue;
        }
        if direction == 0 {
            direction = a - b;
        }
        else if (direction.is_positive() && (a-b).is_negative())
            || (direction.is_negative() && (a-b).is_positive()) {
            safe = false;
            continue
        }
        //println!("{a} {b}");
    }
    return safe;
}

fn main() {
    let file_path = "./input";

    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut safes = 0;
    for line in lines {
        println!("line {line}");
        let elts = line.split(" ")
                       .map(|x| x.parse::<i32>().unwrap())
                       .collect::<Vec<_>>();
        let mut safe = isSafe(&elts);
        let mut toRemove = 0..elts.len();
        let mut i = toRemove.next();
        while !safe && i.is_some() {
            let j = i.unwrap();
            let mut newElts = elts.to_vec();
            newElts.remove(j);
            println!("blbl is for {j}: {:?}", newElts);
            safe = isSafe(&newElts);
            i = toRemove.next();
        }

        if safe {
           safes += 1;
        }
    }
    println!("safes {safes}")
}
