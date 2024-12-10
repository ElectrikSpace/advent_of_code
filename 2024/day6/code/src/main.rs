use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
//use std::cmp::Ordering;
//use itertools::Itertools;

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Direction {
        return match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => Direction::Up
        }
    }
}

fn get_next(current: (i32, i32), direction: Direction) -> (i32, i32) {
     return match direction {
         Direction::Up => (current.0 - 1, current.1),
         Direction::Down => (current.0 + 1, current.1),
         Direction::Left => (current.0, current.1 - 1),
         Direction::Right => (current.0, current.1 + 1),
     };
}

fn run(contents: &str, new_obstacle: Option<(i32, i32)>) -> (bool, usize) {
    let width = contents.lines().next().unwrap().len() as i32;
    let height = contents.lines().count() as i32;
    
    let mut lab_map: HashMap<i32, Vec<bool>> = HashMap::new();
    let mut obstacles: HashSet<i32> = HashSet::new();

    let mut visited_count: usize = 0;
    let mut current: (i32, i32) = (0, 0);
    let mut direction = Direction::Up;

    // Init structures
    let _ = contents.lines()
        .enumerate()
        .for_each(|(i, a)| {
            let si = i as i32;
            a.chars()
             .enumerate()
             .for_each(|(j, b)| {
                let sj = j as i32;
                match b {
                    '.' => {lab_map.insert((si*width + sj) as i32, 
                                           vec![false, false, false, false]);},
                    '#' => {obstacles.insert((si*width + sj) as i32);},
                    '^' => {
                        lab_map.insert((si*width + sj) as i32,
                                       vec![false, false, false, false]);
                        current = (si, sj);}
                    _ => {}
                }
             })
        });

    match new_obstacle {
        Some(o) => {obstacles.insert(o.0*width + o.1);},
        None => {}
    }
    
    // Run
    let mut cycle = false;
    while (current.0 >= 0 && current.0 < height) && 
          (current.1 >= 0 && current.1 < width) {
        if lab_map[&(current.0*width + current.1)][direction.clone() as usize] == true {
            cycle = true;
            break;
        }
        let index = current.0*width + current.1;
        if lab_map[&index].iter().all(|x| *x == false) {
            visited_count += 1;
        }
        lab_map.get_mut(&index).unwrap()[direction.clone() as usize] = true;
        let mut next = get_next(current, direction);
        while obstacles.contains(&(next.0*width + next.1)) {
            direction = Direction::from(((direction as usize) + 1) % 4);
            next = get_next(current, direction);
        }
        current = next;
    }

    return (cycle, visited_count);
}

fn main() {
    let file_path = "./input";
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (loop_detected, visited) = run(&contents, None);
    println!("Loop detected: {loop_detected}");
    println!("Visited is {visited}");
    //
    //let mut lab_map: HashMap<i32, Vec<bool>> = HashMap::new();
    //let mut obstacles: HashSet<i32> = HashSet::new();

    //let mut visited_count = 0;
    //let mut current: (i32, i32) = (0, 0);
    //let mut direction = Direction::Up;

    //// Init structures
    //let _ = contents.lines()
    //    .enumerate()
    //    .for_each(|(i, a)| {
    //        let si = i as i32;
    //        a.chars()
    //         .enumerate()
    //         .for_each(|(j, b)| {
    //            let sj = j as i32;
    //            match b {
    //                '.' => {lab_map.insert((si*width + sj) as i32, 
    //                                       vec![false, false, false, false]);},
    //                '#' => {obstacles.insert((si*width + sj) as i32);},
    //                '^' => {
    //                    lab_map.insert((si*width + sj) as i32,
    //                                   vec![false, false, false, false]);
    //                    current = (si, sj);}
    //                _ => {}
    //            }
    //         })
    //    });
    //
    //// Run
    //while (current.0 >= 0 && current.0 < height) && 
    //      (current.1 >= 0 && current.1 < width) &&
    //      (lab_map[&(current.0*width + current.1)][direction.clone() as usize] == false) {
    //    let index = current.0*width + current.1;
    //    if lab_map[&index].iter().all(|x| *x == false) {
    //        visited_count += 1;
    //    }
    //    lab_map.get_mut(&index).unwrap()[direction.clone() as usize] = true;
    //    let mut next = get_next(current, direction);
    //    while obstacles.contains(&(next.0*width + next.1)) {
    //        direction = Direction::from(((direction as usize) + 1) % 4);
    //        next = get_next(current, direction);
    //    }
    //    current = next;
    //}

    //if lab_map[&(current.0*width + current.1)][direction.clone() as usize] == true {
    //    println!("Loop detected");
    //}
    //println!("Visited is {visited_count}");

    let width = contents.lines().next().unwrap().len() as i32;
    let height = contents.lines().count() as i32;
    let mut cycles = 0;
    for x in 0..height {
    //for x in 0..10 {
        for y in 0..width {
        //for y in 0..10 {
            println!("x={x}/y={y}");
            let (cycle, _) = run(&contents, Some((x, y)));
            if cycle {
                cycles += 1;
            }
        }
    }
    println!("cycles = {cycles}");
}
