use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

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

    fn insert(&mut self, p: Point) -> bool {
        if (p.x < self.bound.x) && (p.y < self.bound.y) &&
           (p.x >= 0) && (p.y >= 0) {
            self.points.insert(p);
            return true;
        }
        else {
            return false;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // Init structures
    let bound = Point {
        x: contents.lines().next().unwrap().len() as i32,
        y: contents.lines().count() as i32
    };
    let mut antennas: HashMap<char, Grid> = HashMap::new();
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
    let mut antinodes = Grid::new(bound);
    let mut antinodes2 = Grid::new(bound);
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
                                    let mut a1 = Point {
                                        x: p.x - v.x,
                                        y: p.y - v.y
                                    };
                                    // antinode 2
                                    let mut a2 = Point {
                                        x: t.x + v.x,
                                        y: t.y + v.y
                                    };
                                    // part 1
                                    antinodes.insert(a1);
                                    antinodes.insert(a2);
                                    // part 2
                                    antinodes2.insert(*p);
                                    antinodes2.insert(*t);
                                    while antinodes2.insert(a1) {
                                        a1.x = a1.x - v.x;
                                        a1.y = a1.y - v.y;
                                    }
                                    while antinodes2.insert(a2) {
                                        a2.x = a2.x + v.x;
                                        a2.y = a2.y + v.y;
                                    }
                                })
                    })
                });

    println!("Part1: There are {} antinodes", antinodes.points.len());
    println!("Part2: There are {} antinodes", antinodes2.points.len());
}
