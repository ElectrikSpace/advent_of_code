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

fn run(contents: &str, lab_map_base: &HashMap<i32, Vec<bool>>, obstacles_base: &HashSet<i32>, start: &(i32, i32), new_obstacle: Option<(i32, i32)>) -> (bool, usize, HashMap<i32, Vec<bool>>) {
    let width = contents.lines().next().unwrap().len() as i32;
    let height = contents.lines().count() as i32;
    
    let mut lab_map = lab_map_base.clone();
    let mut obstacles = obstacles_base.clone();

    let mut visited_count: usize = 0;
    let mut current = start.clone();
    let mut direction = Direction::Up;

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
            lab_map.get_mut(&index).unwrap()[direction.clone() as usize] = true;
            next = get_next(current, direction);
        }
        current = next;
    }

    return (cycle, visited_count, lab_map);
}

fn main() {
    let file_path = "./input";
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let width = contents.lines().next().unwrap().len() as i32;
    let height = contents.lines().count() as i32;
    let mut lab_map: HashMap<i32, Vec<bool>> = HashMap::new();
    let mut obstacles: HashSet<i32> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);
    
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
                        start = (si, sj);}
                    _ => {}
                }
             })
        });


    let (loop_detected, visited, visited_map) = run(&contents, &lab_map, &obstacles, &start, None);
    println!("Loop detected: {loop_detected}");
    println!("Visited is {visited}");

    //let mut forbidden: HashSet<i32> =HashSet::new();
    //let mut y = start.0;
    //while (y > 0) && (!obstacles.contains(&(y*width+start.1))) {
    //    forbidden.insert(y);
    //    y -= 1;
    //}

    let mut cycles = 0;
    for x in 0..height {
    //for x in 0..10 {
        for y in 0..width {
        //for y in 0..10 {
            println!("x={x}/y={y}");
            if !visited_map.contains_key(&(x*width+y)) {
                continue;
            }
            if visited_map[&(x*width+y)].iter().all(|a| *a == false) {
                continue;
            }
            if obstacles.contains(&(x*width+y)) {
                continue;
            }
            if (x == start.0) && (y == start.1) {
                continue;
            }
            //if (y == start.1) && forbidden.contains(&x) {
            //  continue;
            //}
            let (cycle, _, _) = run(&contents,  &lab_map, &obstacles, &start, Some((x, y)));
            if cycle == true {
               cycles += 1;
            }
        }
    }
    println!("cycles = {cycles}");
}
