use std::fs;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Robot {
    start: Point,
    speed: Point,
    current: Point
}

impl Robot {
    fn new(start: Point, speed: Point) -> Self {
        return Self {
            start: start,
            speed: speed,
            current: start
        };
    }

    fn move_n(&mut self, n: i64, grid_bound: &Point) {
        self.current.x = (self.current.x + self.speed.x*n).rem_euclid(grid_bound.x);
        self.current.y = (self.current.y + self.speed.y*n).rem_euclid(grid_bound.y);
    }

    fn reset(&mut self) {
        self.current = self.start;
    }
}

fn display_robots(robots: &Vec<Robot>, grid_bound: &Point) -> bool {
    let mut grid = vec![vec!['.'; grid_bound.x as usize]; grid_bound.y as usize];
    robots.iter()
        .for_each(|robot| {
            grid[robot.current.y as usize][robot.current.x as usize] = 'x';
        });
    grid.iter()
        .for_each(|line| {
            let line_str = line.iter().collect::<String>();
            println!("{line_str}");
        });
    // try to search for a line
    for line in grid.iter() {
        let line_str = line.iter().collect::<String>();
        if line_str.contains("xxxxxxxx") {
           return true;
        }
    }
    return false; 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    //let grid_bound = Point {x: 11, y: 7};
    let grid_bound = Point {x: 101, y: 103};
    let mut robots = Vec::<Robot>::new();
    for line in contents.lines() {
        // Parse
        let cut = line.trim().split(" ").collect::<Vec<_>>();
        let mut cut_iter = cut.iter();
        let pos_v = cut_iter.next().unwrap()
            .strip_prefix("p=").unwrap()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let speed_v = cut_iter.next().unwrap()
            .strip_prefix("v=").unwrap()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        // Create robot
        let pos = Point {x: pos_v[0], y: pos_v[1]};
        let speed = Point {x: speed_v[0], y: speed_v[1]};
        robots.push(Robot::new(pos, speed));
    }

    // 100 seconds
    let mut quadrants_count = vec![0, 0, 0, 0];
    let quadrant_bound = Point {
        x: grid_bound.x / 2,
        y: grid_bound.y / 2
    };
    for mut robot in &mut robots {
        robot.move_n(100, &grid_bound);
        if robot.current.x == quadrant_bound.x ||
           robot.current.y == quadrant_bound.y {
            continue;
        }
        let index = 2*(robot.current.x / (quadrant_bound.x + 1))
                    + (robot.current.y / (quadrant_bound.y + 1));
        quadrants_count[index as usize] += 1;
    }
    println!("quadrants {:?}", quadrants_count);
    let safety_factor = quadrants_count.iter()
        .fold(1, |acc, x| acc * x);
    println!("safety_factor = {safety_factor}");

    robots.iter_mut().for_each(|x| x.reset());
    for i in 0..=10000 {
        println!("****************** I={i} ******************");
        if !display_robots(&robots, &grid_bound) {
            robots.iter_mut().for_each(|x| x.move_n(1, &grid_bound));
            continue;
        }
        terminal::enable_raw_mode()?;
        if let Event::Key(key_event) = event::read()? {
           match key_event.code {
               KeyCode::Char('y') => {
               }
               _ => {
                   break;
               }
           }
        }
        terminal::disable_raw_mode()?;
        robots.iter_mut().for_each(|x| x.move_n(1, &grid_bound));
    }
    terminal::disable_raw_mode()?;

    Ok(())
}
