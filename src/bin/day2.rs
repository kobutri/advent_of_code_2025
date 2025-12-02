fn main() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let input = include_str!("../../input/d2.txt");

    let input = input
        .trim()
        .split(",")
        .map(|r| {
            let (low, high) = r.split_once("-").unwrap();
            (low.parse::<u64>().unwrap(), high.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("p1: {p1}, p2: {p2}");
}

fn count_digit(mut v: u64) -> u64 {
    if v == 0 {
        return 0;
    }
    let mut digits = 0;
    while v != 0 {
        v /= 10;
        digits += 1;
    }
    digits
}

fn get_high_half(mut v: u64) -> u64 {
    let mut highs = vec![];
    while v != 0 {
        v /= 10;
        highs.push(v);
    }

    highs[(highs.len() - 1) / 2]
}

fn repeat_digits(v: u64) -> u64 {
    let d = count_digit(v);
    v * 10u64.pow(d as u32) + v
}

fn part1(input: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    input.iter().for_each(|(low, high)| {
        let mut current_digits = get_high_half(*low);
        let mut current = repeat_digits(current_digits);
        while current <= *high {
            if current >= *low {
                sum += current;
            }
            current_digits += 1;
            current = repeat_digits(current_digits);
        }
    });

    sum
}

fn is_invalid(v: u64) -> bool {
    let d = count_digit(v);
    'outer: for i in 1..d {
        if !d.is_multiple_of(i) {
            continue;
        }
        let m = 10u64.pow(i as u32);
        let t = v % m;
        let mut c = v;
        while c != 0 {
            if c % m != t {
                continue 'outer;
            }
            c /= m;
        }
        return true;
    }
    false
}

fn part2(input: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    input.iter().for_each(|(low, high)| {
        for i in *low..=*high {
            if is_invalid(i) {
                sum += i;
            }
        }
    });

    sum
}
