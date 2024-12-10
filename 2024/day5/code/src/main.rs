use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
//use itertools::Itertools;

fn main() {
    let file_path = "./input";
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut successors: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut acc_ok = 0;
    let mut acc_ko = 0;
    for line in contents.lines() {
        if line.contains("|") {
            let mut line_iter = line.split("|");
            let key = line_iter.next().unwrap().parse::<u32>().unwrap();
            let val = line_iter.next().unwrap().parse::<u32>().unwrap();
            if !successors.contains_key(&key) {
                successors.insert(key, Vec::<u32>::new());
            }
            successors.get_mut(&key).unwrap().push(val);
        }
        else if line.contains(",") {
            let mut pages = line.split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let mut ok = true;
            for (i, page) in pages.iter().enumerate() {
                if !successors.contains_key(&page) {
                    continue;
                }
                for k in 0..i {
                    if successors[page].contains(&pages[k]) {
                        ok = false;
                    }
                }
            }
            if ok {
                acc_ok += pages[pages.len() / 2];
            }
            else {
                pages.sort_by(|a, b| {
                    let a_successors = successors.get_key_value(&a);
                    let b_successors = successors.get_key_value(&b);
                    if a == b {
                        return a.cmp(b);
                    }
                    if let Some((_, x)) = a_successors {
                        if x.contains(&b) {
                            return Ordering::Less;
                        }
                    }
                    if let Some((_, x)) = b_successors {
                        if x.contains(&a) {
                            return Ordering::Greater;
                        }
                    }
                    return a.cmp(b);
                });
                acc_ko += pages[pages.len() / 2];
            }
            //println!("ok = {ok}");
        }
    }
    println!("acc_ok = {acc_ok}");
    println!("acc_ko = {acc_ko}");
}
