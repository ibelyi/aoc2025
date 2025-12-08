use std::collections::HashMap;

fn main() {
    let input = include_str!("../../data/day07/input");
    let mut splits = 0;
    let mut timelines = HashMap::new();
    for line in input.split('\n').filter(|l| !l.is_empty()) {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                timelines.insert(x, 1);
            } else if c == '^'
                && let Some(timeline) = timelines.remove(&x)
            {
                splits += 1;
                *timelines.entry(x + 1).or_default() += timeline;
                *timelines.entry(x - 1).or_default() += timeline;
            }
        }
    }
    println!("Result1: {splits}");
    println!("Result2: {}", timelines.values().sum::<usize>());
}
