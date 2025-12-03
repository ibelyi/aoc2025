use std::collections::HashSet;

fn collect_invalid(pattern: &str, start: &str, end: &str) -> HashSet<u64> {
    let mult: u64 = pattern.parse().unwrap();
    let start: u64 = start.parse().unwrap();
    let end: u64 = end.parse().unwrap();
    let mut sum = HashSet::new();
    let mut current = mult * (start / mult);
    while current <= end {
        if current >= start && current.to_string().len() == pattern.len() {
            sum.insert(current);
        }
        current += mult;
    }
    sum
}

fn main() {
    let input = include_str!("../../data/day02/input");
    let mut invalid1 = HashSet::new();
    let mut invalid2 = HashSet::new();
    for range in input.split(',').map(str::trim).filter(|l| !l.is_empty()) {
        let (start, end) = range.split_once('-').unwrap();

        for len in (start.len()..=end.len()).filter(|l| l % 2 == 0) {
            invalid1.extend(collect_invalid(
                &format!("{}1", "0".repeat(len / 2 - 1)).repeat(2),
                start,
                end,
            ));
        }

        let mut pattern = "1".to_string();
        while pattern.len() <= end.len() / 2 {
            for len in (start.len()..=end.len()).filter(|l| pattern.len() <= l / 2) {
                invalid2.extend(collect_invalid(
                    &pattern.repeat(len / pattern.len()),
                    start,
                    end,
                ));
            }
            pattern = format!("0{pattern}");
        }
    }
    println!("Result1: {}", invalid1.iter().sum::<u64>());
    println!("Result2: {}", invalid2.iter().sum::<u64>());
}
