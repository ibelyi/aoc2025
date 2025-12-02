fn main() {
    let input = include_str!("../../data/day01/input");
    let mut curr = 50u32;
    let mut count1 = 0;
    let mut count2 = 0;
    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let (d, n) = line.split_at(1);
        let number: u32 = n.parse().unwrap();
        curr = match d {
            "R" => {
                if curr + number >= 100 {
                    count2 += 1 + (number + curr - 100) / 100;
                }
                (curr + number) % 100
            }
            "L" => {
                if number >= curr {
                    if curr > 0 {
                        count2 += 1;
                    }
                    count2 += (number - curr) / 100
                }
                (curr + 100 * number.div_ceil(100) - number) % 100
            }
            _ => panic!("Invalid direction: {d}"),
        };
        if curr == 0 {
            count1 += 1;
        }
    }
    println!("Result1: {count1}");
    println!("Result2: {count2}");
}
