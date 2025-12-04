fn main() {
    let input = include_str!("../../data/day03/input");
    let banks: Vec<Vec<u32>> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|bank| bank.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    for (i, size) in [2, 12].into_iter().enumerate() {
        let mut sum = 0;
        for bank in &banks {
            let mut maxs = vec![0; size];
            let mut index = 0;
            for (m, max) in maxs.iter_mut().enumerate() {
                for (i, &val) in bank
                    .iter()
                    .take(bank.len() - size + m + 1)
                    .enumerate()
                    .skip(index)
                {
                    if val > *max {
                        *max = val;
                        index = i + 1;
                    }
                }
            }
            sum += maxs
                .into_iter()
                .fold(0u64, |res, val| res * 10 + val as u64);
        }
        println!("Result{}: {sum}", i + 1);
    }
}
