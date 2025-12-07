#![feature(iter_intersperse)]

use std::collections::HashSet;
use std::fmt::Display;

fn main() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    let input = include_str!("../../input/d7.txt");

    let input: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let p1 = part1(input.clone());
    let p2 = part2(input);
    println!("p1: {p1}, p2: {p2}");
}

fn part1(mut input: Vec<Vec<char>>) -> u64 {
    let mut split_count = 0;
    for i in 1..input.len() {
        for j in 0..input[i].len() {
            if input[i - 1][j] == '|' || input[i - 1][j] == 'S' {
                if input[i][j] == '.' {
                    input[i][j] = '|'
                } else if input[i][j] == '^' {
                    split_count += 1;
                    if j > 0 {
                        input[i][j - 1] = '|';
                    }
                    if j < input[i].len() - 1 {
                        input[i][j + 1] = '|';
                    }
                }
            }
        }
    }

    split_count
}

fn print_board(input: &Vec<Vec<impl Display>>) {
    for line in input {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

enum Board {
    Particle(u64),
    Manifold(char),
}

fn part2(input: Vec<Vec<char>>) -> u64 {
    let mut input: Vec<Vec<Board>> = input
        .into_iter()
        .map(|line| line.into_iter().map(Board::Manifold).collect())
        .collect();
    for i in 1..input.len() {
        for j in 0..input[i].len() {
            match input[i - 1][j] {
                Board::Particle(p) => {
                    match input[i][j] {
                        Board::Particle(p2) => {
                            input[i][j] = Board::Particle(p + p2);
                        }
                        Board::Manifold('.') => {
                            input[i][j] = Board::Particle(p);
                        }
                        Board::Manifold('^') => {
                            if j > 0 {
                                input[i][j - 1] = match input[i][j - 1] {
                                    Board::Particle(p2) => Board::Particle(p + p2),
                                    Board::Manifold(_) => Board::Particle(p),
                                }
                            }
                            if j + 1 < input[i].len() {
                                input[i][j + 1] = match input[i][j + 1] {
                                    Board::Particle(p2) => Board::Particle(p + p2),
                                    Board::Manifold(_) => Board::Particle(p),
                                }
                            }
                        }
                        _ => unreachable!(),
                    };
                }
                Board::Manifold('S') => input[i][j] = Board::Particle(1),
                Board::Manifold(_) => {}
            }
        }
    }

    // print_board(
    //     &input
    //         .iter()
    //         .map(|line| {
    //             line.iter()
    //                 .map(|b| match b {
    //                     Board::Particle(p) => format!("{:02}#", p),
    //                     Board::Manifold(m) => format!(" {} ", m),
    //                 })
    //                 .collect()
    //         })
    //         .collect(),
    // );

    input
        .last()
        .unwrap()
        .iter()
        .map(|b| match b {
            Board::Particle(p) => *p,
            Board::Manifold(_) => 0,
        })
        .sum()
}
