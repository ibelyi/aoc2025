use std::collections::HashSet;

fn accessible(input: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    input
        .iter()
        .filter(|(y, x)| {
            (-1..=1)
                .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
                .filter(|&(dy, dx)| (dy != 0 || dx != 0) && input.contains(&(y + dy, x + dx)))
                .count()
                < 4
        })
        .cloned()
        .collect()
}

fn main() {
    let input = include_str!("../../data/day04/input");
    let mut input: HashSet<(i32, i32)> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(move |(x, _)| (y as i32, x as i32))
        })
        .collect();

    println!("Result1: {}", accessible(&input).len());

    let start = input.len();
    loop {
        let removals = accessible(&input);
        if removals.is_empty() {
            break;
        }
        input.retain(|p| !removals.contains(p));
    }
    println!("Result2: {}", start - input.len());
}
