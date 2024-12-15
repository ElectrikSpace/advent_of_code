use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32
}

struct Grid {
    points: Vec<Vec<char>>,
    plants: HashMap<char, HashSet<Point>>,
    bound: Point
}

impl Grid {
    fn new(bound: Point) -> Self {
        return Self {
            points: vec![vec!['.'; bound.x as usize]; bound.y as usize],
            plants: HashMap::<char, HashSet<Point>>::new(),
            bound: bound
        };
    }

    fn insert(&mut self, p: Point, v: char) {
        if (p.x < self.bound.x) && (p.y < self.bound.y) &&
           (p.x >= 0) && (p.y >= 0) {
            self.points[p.y as usize][p.x as usize] = v;
            self.plants.entry(v).or_insert(HashSet::<Point>::new()).insert(p);
        }
    }

    fn get(&self, p: Point) -> char {
        if (p.x < self.bound.x) && (p.y < self.bound.y) &&
           (p.x >= 0) && (p.y >= 0) {
            return self.points[p.y as usize][p.x as usize];
        }
        return '.';
    }

    fn get_points(&self, plant: char) -> Option<&HashSet<Point>> {
        return self.plants.get(&plant);
    }
}

fn get_fences_cost_for_plant(grid: &Grid, plant: char) -> usize {
    let mut points = grid.get_points(plant).expect("no points found!").clone();
    let mut fences_cost = 0;
    //println!("{plant}");
    while !points.is_empty() {
        let mut region = vec![points.iter().next().unwrap().clone()];
        let mut local_fences = 0;
        let mut local_area = 0;
        while !region.is_empty() {
            let point = region.pop().unwrap();
            points.remove(&point);
            local_area += 1;
            let neighbours = vec![
                Point {x: point.x  , y:point.y-1},
                Point {x: point.x-1, y:point.y},
                Point {x: point.x+1, y:point.y},
                Point {x: point.x  , y:point.y+1},
            ];
            for n in neighbours {
                if grid.get(n) != plant {
                    local_fences += 1;
                }
                else if points.contains(&n) && !region.contains(&n){
                    region.push(n);
                }
            }
        }
        fences_cost += local_area * local_fences;
    }
    return fences_cost;
}

fn get_fences_cost_for_plant_bulk(grid: &Grid, plant: char) -> usize {
    let mut points = grid.get_points(plant).expect("no points found!").clone();
    let mut fences_cost = 0;
    //println!("{plant}");
    while !points.is_empty() {
        let mut region = vec![points.iter().next().unwrap().clone()];
        let mut sides = HashSet::<(Point, Point, Point, Point)>::new();
        let mut local_area = 0;
        while !region.is_empty() {
            let point = region.pop().unwrap();
            //println!("{:?}", point);
            points.remove(&point);
            local_area += 1;
            let neighbours = vec![
                Point {x: point.x  , y:point.y-1},
                Point {x: point.x-1, y:point.y},
                Point {x: point.x+1, y:point.y},
                Point {x: point.x  , y:point.y+1},
            ];
            for n in neighbours {
                if grid.get(n) != plant {
                    let mut side = (point, n, point, n);
                    for dir in vec![-1, 1] {
                        if n.y == point.y {
                            let mut next_p = Point {x: point.x, y: point.y+dir};
                            let mut next_n = Point {x: n.x, y: n.y+dir};
                            while (grid.get(next_n) != plant) &&
                                  (grid.get(next_p) == plant) {
                                if dir == -1 {
                                    side.0 = next_p;
                                    side.1 = next_n;
                                }
                                else {
                                    side.2 = next_p;
                                    side.3 = next_n;
                                }
                                next_p.y += dir;
                                next_n.y += dir;
                            }
                        }
                        else {
                            let mut next_p = Point {x: point.x+dir, y: point.y};
                            let mut next_n = Point {x: n.x+dir, y: n.y};
                            while (grid.get(next_n) != plant) &&
                                  (grid.get(next_p) == plant) {
                                if dir == -1 {
                                    side.0 = next_p;
                                    side.1 = next_n;
                                }
                                else {
                                    side.2 = next_p;
                                    side.3 = next_n;
                                }
                                next_p.x += dir;
                                next_n.x += dir;
                            }
                        }
                    }
                    sides.insert(side);
                }
                else if points.contains(&n) && !region.contains(&n){
                    region.push(n);
                }
            }
        }
        fences_cost += sides.len() * local_area;
    }
    return fences_cost;
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // Init structures
    let bound = Point {
        x: contents.lines().next().unwrap().len() as i32,
        y: contents.lines().count() as i32
    };
    let mut grid = Grid::new(bound);
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert(Point {x:i as i32, y:j as i32}, c);
        }
    }

    // Part 1
    let fences_cost = grid.plants.keys()
        .map(|plant| get_fences_cost_for_plant(&grid, *plant)) 
        .sum::<usize>();
    println!("Fence cost 1 = {fences_cost}");
    
    // Part 2
    let fences_cost2 = grid.plants.keys()
        .map(|plant| get_fences_cost_for_plant_bulk(&grid, *plant)) 
        .sum::<usize>();
    println!("Fence cost 2 = {fences_cost2}");
}
