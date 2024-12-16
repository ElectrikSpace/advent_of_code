use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Equation {
    a: i64,
    b: i64,
    r: i64
}

fn try_solve(l1: Equation, l2: Equation) -> Option<(i64, i64)> {
    let d = l1.a*l2.b - l1.b*l2.a;
    if d == 0 {
        println!("aie!");
        return None;
    }
    let x = (l1.r*l2.b - l2.r*l1.b) / d;
    let y = (l2.r*l1.a - l1.r*l2.a) / d;
    if (x >= 0) && (y >= 0) && 
       ((x*l1.a + y*l1.b) == l1.r) && ((x*l2.a + y*l2.b) == l2.r) {
        return Some((x, y));
    }
    return None;
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    let mut tokens = 0;
    let mut tokens2 = 0;
    for machine in contents.split("\n\n") {
        // Parse
        let lines = machine.lines().collect::<Vec<_>>();
        let mut lines_iter = lines.iter();
        let line_a = lines_iter.next().unwrap().split(" ").collect::<Vec<_>>();
        let line_b = lines_iter.next().unwrap().split(" ").collect::<Vec<_>>();
        let line_price = lines_iter.next().unwrap().split(" ").collect::<Vec<_>>();
        let x_a = line_a[2]
            .strip_prefix("X+").unwrap()
            .strip_suffix(",").unwrap()
            .parse::<i64>().unwrap();
        let x_b = line_a[3]
            .strip_prefix("Y+").unwrap()
            .parse::<i64>().unwrap();
        let y_a = line_b[2]
            .strip_prefix("X+").unwrap()
            .strip_suffix(",").unwrap()
            .parse::<i64>().unwrap();
        let y_b = line_b[3]
            .strip_prefix("Y+").unwrap()
            .parse::<i64>().unwrap();
        let x_price = line_price[1]
            .strip_prefix("X=").unwrap()
            .strip_suffix(",").unwrap()
            .parse::<i64>().unwrap();
        let y_price = line_price[2]
            .strip_prefix("Y=").unwrap()
            .parse::<i64>().unwrap();

        // Part 1
        let mut l1 = Equation {a: x_a, b: y_a, r: x_price};
        let mut l2 = Equation {a: x_b, b: y_b, r: y_price};
        if let Some((s0, s1)) = try_solve(l1, l2) {
            tokens += s0*3 + s1;
        }

        // Part 2
        l1.r += 10000000000000;
        l2.r += 10000000000000;
        if let Some((s0, s1)) = try_solve(l1, l2) {
            tokens2 += s0*3 + s1;
        }
    }

    println!("tokens part1 = {tokens}");
    println!("tokens part2 = {tokens2}");
}
