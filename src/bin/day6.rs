fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut vec = Vec::from_iter(std::iter::repeat_with(|| vec![]).take(input[0].len()));

    for line in input {
        for (i, v) in line.into_iter().enumerate() {
            vec[i].push(v);
        }
    }

    vec
}

fn main() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    let input = include_str!("../../input/d6.txt");
    let input = input.lines().collect::<Vec<_>>();
    let input = {
        let mut positions = vec![];
        input.last().unwrap().char_indices().for_each(|(i, c)| {
            if !c.is_whitespace() {
                positions.push(i);
            }
        });
        positions.push(input.last().unwrap().len());
        input
            .into_iter()
            .map(|line| {
                positions
                    .windows(2)
                    .map(|p| &line[p[0]..p[1]])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
    let input = transpose(input);

    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("p1: {p1}, p2: {p2}");
}

fn part1(input: &[Vec<&str>]) -> i64 {
    let input = input
        .iter()
        .map(|col| {
            let mut iter = col.iter().rev();
            let op = *iter.next().unwrap();
            let op = op.trim();
            let values = iter
                .map(|v| v.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (op, values)
        })
        .collect::<Vec<_>>();
    input
        .into_iter()
        .map(|(op, vals)| {
            vals.into_iter()
                .reduce(|acc, v| match op {
                    "+" => acc + v,
                    "*" => acc * v,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum()
}

fn part2(input: &[Vec<&str>]) -> i64 {
    let input = input
        .iter()
        .map(|col| {
            let mut iter = col.iter().rev();
            let op = *iter.next().unwrap();
            let op = op.trim();
            let iter = iter.rev();
            let values = iter
                .map(|v| v.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let values = transpose(values);
            let values = values
                .into_iter()
                .map(|col| {
                    col.into_iter().fold(0i64, |acc, v| {
                        if let Some(v) = v.to_digit(10) {
                            acc * 10 + v as i64
                        } else {
                            acc
                        }
                    })
                })
                .filter(|v| *v != 0)
                .collect::<Vec<_>>();
            (op, values)
        })
        .collect::<Vec<_>>();
    input
        .into_iter()
        .map(|(op, vals)| {
            vals.into_iter()
                .reduce(|acc, v| match op {
                    "+" => acc + v,
                    "*" => acc * v,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum()
}
