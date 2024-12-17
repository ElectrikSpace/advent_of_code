use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Item {
    Wall,
    Start,
    End,
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
        if grid.get(current) == None {
            continue;
        }
        if grid.get(current) == Some(Item::Wall) {
            continue;
        }
        let (score, steps, turns) = *scores.get(&(current, dir.clone())).unwrap();
        let next = vec![
            (Direction::Left, (current.0-1, current.1)),
            (Direction::Right, (current.0+1, current.1)),
            (Direction::Up, (current.0, current.1-1)),
            (Direction::Down, (current.0, current.1+1)),
        ];
        for (new_dir, n) in next {
            let new_dir_cost = get_rotate_cost(&dir, &new_dir);
            if new_dir_cost > 1 {
                continue;
            }
            let new_dir_score = 1000 * new_dir_cost;
            if scores.contains_key(&(n, new_dir.clone())) {
                if scores.get(&(n, new_dir.clone())).unwrap().0 > (score + 1 + new_dir_cost) {
                    *scores.get_mut(&(n, new_dir.clone())).unwrap() = (score + 1 + new_dir_score, steps + 1, turns + new_dir_cost);
                    to_explore.push((n, new_dir.clone()));
                }
            }
            else {
                scores.insert((n, new_dir.clone()), (score + 1 + new_dir_score, steps + 1, turns + new_dir_cost));
                to_explore.push((n, new_dir.clone()));
            }
            //println!("end score is {:?}", scores.get(&end));
        }
    }
    println!("end score left is {:?}", scores.get(&(end, Direction::Left)));
    println!("end score right is {:?}", scores.get(&(end, Direction::Right)));
    println!("end score up is {:?}", scores.get(&(end, Direction::Up)));
    println!("end score down is {:?}", scores.get(&(end, Direction::Down)));
    //return scores.get(&(end, start_dir)).copied();
    return scores;
}

fn count_nice_tiles(scores: &HashMap<((i32, i32), Direction), (usize, usize, usize)>,
                    start_pos: (i32, i32), 
                    end_pos: (i32, i32)) -> usize {
    let mut to_explore = vec![end_pos];
    let mut explored = HashSet::<(i32, i32)>::new();
    while !to_explore.is_empty() {
        let current = to_explore.pop().unwrap();
        explored.insert(current);
        if current == start_pos {
            continue;
        }
        let dirs = vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];
        let mut min_cost = usize::MAX;
        let mut local_mins = Vec::<Direction>::new();
        for dir in dirs {
            if scores.get(&(current, dir.clone())) == None {
                continue;
            }
            let (cost, _, _) = scores.get(&(current, dir.clone())).unwrap();
            println!("cost is {cost}");
            if current == end_pos {
                if *cost < min_cost {
                    min_cost = *cost;
                    local_mins.clear();
                    local_mins.push(dir);
                }
                else if *cost == min_cost {
                    local_mins.push(dir);
                }
            }
            else {
                local_mins.push(dir);
            }
        } 
        for dir in local_mins {
            let next = match dir {
                Direction::Left => (current.0+1, current.1),
                Direction::Right => (current.0-1, current.1),
                Direction::Up => (current.0, current.1+1),
                Direction::Down => (current.0, current.1-1),
            };
            to_explore.push(next);
        }
    }
    return explored.len();
}

fn main() {
    //let contents = fs::read_to_string("itest1")
    let contents = fs::read_to_string("input_test")
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
    let score = find_best_path(&grid, start_pos, end_pos);
    let nice_tiles_count = count_nice_tiles(&score, start_pos, end_pos);
    println!("nice tiles count is {:?}", nice_tiles_count);
}
