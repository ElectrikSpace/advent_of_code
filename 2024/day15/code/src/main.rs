use std::fs;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Item {
    Robot,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
    Empty
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
                        Item::Empty => '.',
                        Item::Robot => '@',
                        Item::Box => 'O',
                        Item::BoxLeft => '[',
                        Item::BoxRight => ']',
                        Item::Wall => '#',
                      };
                      print!("{c}");
                  });
                 println!("");
             })
    }
    
    fn compute_sum(&self) -> i64 {
        // Compute sum of GPS coords
        return self.items
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.iter()
                .enumerate()
                .filter(|(_, v)| (**v == Item::Box) || (**v == Item::BoxLeft))
                .map(|(x, v)| 100*(y as i64) + (x as i64))
                .sum::<i64>()
            })
            .sum::<i64>();
    }
}

fn try_push_left(grid: &mut Grid, pos: &((i32, i32), (i32, i32))) -> bool {
    let next = (pos.0.0-1, pos.0.1);
    if grid.get(next) == Some(Item::Empty) {}
    else if grid.get(next) == Some(Item::BoxRight) {
        if !try_push_left(grid, &((next.0-1, next.1), next)) {
            return false;
        }
    }
    else {
        return false;
    }
    grid.set(next, Item::BoxLeft);
    grid.set(pos.0, Item::BoxRight);
    grid.set(pos.1, Item::Empty);
    return true;
}

fn try_push_right(grid: &mut Grid, pos: &((i32, i32), (i32, i32))) -> bool {
    let next = (pos.1.0+1, pos.1.1);
    if grid.get(next) == Some(Item::Empty) {}
    else if grid.get(next) == Some(Item::BoxLeft) {
        if !try_push_right(grid, &(next, (next.0+1, next.1))) {
            return false;
        }
    }
    else {
        return false;
    }
    grid.set(next, Item::BoxRight);
    grid.set(pos.1, Item::BoxLeft);
    grid.set(pos.0, Item::Empty);
    return true;
}

fn try_push_vert(grid: &mut Grid, vel: &(i32, i32), pos: &((i32, i32), (i32, i32))) -> bool {
    let next = ((pos.0.0+vel.0, pos.0.1+vel.1), (pos.1.0+vel.0, pos.1.1+vel.1));
    if grid.get(next.0) == Some(Item::Empty) && grid.get(next.1) == Some(Item::Empty) {}
    else if grid.get(next.0) == Some(Item::BoxLeft) && grid.get(next.1) == Some(Item::BoxRight) {
        if !try_push(grid, vel, &next) {
            return false;
        }
    }
    else if grid.get(next.0) == Some(Item::BoxRight) && grid.get(next.1) == Some(Item::Empty) {
        let shifted_next = ((next.0.0-1, next.0.1), next.0);
        if !try_push(grid, vel, &shifted_next) {
            return false;
        }
    }
    else if grid.get(next.0) == Some(Item::Empty) && grid.get(next.1) == Some(Item::BoxLeft) {
        let shifted_next = (next.1, (next.1.0+1, next.1.1));
        if !try_push(grid, vel, &shifted_next) {
            return false;
        }
    }
    else if grid.get(next.0) == Some(Item::BoxRight) && grid.get(next.1) == Some(Item::BoxLeft) {
        let shifted_next = ((next.0.0-1, next.0.1), next.0);
        let shifted_next2 = (next.1, (next.1.0+1, next.1.1));
        let saved_grid = grid.clone();
        if !try_push(grid, vel, &shifted_next) {
            *grid = saved_grid;
            return false;
        }
        if !try_push(grid, vel, &shifted_next2) {
            *grid = saved_grid;
            return false;
        }
    }
    else {
        return false;
    }
    grid.set(next.0, Item::BoxLeft);
    grid.set(next.1, Item::BoxRight);
    grid.set(pos.0, Item::Empty);
    grid.set(pos.1, Item::Empty);
    return true;
}

