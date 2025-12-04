fn main() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let input = include_str!("../../input/d3.txt");

    let input = input
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("p1: {p1}, p2: {p2}");
}

fn part1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|bank| {
            let mut h = 0;
            let mut l = 0;
            for (i, j) in bank.iter().enumerate() {
                let j = *j;

                if j > h && i != bank.len() - 1 {
                    h = j;
                    l = 0;
                } else if j > l {
                    l = j;
                }
            }

            // println!("{h}{l}");
            10 * h + l
        })
        .sum()
}

fn foo(bank: &[u64], end_offset: usize) -> (usize, u64) {
    let mut index = 0;
    let mut high = 0;
    for (i, j) in bank[0..bank.len() - end_offset].iter().enumerate() {
        if *j > high {
            high = *j;
            index = i;
        }
    }
    (index + 1, high)
}

fn part2(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|bank| {
            let mut v = 0;
            let mut start_offset = 0;
            for end_offset in (0..12).rev() {
                let (so, max) = foo(&bank[start_offset..], end_offset);
                v += max * 10u64.pow(end_offset as u32);
                start_offset += so;
            }

            v
        })
        .sum()
}
