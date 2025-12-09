fn main() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let input = include_str!("../../input/d9.txt");

    let input = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();

    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("p1: {p1}, p2: {p2}");
}

fn part1(input: &[(i64, i64)]) -> i64 {
    let mut max = 0;
    for (a1, b1) in input {
        for (a2, b2) in input {
            max = max.max(((a2 - a1).abs() + 1) * ((b2 - b1).abs() + 1));
        }
    }
    max
}

fn range_overlap(x1: (i64, i64), x2: (i64, i64)) -> bool {
    let x1min = x1.0.min(x1.1);
    let x1max = x1.0.max(x1.1);
    let x2min = x2.0.min(x2.1);
    let x2max = x2.0.max(x2.1);
    let x1range = x1min..=x1max;
    let x2range = x2min..=x2max;

    x1range.contains(&x2min)
        || x1range.contains(&x2max)
        || x2range.contains(&x1min)
        || x2range.contains(&x1min)
}

fn range_point_contain(range: (i64, i64), p: i64) -> bool {
    let rmin = range.0.min(range.1);
    let rmax = range.0.max(range.1);

    (rmin..=rmax).contains(&p)
}

fn intersect_line_line(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64), p4: (i64, i64)) -> bool {
    assert!(p1.0 == p2.0 || p1.1 == p2.1);
    assert!(p3.0 == p4.0 || p3.1 == p4.1);

    if p1.0 == p2.0 && p3.0 == p4.0 {
        return p1.0 == p3.0 && range_overlap((p1.1, p2.1), (p3.1, p4.1));
    }
    if p1.1 == p2.1 && p3.1 == p4.1 {
        return p1.1 == p3.1 && range_overlap((p1.0, p2.0), (p3.0, p4.0));
    }

    if p1.0 == p2.0 {
        assert!(p3.1 == p4.1);
        return range_point_contain((p3.0, p4.0), p1.0) && range_point_contain((p1.1, p2.1), p3.1);
    }

    if p3.0 == p4.0 {
        assert!(p1.1 == p2.1);
        return range_point_contain((p3.1, p4.1), p1.1) && range_point_contain((p1.0, p2.0), p3.0);
    }

    unreachable!()
}

fn intersect_square_line(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64), p4: (i64, i64)) -> bool {
    intersect_line_line(p1, (p1.0, p2.1), p3, p4)
        || intersect_line_line(p1, (p2.0, p1.1), p3, p4)
        || intersect_line_line(p2, (p2.0, p1.1), p3, p4)
        || intersect_line_line(p2, (p1.0, p2.1), p3, p4)
}

fn part2(input: &[(i64, i64)]) -> i64 {
    let mut max = 0;
    for (a1, b1) in input {
        for (a2, b2) in input {
            let size = ((a2 - a1).abs() + 1) * ((b2 - b1).abs() + 1);
            if size > max {
                let mut inside = false;
                let amin = *a1.min(a2);
                let amax = *a1.max(a2);
                let bmin = *b1.min(b2);
                let bmax = *b1.max(b2);
                for i in 0..input.len() {
                    let p1 = input[i];
                    let p2 = input[(i + 1) % input.len()];
                    // point is in square
                    if (amin + 1..=amax - 1).contains(&p1.0)
                        && (bmin + 1..=bmax - 1).contains(&p1.1)
                        || (amin + 1..=amax - 1).contains(&p2.0)
                            && (bmin + 1..=bmax - 1).contains(&p2.1)
                        || intersect_square_line((amin + 1, bmin + 1), (amax - 1, bmax - 1), p1, p2)
                    {
                        inside = true;
                        break;
                    };
                }
                if !inside {
                    max = size;
                }
            }
        }
    }
    max
}
