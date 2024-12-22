use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Item {
    Wall,
    Start,
    End,
    Nice,
    Empty
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Grid {
    items: Vec<Vec<Item>>,
    height: usize,
    width: usize
}

impl Grid {
    fn new(height: usize, width: usize) -> Self {
        return Self {
            items: vec![vec![Item::Empty; width]; height],
            height: height,
            width: width
        };
    }

    fn set(&mut self, index: (i32, i32), value: Item) {
        if index.0 < 0 || index.0 >= (self.width as i32) ||
           index.1 < 0 || index.1 >= (self.height as i32) {
               return;
        }
        self.items[index.1 as usize][index.0 as usize] = value;
    }

    fn get(&self, index: (i32, i32)) -> Option<Item> {
        if index.0 < 0 || index.0 >= (self.width as i32) ||
           index.1 < 0 || index.1 >= (self.height as i32) {
               return None;
        }
        return Some(self.items[index.1 as usize][index.0 as usize].clone());
    }

    fn display(&self) {
        self.items.iter()
             .for_each(|a| {
                 a.iter()
                  .for_each(|b| {
                      let c = match b {
                        Item::Wall => '#',
                        Item::Start => 'S',
                        Item::End => 'E',
                        Item::Nice => 'O',
                        Item::Empty => '.',
                      };
                      print!("{c}");
                  });
                 println!("");
             })
    }
}

fn get_rotate_cost(dir: &Direction, new_dir: &Direction) -> usize {
    return match dir {
        Direction::Left => {
            match new_dir {
                Direction::Left => 0,
                Direction::Right => 2,
                Direction::Up => 1,
                Direction::Down => 1,
            }
        },
        Direction::Right => {
            match new_dir {
                Direction::Left => 2,
                Direction::Right => 0,
                Direction::Up => 1,
                Direction::Down => 1,
            }
        },
        Direction::Up => {
            match new_dir {
                Direction::Left => 1,
                Direction::Right => 1,
                Direction::Up => 0,
                Direction::Down => 2,
            }
        },
        Direction::Down => {
            match new_dir {
                Direction::Left => 1,
                Direction::Right => 1,
                Direction::Up => 2,
                Direction::Down => 0,
            }
        }
    };
}

fn find_best_path(grid: &Grid, start: (i32, i32), end: (i32, i32)) ->  HashMap<((i32, i32), Direction), (usize, usize, usize)> {
    let start_dir = Direction::Right;
    let mut scores = HashMap::<((i32, i32), Direction), (usize, usize, usize)>::new();
    let mut to_explore = vec![(start, start_dir.clone())];
    scores.insert((start, start_dir.clone()), (0, 0, 0));
    while !to_explore.is_empty() {
        let (current, dir) = to_explore.pop().unwrap();
        if grid.get(current) == None || grid.get(current) == Some(Item::Wall) {
            continue;
        }
        let (score, steps, turns) = *scores.get(&(current, dir.clone())).unwrap();
        if score == 65347 {
            println!("coucou je suis là en {:?}", current);
        }
        let next = vec![
            (Direction::Left, (current.0-1, current.1)),
            (Direction::Right, (current.0+1, current.1)),
            (Direction::Up, (current.0, current.1-1)),
            (Direction::Down, (current.0, current.1+1)),
            //(Direction::Left, current),
            //(Direction::Right, current),
            //(Direction::Up, current),
            //(Direction::Down, current),
        ];
        for (new_dir, n) in next {
            if grid.get(n) == None || grid.get(n) == Some(Item::Wall) {
                continue;
            }
            let new_dir_cost = get_rotate_cost(&dir, &new_dir);
            if new_dir_cost > 1 {
                continue;
            }
            let new_dir_score = 1000 * new_dir_cost;
            if scores.contains_key(&(n, new_dir.clone())) {
                if scores.get(&(n, new_dir.clone())).unwrap().0 > (score + 1 + new_dir_score) {
                    //if n == (135, 46) {
                    //    println!("From {:?} to {:?}", scores.get(&(n, new_dir.clone())).unwrap().0, score + 1 + new_dir_score);
                    //}
                    //*scores.get_mut(&(n, new_dir.clone())).unwrap() = (score + 1 + new_dir_score, steps + 1, turns + new_dir_cost);
                    scores.insert((n, new_dir.clone()), (score + 1 + new_dir_score, steps + 1, turns + new_dir_cost));
                    to_explore.push((n, new_dir.clone()));
                }
            }
            else {
                //if n == (135, 46) {
                //   println!("Inserting {:?}", scores.get(&(n, new_dir.clone())).unw, score + 1 + new_dir_cost);
                //}
                scores.insert((n, new_dir.clone()), (score + 1 + new_dir_score, steps + 1, turns + new_dir_cost));
                to_explore.push((n, new_dir.clone()));
            }
            //println!("end score is {:?}", scores.get(&end));
        }
    }
    //println!("end score left is {:?}", scores.get(&(end, Direction::Left)));
    //println!("end score right is {:?}", scores.get(&(end, Direction::Right)));
    //println!("end score up is {:?}", scores.get(&(end, Direction::Up)));
    //println!("end score down is {:?}", scores.get(&(end, Direction::Down)));
    //return scores.get(&(end, start_dir)).copied();
    return scores;
}

fn get_smallest_score_at(scores: &HashMap<((i32, i32), Direction), (usize, usize, usize)>,
                         pos: (i32, i32)) -> Option<(usize, Direction)> {
    let mut min_cost = usize::MAX;
    let mut local_mins = Vec::<Direction>::new();
    let dirs = vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];
    for dir in dirs {
        if scores.get(&(pos, dir.clone())) == None {
            continue;
        }
        let (cost, _, _) = scores.get(&(pos, dir.clone())).unwrap();
        if *cost < min_cost {
            min_cost = *cost;
            local_mins.clear();
            local_mins.push(dir);
        }
        else if *cost == min_cost {
            local_mins.push(dir);
        }
    } 
    if local_mins.len() == 0 {
        return None;
    }
    else if local_mins.len() > 1 {
        println!("aie!");
        return None;
    }
    return Some((min_cost, local_mins[0].clone()));
}

