fn main() {
    let input = include_str!("../../data/day06/input");
    let mut lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let opers: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();
    let numbers: Vec<Vec<i64>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let sum = opers
        .iter()
        .enumerate()
        .map(|(i, oper)| match *oper {
            "*" => (0..numbers.len()).map(|n| numbers[n][i]).product::<i64>(),
            "+" => (0..numbers.len()).map(|n| numbers[n][i]).sum::<i64>(),
            _ => panic!("Invalid operator: {oper}"),
        })
        .sum::<i64>();
    println!("Result1: {sum}");

    // full matrix of chars
    let input: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    // flip columns with rows, empty lines will be the separators
    let flipped: Vec<String> = (0..lines[0].len())
        .map(|x| (0..lines.len()).map(|y| input[y][x]).collect())
        .collect();
    let mut iter = flipped.iter();
    let sum = opers
        .iter()
        .map(|oper| match *oper {
            "*" => iter
                .by_ref()
                .take_while(|l| !l.trim().is_empty())
                .map(|l| l.trim().parse::<i64>().unwrap())
                .product::<i64>(),
            "+" => iter
                .by_ref()
                .take_while(|l| !l.trim().is_empty())
                .map(|l| l.trim().parse::<i64>().unwrap())
                .sum::<i64>(),
            _ => panic!("Invalid operator {oper}"),
        })
        .sum::<i64>();
    println!("Result2: {sum}");
}
