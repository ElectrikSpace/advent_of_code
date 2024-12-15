use std::fs;

fn compute_checksum(disk: &Vec<i32>) -> i64 {
    return disk.iter()
        .enumerate()
        .filter(|(_, x)| **x >= 0)
        .map(|(i, x)| (i as i64 ) * (*x as i64))
        .reduce(|a, b| a + b)
        .unwrap();
}

fn defragment_part1(disk: &mut Vec<i32>) {
    let mut fi = 0;
    let mut bi = disk.len() - 1;
    while fi < bi {
        // Used place
        if disk[fi] >= 0 {
            fi += 1;
            continue;
        }
        // Free place
        while disk[bi] < 0 {
            if fi == (bi - 1) {
                break;
            }
            bi -= 1;
        }
        disk[fi] = disk[bi];
        disk[bi] = -1;
        fi += 1;
    }
}

fn defragment_part2(disk: &mut Vec<i32>) {
    let mut bi = disk.len() - 1;
    while bi > 0 {
        // Free place
        if disk[bi] < 0 {
            bi -= 1;
            continue;
        }

        // Used block
        // compute size of block
        let id = disk[bi];
        let mut size = 1;
        while ((bi-1) > 0) && (disk[bi-1] == id) {
            size += 1;
            bi -= 1;
        }

        // try to find a place to fit
        let mut fi = 0;
        let mut free_size = 0;
        while fi < bi {
            if disk[fi] < 0 {
                free_size = 1;
                while (fi+free_size < bi) && (disk[fi+free_size] < 0) {
                    free_size += 1;
                }
                if free_size >= size {
                    break;
                }
                fi += free_size;
            }
            else {
                fi += 1;
            }
        }

        // replace if place found
        if free_size >= size {
            for i in 0..size {
                disk[fi + i] = disk[bi + i];
                disk[bi + i] = -1;
            }
        }
        bi -= 1;
    }
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");

    // Init structures
    let mut disk = contents.lines().next().unwrap().chars()
        .enumerate()
        .flat_map(|(i, x)| {
            let id = match i%2 {
                0 => (i/2) as i32,
                _ => -1
            };
            return std::iter::repeat(id)
                .take(x.to_digit(10).unwrap() as usize);
        })
        .collect::<Vec<i32>>();

    let mut part1_disk = disk.clone();
    defragment_part1(&mut part1_disk);
    let part1_checksum = compute_checksum(&part1_disk);
    println!("Part1 checksum={part1_checksum}");

    let mut part2_disk = disk.clone();
    defragment_part2(&mut part2_disk);
    let part2_checksum = compute_checksum(&part2_disk);
    println!("Part2 checksum={part2_checksum}");
}