fn count_nice_tiles(scores: &HashMap<((i32, i32), Direction), (usize, usize, usize)>,
                    start_pos: (i32, i32), 
                    end_pos: (i32, i32),
                    grid: &mut Grid) -> usize {
    let smallest_score = get_smallest_score_at(scores, end_pos);
    if smallest_score == None {
        return 0;
    }
    let end_pos_dir = smallest_score.unwrap().1;
    let mut to_explore = vec![(end_pos, end_pos_dir)];
    let mut explored = HashSet::<(i32, i32)>::new();
    while !to_explore.is_empty() {
        let (current, current_dir) = to_explore.pop().unwrap();
        let (current_cost, _, _) = *scores.get(&(current, current_dir.clone())).unwrap();
        //if !explored.contains(&current) {
            explored.insert(current);
            grid.set(current, Item::Nice);
        //}
        if current == start_pos {
            continue;
        }
        let prevs = vec![
            (current.0+1, current.1),
            (current.0-1, current.1),
            (current.0, current.1+1),
            (current.0, current.1-1),
        ];
        let dirs = vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];
        let mut pushed = false;
        for prev in prevs.clone() {
            if  grid.get(prev) == None || grid.get(prev) == Some(Item::Wall) {
                continue;
            }
            for prev_dir in &dirs {
                if !scores.contains_key(&(prev, prev_dir.clone())) {
                    continue;
                }
                let rotate_cost = get_rotate_cost(&prev_dir, &current_dir);
                //if rotate_cost > 1 {
                //    continue;
                //}
                let cost = 1 + 1000*rotate_cost;
                let (prev_cost, _, _) = *scores.get(&(prev, prev_dir.clone())).unwrap();
                //println!("from cost {:?} to core {:?}", prev_cost, current_cost);
                if (prev_cost + cost) == current_cost {
                //if prev_cost < current_cost {
                    //println!("OK");
                    to_explore.push((prev, prev_dir.clone()));
                    pushed = true;
                }
            }
        }
        if !pushed {
            println!("NO PUSH!!!!!!!");
            let cscore = get_smallest_score_at(scores, current);
            println!("en {:?}", current);
            println!("-> {:?}", cscore);
            for prev in prevs {
                let pscore = get_smallest_score_at(scores, prev);
                println!("  - {:?}", pscore);
            }
            
        }
    }
    return explored.len();
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // Parse
    let contents_cut = contents.split("\n\n").collect::<Vec<_>>();
    let mut contents_cut_iter = contents_cut.iter();
    // Parse grid
    let contents_grid = contents_cut_iter.next().unwrap().trim();
    let height = contents_grid.lines().count();
    let width = contents_grid.lines().next().unwrap().len();
    let mut grid = Grid::new(height, width);
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    contents_grid.lines()
        .enumerate()
        .for_each(|(i, line)| {
            line.chars()
                .enumerate()
                .for_each(|(j, v)| {
                    let item = match v {
                        '#' => Item::Wall,
                        'S' => Item::Start,
                        'E' => Item::End,
                        '.' => Item::Empty,
                         _  => unreachable!()
                    };
                    if item == Item::Start {
                        start_pos = (j as i32, i as i32);
                    }
                    if item == Item::End {
                        end_pos = (j as i32, i as i32);
                    }
                    grid.items[i][j] = item;
                })
        });

    println!("Part1");
    println!("Initial state:");
    println!("S={:?}", start_pos);
    println!("E={:?}", end_pos);
    grid.display();
    let scores = find_best_path(&grid, start_pos, end_pos);
    let end_score = get_smallest_score_at(&scores, end_pos);
    println!("End score is {:?}", end_score);
    let nice_tiles_count = count_nice_tiles(&scores, start_pos, end_pos, &mut grid);
    grid.display();
    println!("nice tiles count is {:?}", nice_tiles_count);
}
