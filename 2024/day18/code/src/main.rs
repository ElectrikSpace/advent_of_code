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

fn find_best_path(grid: &Grid, start: (i32, i32), end: (i32, i32)) ->  Option<usize> {
    let mut scores = HashMap::<(i32, i32), usize>::new();
    let mut to_explore = vec![start];
    scores.insert(start, 0);
    while !to_explore.is_empty() {
        let current = to_explore.pop().unwrap();
        if grid.get(current) == None || grid.get(current) == Some(Item::Wall) {
            continue;
        }
        let score = *scores.get(&current).unwrap();
        let next = vec![
            (current.0-1, current.1),
            (current.0+1, current.1),
            (current.0, current.1-1),
            (current.0, current.1+1),
        ];
        for n in next {
            if grid.get(n) == None || grid.get(n) == Some(Item::Wall) {
                continue;
            }
            if scores.contains_key(&n) {
                if *scores.get(&n).unwrap() > (score + 1) {
                    scores.insert(n, score + 1);
                    to_explore.push(n);
                }
            }
            else {
                scores.insert(n, score + 1);
                to_explore.push(n);
            }
        }
    }
    return scores.get(&end).copied();
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // Parse
    //let height = 7;
    //let width = 7;
    //let taken_bytes = 12;
    let height = 71;
    let width = 71;
    let taken_bytes = 1024;
    let mut grid = Grid::new(height, width);
    let mut start_pos = (0, 0);
    let mut end_pos = ((width-1) as i32, (height-1) as i32);
    let bytes = contents
        .lines()
        .map(|l| {
            let p = l.split(',')
                     .map(|x| x.parse::<i32>().unwrap())
                     .collect::<Vec<i32>>();
            return (p[0], p[1]);
        })
        .collect::<Vec<(i32, i32)>>();
    bytes.iter()
         .take(taken_bytes)
         .for_each(|&b| grid.set(b, Item::Wall));
    grid.set(start_pos, Item::Start);
    grid.set(end_pos, Item::End);

    // Part1
    println!("Part1");
    grid.display();
    let score = find_best_path(&grid, start_pos, end_pos);
    println!("End score is {:?}", score);

    // Part2
    // It's brut force, bit it works :)
    let mut current_score = score;
    let mut current_byte = taken_bytes - 1;
    while current_score != None {
        current_byte += 1;
        grid.set(bytes[current_byte], Item::Wall);
        println!("Adding byte number {current_byte} : {:?}", bytes[current_byte]);
        current_score = find_best_path(&grid, start_pos, end_pos);
    }
    println!("First blocking byte is at {:?}", bytes[current_byte]);
}
