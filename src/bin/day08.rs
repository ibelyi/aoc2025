use std::collections::HashSet;

fn distance(a: &[u64], b: &[u64]) -> f64 {
    ((0..3)
        .map(|i| a[i].abs_diff(b[i]))
        .map(|v| v * v)
        .sum::<u64>() as f64)
        .sqrt()
}

fn circuit(res: &[HashSet<usize>], thebox: usize) -> Option<usize> {
    res.iter().position(|c| c.contains(&thebox))
}

fn main() {
    let (input, mut wires_left) = (include_str!("../../data/day08/input"), 1000);
    let boxes: Vec<Vec<u64>> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();
    let mut dists: Vec<(f64, (usize, usize))> = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            boxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| (distance(a, b), (i, j)))
        })
        .collect();
    dists.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut res = Vec::new();
    for (_, (i, j)) in dists {
        if let Some(circ_i) = circuit(&res, i) {
            if let Some(circ_j) = circuit(&res, j) {
                if circ_j != circ_i {
                    let (circ, other) = if circ_i < circ_j {
                        (circ_i, res.remove(circ_j))
                    } else {
                        (circ_j, res.remove(circ_i))
                    };
                    res[circ].extend(other);
                }
            } else {
                res[circ_i].insert(j);
            }
        } else if let Some(circ_j) = circuit(&res, j) {
            res[circ_j].insert(i);
        } else {
            res.push(HashSet::from([i, j]));
        }
        wires_left -= 1;
        if wires_left == 0 {
            res.sort_by_key(|a| a.len());
            println!(
                "Result1: {}",
                res.iter().rev().map(|h| h.len()).take(3).product::<usize>()
            );
        }
        if res.len() == 1 && res[0].len() == boxes.len() {
            println!("Result2: {}", boxes[i][0] * boxes[j][0]);
            break;
        }
    }
}
