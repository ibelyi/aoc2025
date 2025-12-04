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
            let mut bank_max = 0;
            let mut index = 0;
            for m in 0..size {
                let mut max = 0;
                for (i, &val) in bank
                    .iter()
                    .take(bank.len() - size + m + 1)
                    .enumerate()
                    .skip(index)
                {
                    if val > max {
                        max = val;
                        index = i + 1;
                    }
                }
                bank_max = bank_max * 10 + max as u64;
            }
            sum += bank_max;
        }
        println!("Result{}: {sum}", i + 1);
    }
}
