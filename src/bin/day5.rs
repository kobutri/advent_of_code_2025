use std::ops::RangeInclusive;

fn main() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let input = include_str!("../../input/d5.txt");

    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (low, high) = line.trim().split_once("-").unwrap();
            low.parse::<i64>().unwrap()..=high.parse::<i64>().unwrap()
        })
        .collect::<Vec<_>>();

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let p1 = part1(&ranges, &ingredients);
    let p2 = part2(&ranges);
    println!("p1: {p1}, p2: {p2}");
}

fn part1(ranges: &[RangeInclusive<i64>], ingredients: &[i64]) -> i64 {
    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count() as i64
}

fn range_diff(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    assert!(!r1.contains(r2.start()) || !r1.contains(r2.end()) || r1 == r2);

    if r2.contains(r1.start()) && r2.contains(r1.end()) {
        1..=0
    } else if r1.contains(r2.start()) {
        *r1.start()..=*r2.start() - 1
    } else if r1.contains(r2.end()) {
        *r2.end() + 1..=*r1.end()
    } else {
        r1.clone()
    }
}

fn part2(ranges: &[RangeInclusive<i64>]) -> i64 {
    let mut count = 0;

    let mut ranges = ranges.to_vec();
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..ranges.len() {
            for j in i + 1..ranges.len() {
                if ranges[i] != ranges[j]
                    && ranges[i].contains(ranges[j].start())
                    && ranges[i].contains(ranges[j].end())
                {
                    let temp = ranges[j].clone();
                    ranges[j] = ranges[i].clone();
                    ranges[i] = temp;
                    changed = true;
                }
            }
        }
    }

    for i in 0..ranges.len() {
        let mut range = ranges[i].clone();
        // println!("{:?}", range);
        for r2 in &ranges[i + 1..] {
            // println!("{:?} \\ {:?} = {:?}", range, r2, range_diff(&range, r2));
            range = range_diff(&range, r2);
            if range.end() - range.start() + 1 <= 0 {
                break;
            }
        }
        // println!("-----------");
        count += (range.end() - range.start() + 1).max(0)
    }

    count
}
