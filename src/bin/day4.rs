fn main() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let input = include_str!("../../input/d4.txt");

    let input: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("p1: {p1}, p2: {p2}");
}

fn is_paper(input: &[Vec<char>], i: isize, j: isize) -> bool {
    if i < 0 || i >= input.len() as isize || j < 0 || j >= input[i as usize].len() as isize {
        false
    } else {
        input[i as usize][j as usize] == '@'
    }
}

fn count_paper_around(input: &[Vec<char>], i: isize, j: isize) -> isize {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if (x != 0 || y != 0) && is_paper(input, i + x, j + y) {
                count += 1;
            }
        }
    }

    count
}

fn print(input: &[Vec<char>]) {
    for line in input {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

fn part1(input: &[Vec<char>]) -> i32 {
    let mut input_clone = input.to_vec();
    let mut count = 0;
    for i in 0..input.len() as isize {
        for j in 0..input[i as usize].len() as isize {
            if input[i as usize][j as usize] == '@' && count_paper_around(input, i, j) < 4 {
                count += 1;
                input_clone[i as usize][j as usize] = 'x';
            }
        }
    }
    // print(&input_clone);
    count
}

fn part2(input: &[Vec<char>]) -> i32 {
    let mut input = input.to_vec();
    let mut count = 0;
    loop {
        let mut changed = false;
        for i in 0..input.len() as isize {
            for j in 0..input[i as usize].len() as isize {
                if input[i as usize][j as usize] == '@' && count_paper_around(&input, i, j) < 4 {
                    count += 1;
                    input[i as usize][j as usize] = 'x';
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }
    // print(&input);
    count
}
