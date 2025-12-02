fn main() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let input = include_str!("../../input/d1.txt");

    let input = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(1);
            (first, second.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let p1 = part1(&input);
    let p2 = part2(&input);
    let p2e = part2_enhanced(&input);
    println!("p1: {p1}, p2: {p2} p2e: {p2e}");
}

fn part1(input: &[(&str, i32)]) -> i32 {
    let mut count = 0;
    let mut pos = 50;
    for (d, v) in input {
        match *d {
            "R" => {
                pos = (pos + v) % 100;
            }
            "L" => {
                pos = (pos - v) % 100;
            }
            _ => unreachable!(),
        }
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &[(&str, i32)]) -> i32 {
    let mut count = 0;
    let mut pos = 50;
    for (d, v) in input {
        match *d {
            "R" => {
                for _ in 0..*v {
                    pos += 1;
                    if pos == 100 {
                        count += 1;
                        pos = 0;
                    }
                }
            }
            "L" => {
                for _ in 0..*v {
                    pos -= 1;
                    if pos == 0 {
                        count += 1;
                    }
                    if pos == -1 {
                        pos = 99;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    count
}

fn part2_enhanced(input: &[(&str, i32)]) -> i32 {
    let mut count = 0;
    let mut pos = 50;
    for (d, v) in input {
        match *d {
            "R" => {
                pos += v % 100;
            }
            "L" => {
                pos -= v % 100;
            }
            _ => unreachable!(),
        }
        count += v / 100;
        if pos <= 0 && pos + v % 100 > 0 || pos >= 100 {
            count += 1;
        }
        println!("{pos}");
        pos = pos.rem_euclid(100);
    }
    count
}