fn try_push(grid: &mut Grid, vel: &(i32, i32), pos: &((i32, i32), (i32, i32))) -> bool {
    if vel.1 == 0 {
        if vel.0 == -1 {
            return try_push_left(grid, pos);
        }
        else {
            return try_push_right(grid, pos);
        }
    }
    else {
        return try_push_vert(grid, vel, pos);
    }
}

fn apply_move(m: &char, grid: &mut Grid, pos: &mut(i32, i32)) {
    let vel: (i32, i32) = match m {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
         _  => unreachable!()
    };
    let new_pos = ((pos.0 as i32) + vel.0, (pos.1 as i32) + vel.1);
    println!("{m} pos={:?}, new_pos={:?}", pos, new_pos);
    if grid.get(new_pos) == None {
        println!("Aie!");
        return;
    }
    else if grid.get(new_pos) == Some(Item::Wall) {
        return;
    }
    else if grid.get(new_pos) == Some(Item::Box) {
        // Find an empty place in the line
        let mut empty_pos = new_pos;
        while grid.get(empty_pos) == Some(Item::Box) {
            empty_pos = (empty_pos.0 + vel.0, empty_pos.1 + vel.1);
        }
        if grid.get(empty_pos) != Some(Item::Empty) {
            // Act like a wall
            return;
        }
        // Move boxes and robot
        grid.set(empty_pos, Item::Box);
        grid.set(new_pos, Item::Robot);
        grid.set(pos.clone(), Item::Empty);
        *pos = new_pos;
    }
    else if (grid.get(new_pos) == Some(Item::BoxLeft)) ||
            (grid.get(new_pos) == Some(Item::BoxRight)) {
        // Get box couple
        let boxes = match grid.get(new_pos) {
            Some(Item::BoxLeft) => (new_pos, (new_pos.0 + 1, new_pos.1)),
            Some(Item::BoxRight) => ((new_pos.0 - 1, new_pos.1), new_pos),
            _ => unreachable!()
        };
        if try_push(grid, &vel, &boxes) == true {
            grid.set(new_pos, Item::Robot);
            grid.set(pos.clone(), Item::Empty);
            *pos = new_pos;
        }
    }
    else { // Item::Empty
        grid.set(new_pos, Item::Robot);
        grid.set(pos.clone(), Item::Empty);
        *pos = new_pos;
    }
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
    let mut robot_pos = (0, 0);
    let mut grid2 = Grid::new(height, 2*width);
    let mut robot_pos2 = (0, 0);
    contents_grid.lines()
        .enumerate()
        .for_each(|(i, line)| {
            line.chars()
                .enumerate()
                .for_each(|(j, v)| {
                    let item = match v {
                        '#' => Item::Wall,
                        'O' => Item::Box,
                        '@' => Item::Robot,
                        '.' => Item::Empty,
                         _  => unreachable!()
                    };
                    let items2 = match v {
                        '#' => (Item::Wall, Item::Wall),
                        'O' => (Item::BoxLeft, Item::BoxRight),
                        '@' => (Item::Robot, Item::Empty),
                        '.' => (Item::Empty, Item::Empty),
                         _  => unreachable!()
                    };
                    if item == Item::Robot {
                        robot_pos = (j as i32, i as i32);
                        robot_pos2 = (2*j as i32, i as i32);
                    }
                    grid.items[i][j] = item;
                    grid2.items[i][2*j] = items2.0;
                    grid2.items[i][2*j + 1] = items2.1;
                })
        });
    // Parse moves
    let contents_moves = contents_cut_iter.next().unwrap().trim();
    let moves = contents_moves.lines()
        .flat_map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    println!("Part1");
    println!("Initial state:");
    grid.display();
    moves.iter()
         .for_each(|m| {
            apply_move(&m, &mut grid, &mut robot_pos);
         });
    println!("After moves:");
    grid.display();
    let sum = grid.compute_sum();
    println!("sum is {sum}");

    println!("Part2");
    println!("Initial state:");
    grid2.display();
    moves.iter()
         .for_each(|m| {
            apply_move(&m, &mut grid2, &mut robot_pos2);
         });
    println!("After moves:");
    grid2.display();
    let sum2 = grid2.compute_sum();
    println!("sum2 is {sum2}");

}
