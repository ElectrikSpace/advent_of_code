use std::fs;
use std::collections::HashSet;
use itertools::iproduct;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

struct Grid {
    points: Vec<Vec<i32>>,
    bound: Point
}

impl Grid {
    fn new(bound: Point) -> Self {
        return Self {
            points: vec![vec![0; bound.x as usize]; bound.y as usize],
            bound: bound
        };
    }

    fn get(&self, index: Point) -> i32 {
        if (index.x < self.bound.x) && (index.y < self.bound.y) &&
           (index.x >= 0) && (index.y >= 0) {
            return self.points[index.y as usize][index.x as usize];
        }
        return -1;
    }
}

fn get_reachable_summits(grid: &Grid, summits: &mut HashSet<Point>, from: Point) -> usize {
    let height = grid.get(from);
    if height == 9 {
        summits.insert(from);
        return 1;
    }
    let neighbours = vec![
        Point {x: from.x, y: from.y + 1},
        Point {x: from.x, y: from.y - 1},
        Point {x: from.x + 1, y: from.y},
        Point {x: from.x - 1, y: from.y},
    ];
    return neighbours.iter()
        .filter(|n| grid.get(**n) == (height + 1))
        .map(|n| get_reachable_summits(grid, summits, *n))
        .sum();
}

fn compute_score_and_rating(grid: &Grid, trailhead: Point) -> (usize, usize) {
    let mut summits = HashSet::<Point>::new();
    let rating = get_reachable_summits(grid, &mut summits, trailhead);
    return (summits.len(), rating);
}

fn compute_rating(grid: &Grid, trailhead: Point) -> usize {
    let mut summits = HashSet::<Point>::new();
    let rating = get_reachable_summits(grid, &mut summits, trailhead);
    return rating;
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
    let _ = contents.lines()
        .enumerate()
        .for_each(|(i, a)| {
            a.chars()
             .enumerate()
             .for_each(|(j, b)| {
                grid.points[i][j] = b.to_digit(10).unwrap() as i32;
             })
        });

    // Get trail heads
    let trailheads = iproduct!(0..bound.x, 0..bound.y)
        .map(|(x, y)| Point {x: x, y: y})
        .filter(|&p| grid.get(p) == 0)
        .collect::<Vec<Point>>();

    let mut rating = 0; 
    let score = trailheads.iter()
        .map(|&p| {
            let (local_score, local_rating) = compute_score_and_rating(&grid, p);
            rating += local_rating;
            return local_score;
        })
        .sum::<usize>();

    // Part1
    println!("score is {score}");

    // Part2
    println!("rating is {rating}");
}
