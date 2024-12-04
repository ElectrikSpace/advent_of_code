use std::fs;
use itertools::Itertools;

fn convolution3x3(mat: &Vec<Vec<char>>, kernel: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    return itertools::iproduct!(0..3, 0..3)
        .map(|(i, j)| (&mat[x+i][y+j] == &kernel[i][j]) as i32)
        .sum();
}

fn convolution4x4(mat: &Vec<Vec<char>>, kernel: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    return itertools::iproduct!(0..4, 0..4)
        .map(|(i, j)| (&mat[x+i][y+j] == &kernel[i][j]) as i32)
        .sum();
}

fn main() {
    let file_path = "./input";
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let length = contents.lines().next().unwrap().len();
    let horiz_padded_mat: Vec<Vec<char>> = contents
        .lines()
        .map(|l| format!("...{}...", l).chars().collect())
        .collect();
    let pad: Vec<Vec<char>> = vec![vec!['.'; length+6]; 3];
    let mat = [pad.clone(), horiz_padded_mat, pad].concat();
    //println!("{:?}", mat);

    // Part 1
    let k0: Vec<Vec<char>> = vec![vec!['X', 'M', 'A', 'S'],
                                  vec!['_', '_', '_', '_'],
                                  vec!['_', '_', '_', '_'],
                                  vec!['_', '_', '_', '_']];
    let k1: Vec<Vec<char>> = vec![vec!['S', 'A', 'M', 'X'],
                                  vec!['_', '_', '_', '_'],
                                  vec!['_', '_', '_', '_'],
                                  vec!['_', '_', '_', '_']];
    let k2: Vec<Vec<char>> = vec![vec!['X', '_', '_', '_'],
                                  vec!['M', '_', '_', '_'],
                                  vec!['A', '_', '_', '_'],
                                  vec!['S', '_', '_', '_']];
    let k3: Vec<Vec<char>> = vec![vec!['S', '_', '_', '_'],
                                  vec!['A', '_', '_', '_'],
                                  vec!['M', '_', '_', '_'],
                                  vec!['X', '_', '_', '_']];
    let k4: Vec<Vec<char>> = vec![vec!['X', '_', '_', '_'],
                                  vec!['_', 'M', '_', '_'],
                                  vec!['_', '_', 'A', '_'],
                                  vec!['_', '_', '_', 'S']];
    let k5: Vec<Vec<char>> = vec![vec!['S', '_', '_', '_'],
                                  vec!['_', 'A', '_', '_'],
                                  vec!['_', '_', 'M', '_'],
                                  vec!['_', '_', '_', 'X']];
    let k6: Vec<Vec<char>> = vec![vec!['_', '_', '_', 'S'],
                                  vec!['_', '_', 'A', '_'],
                                  vec!['_', 'M', '_', '_'],
                                  vec!['X', '_', '_', '_']];
    let k7: Vec<Vec<char>> = vec![vec!['_', '_', '_', 'X'],
                                  vec!['_', '_', 'M', '_'],
                                  vec!['_', 'A', '_', '_'],
                                  vec!['S', '_', '_', '_']];
    let kernels = vec![k0, k1, k2, k3, k4, k5, k6, k7];
    let count = itertools::iproduct!(&kernels, 0..length, 0..length)
        .map(|(k, x, y)| convolution4x4(&mat, &k, 3+x, 3+y))
        .filter(|r| *r == 4)
        .count();
    println!("Part1 {count}");

    // Part 2
    let xk0: Vec<Vec<char>> = vec![vec!['M', '_', 'M'],
                                   vec!['_', 'A', '_'],
                                   vec!['S', '_', 'S']];
    let xk1: Vec<Vec<char>> = vec![vec!['M', '_', 'S'],
                                   vec!['_', 'A', '_'],
                                   vec!['M', '_', 'S']];
    let xk2: Vec<Vec<char>> = vec![vec!['S', '_', 'M'],
                                   vec!['_', 'A', '_'],
                                   vec!['S', '_', 'M']];
    let xk3: Vec<Vec<char>> = vec![vec!['S', '_', 'S'],
                                   vec!['_', 'A', '_'],
                                   vec!['M', '_', 'M']];
    let xkernels = vec![xk0, xk1, xk2, xk3];
    let xcount = itertools::iproduct!(&xkernels, 0..length, 0..length)
        .map(|(k, x, y)| convolution3x3(&mat, &k, 3+x, 3+y))
        .filter(|r| *r == 5)
        .count();
    println!("Part2 {xcount}");
}
