use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn dist_sq(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct PosPair {
    p1: usize,
    p2: usize,
    dist: i64,
}

impl PartialOrd for PosPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PosPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

fn main() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    let input = include_str!("../../input/d8.txt");

    let input = input
        .lines()
        .map(|line| {
            let mut it = line.splitn(3, ",");
            Pos {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
                z: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("p1: {p1}, p2: {p2}");
}

fn part1(input: &[Pos]) -> i64 {
    let mut pair_heap = BinaryHeap::new();
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if i == j {
                continue;
            }
            pair_heap.push(PosPair {
                p1: i,
                p2: j,
                dist: input[i].dist_sq(&input[j]),
            });
        }
    }

    let mut group_map: Vec<Option<i64>> = vec![None; input.len()];
    let mut groups: Vec<HashSet<usize>> = vec![];
    let mut group_counter = 0;
    for _ in 0..1000 {
        let Some(PosPair { p1, p2, .. }) = pair_heap.pop() else {
            break;
        };
        match (group_map[p1], group_map[p2]) {
            (Some(g1), Some(g2)) => {
                // println!("{p1},{p2}: {g1},{g2} (some, some)");
                if g1 == g2 {
                } else {
                    for c in &groups[g2 as usize] {
                        group_map[*c] = Some(g1);
                    }
                    groups[g1 as usize] = groups[g1 as usize]
                        .union(&groups[g2 as usize])
                        .copied()
                        .collect();
                    groups[g2 as usize].clear();
                }
            }
            (Some(g), None) | (None, Some(g)) => {
                // println!("{p1},{p2}: {g} (some, none)");
                group_map[p1] = Some(g);
                group_map[p2] = Some(g);
                groups[g as usize].insert(p1);
                groups[g as usize].insert(p2);
            }
            (None, None) => {
                // println!("{p1},{p2}: {group_counter} (none, none)");
                group_map[p1] = Some(group_counter);
                group_map[p2] = Some(group_counter);
                groups.push(HashSet::new());
                groups[group_counter as usize].insert(p1);
                groups[group_counter as usize].insert(p2);
                group_counter += 1;
            }
        }
    }

    let (less, pivot, _) = groups.select_nth_unstable_by_key(2, |a| -(a.len() as i64));
    let res = less.iter().fold(pivot.len(), |acc, v| acc * v.len());

    res as i64
}

fn part2(input: &[Pos]) -> i64 {
    let mut pair_heap = BinaryHeap::new();
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if i == j {
                continue;
            }
            pair_heap.push(PosPair {
                p1: i,
                p2: j,
                dist: input[i].dist_sq(&input[j]),
            });
        }
    }

    let mut group_map: Vec<Option<i64>> = vec![None; input.len()];
    let mut groups: Vec<HashSet<usize>> = vec![];
    let mut group_counter = 0;
    for _ in 0..pair_heap.len() {
        let PosPair { p1, p2, .. } = pair_heap.pop().unwrap();
        match (group_map[p1], group_map[p2]) {
            (Some(g1), Some(g2)) => {
                let (g1, g2) = (g1.min(g2), g1.max(g2));
                // println!("{p1},{p2}: {g1},{g2} (some, some)");
                if g1 == g2 {
                } else {
                    for c in &groups[g2 as usize] {
                        group_map[*c] = Some(g1);
                    }
                    groups[g1 as usize] = groups[g1 as usize]
                        .union(&groups[g2 as usize])
                        .copied()
                        .collect();
                    groups[g2 as usize].clear();
                }
            }
            (Some(g), None) | (None, Some(g)) => {
                // println!("{p1},{p2}: {g} (some, none)");
                group_map[p1] = Some(g);
                group_map[p2] = Some(g);
                groups[g as usize].insert(p1);
                groups[g as usize].insert(p2);
            }
            (None, None) => {
                // println!("{p1},{p2}: {group_counter} (none, none)");
                group_map[p1] = Some(group_counter);
                group_map[p2] = Some(group_counter);
                groups.push(HashSet::new());
                groups[group_counter as usize].insert(p1);
                groups[group_counter as usize].insert(p2);
                group_counter += 1;
            }
        }
        if groups[0].len() == input.len() {
            return input[p1].x * input[p2].x;
        }
    }

    unreachable!()
}
