use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../data/day05/input");
    let mut iter = input.split('\n');
    let mut ranges: Vec<(u64, u64)> = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .filter_map(|l| {
            l.split_once('-')
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        })
        .collect();
    let res = iter
        .take_while(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|&id| ranges.iter().any(|(l, r)| id <= *r && id >= *l))
        .count();
    println!("Result1: {res}");

    ranges.sort_by(|a, b| {
        let res = a.0.cmp(&b.0);
        if res == Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            res
        }
    });
    let mut sum = 0;
    let mut right = 0;
    for (l, r) in ranges {
        if l > right {
            sum += r - l + 1;
        } else if l == right {
            sum += r - l;
        } else if r > right {
            sum += r - right;
        }
        right = r;
    }
    println!("Result2: {sum}");
}
