use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

//#[derive(Clone, Copy)]
//enum Direction {
//    Up = 0,
//    Right = 1,
//    Down = 2,
//    Left = 3,
//}
//
//impl Direction {
//    fn turn_right(&mut self) {
//        *self = match self {
//            Direction::Up => Direction::Right,
//            Direction::Right => Direction::Down,
//            Direction::Down => Direction::Left,
//            Direction::Left => Direction::Up,
//        };
//    }
//}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

struct Grid {
    points: HashSet<Point>,
    bound: Point
}

impl Grid {
    fn new(bound: Point) -> Self {
        return Self {
            points: HashSet::<Point>::new(),
            bound: bound
        };
    }

    fn insert(&mut self, p: Point) {
        if (p.x < self.bound.x) && (p.y < self.bound.y) &&
           (p.x >= 0) && (p.y >= 0) {
            self.points.insert(p);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let bound = Point {
        x: contents.lines().next().unwrap().len() as i32,
        y: contents.lines().count() as i32
    };
    let mut antennas: HashMap<char, Grid> = HashMap::new();
    let mut antinodes = Grid::new(bound);
    
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
                    '.' => {},
                     _  => {
                        if !antennas.contains_key(&b) {
                            antennas.insert(b, Grid::new(bound));
                        }
                        antennas.get_mut(&b).unwrap().insert(Point {x:si, y:sj});
                    }
                }
             })
        });

    // Compute antinodes
    let _ = antennas.values()
                .for_each(|g| {
                    g.points
                        .iter()
                        .for_each(|p| {
                            g.points
                                .iter()
                                .filter(|&t| t != p)
                                .for_each(|t| {
                                    // distance vector
                                    let v = Point {
                                        x: t.x - p.x,
                                        y: t.y - p.y
                                    };
                                    // antinode 1
                                    antinodes.insert(Point {
                                        x: p.x - v.x,
                                        y: p.y - v.y
                                    });
                                    // antinode 2
                                    antinodes.insert(Point {
                                        x: t.x + v.x,
                                        y: t.y + v.y
                                    });
                                })
                    })
                });
    let antinodes_count = antinodes.points.len();
    println!("There are {antinodes_count} antinodes");
}
