use std::fs;
use std::collections::HashMap;

struct Stones {
    current: HashMap<i64, u64>
}

impl Stones {
    fn new(initial_stones: Vec<i64>) -> Self {
        let mut current = HashMap::<i64, u64>::new();
        for s in initial_stones {
            if current.contains_key(&s) {
                *current.get_mut(&s).unwrap() += 1;
            }
            else {
                current.insert(s, 1);
            }
        }
        return Self {
            current: current
        };
    }

    fn len(&self) -> u64 {
        return self.current.values().sum();
    }

    fn blink(&mut self) {
        let mut new = HashMap::<i64, u64>::new();
        for (stone, count) in &self.current {
            // Rule 1
            if *stone == 0 {
                *new.entry(1).or_insert(0) += count;
                continue;
            }
            let digit_count = stone.ilog10() + 1;
            let p = 10_i64.pow(digit_count/2);
            // Rule 2
            if (digit_count % 2) == 0 {
                *new.entry(stone / p).or_insert(0) += count;
                *new.entry(stone % p).or_insert(0) += count;
                continue;
            }
            // Rule 3
            *new.entry(stone * 2024).or_insert(0) += count;
        }
        self.current = new;
    }
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // Init structures
    let initial_stones = contents.lines().next().unwrap().trim()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut stones = Stones::new(initial_stones);

    // Part 1
    for _ in 0..25 {
        stones.blink();
    }
    let stones_count = stones.len();
    println!("number of stones after 25 blinks is {stones_count}");

    // Part 2
    for i in 0..50 {
        stones.blink();
    }
    let stones_count2 = stones.len();
    println!("number of stones after 75 blinks is {stones_count2}");
}
