use std::collections::HashMap;

fn count(node: &str, all: &HashMap<&str, Vec<&str>>, hash: &mut HashMap<String, usize>) -> usize {
    if node == "out" {
        return 1;
    }
    if let Some(res) = hash.get(node) {
        return *res;
    }
    let Some(choices) = all.get(node) else {
        panic!("Unknown node {node}");
    };
    let res = choices.iter().map(|node| count(node, all, hash)).sum();
    hash.insert(node.to_string(), res);
    res
}

fn count2(
    node: &str,
    all: &HashMap<&str, Vec<&str>>,
    hash: &mut HashMap<String, (usize, usize, usize, usize)>,
) -> (usize, usize, usize, usize) {
    if node == "out" {
        return (1, 0, 0, 0);
    }
    if let Some(res) = hash.get(node) {
        return *res;
    }
    let Some(choices) = all.get(node) else {
        panic!("Unknown node {node}");
    };
    let res = choices
        .iter()
        .map(|child| count2(child, all, hash))
        .map(|(none, fft, dac, both)| match node {
            "fft" => (0, fft + none, 0, dac + both),
            "dac" => (0, 0, dac + none, fft + both),
            _ => (none, fft, dac, both),
        })
        .fold((0, 0, 0, 0), |acc, res| {
            (acc.0 + res.0, acc.1 + res.1, acc.2 + res.2, acc.3 + res.3)
        });
    hash.insert(node.to_string(), res);
    res
}

fn main() {
    let input = include_str!("../../data/day11/input");
    let input: HashMap<&str, Vec<&str>> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap())
        .map(|(d, outs)| (d, outs.split_whitespace().collect()))
        .collect();
    println!("Result1: {}", count("you", &input, &mut HashMap::new()));

    let input: HashMap<&str, Vec<&str>> = include_str!("../../data/day11/input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap())
        .map(|(d, outs)| (d, outs.split_whitespace().collect()))
        .collect();
    println!("Result1: {}", count2("svr", &input, &mut HashMap::new()).3);
}
