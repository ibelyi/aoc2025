fn area(a: &(u64, u64), b: &(u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn contains_edges(a: &(u64, u64), b: &(u64, u64), others: &[(u64, u64)]) -> bool {
    let (min_x, max_x) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
    let (min_y, max_y) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };

    (0..others.len()).any(|i| {
        let e1 = &others[i];
        let e2 = &others[(i + 1) % others.len()];
        if e1.0 == e2.0 {
            if e1.0 <= min_x || e1.0 >= max_x {
                false
            } else if e1.1 < e2.1 {
                e1.1 < max_y && e2.1 > min_y
            } else {
                e2.1 < max_y && e1.1 > min_y
            }
        } else if e1.1 <= min_y || e1.1 >= max_y {
            false
        } else if e1.0 < e2.0 {
            e1.0 < max_x && e2.0 > min_x
        } else {
            e2.0 < max_x && e1.0 > min_x
        }
    })
}

#[derive(Debug)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

fn outside(a_turn: (&Dir, &Dir), a: &(u64, u64), b: &(u64, u64)) -> bool {
    // Assumes inside is on the right of the loop direction
    match a_turn {
        (Dir::Right, Dir::Up) => a.0 > b.0 && a.1 > b.1, // top-left is outside
        (Dir::Down, Dir::Left) => a.0 < b.0 || a.1 < b.1, // top-left is inside
        (Dir::Down, Dir::Right) => a.0 < b.0 && a.1 > b.1, // top-right is outside
        (Dir::Left, Dir::Up) => a.0 > b.0 || a.1 < b.1,  // top-right is inside
        (Dir::Up, Dir::Left) => a.0 > b.0 && a.1 < b.1,  // bottom-left is outside
        (Dir::Right, Dir::Down) => a.0 < b.0 || a.1 > b.1, // bottom-left is inside
        (Dir::Left, Dir::Down) => a.0 < b.0 && a.1 < b.1, // bottom-right is outside
        (Dir::Up, Dir::Right) => a.0 > b.0 || a.1 > b.1, // bottom-right is inside
        _ => panic!("Invalid turn {a_turn:?}"),
    }
}

fn main() {
    let input = include_str!("../../data/day09/input");
    let input: Vec<(u64, u64)> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (y, x) = l.split_once(',').unwrap();
            (y.trim().parse().unwrap(), x.trim().parse().unwrap())
        })
        .collect();

    let res = input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input.iter().skip(i + 1).map(|b| area(a, b)))
        .max()
        .unwrap();
    println!("Result1: {res}");

    let edges: Vec<Dir> = input
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let next = &input[(i + 1) % input.len()];
            if a.0 == next.0 {
                if a.1 > next.1 { Dir::Up } else { Dir::Down }
            } else if a.0 > next.0 {
                Dir::Left
            } else {
                Dir::Right
            }
        })
        .collect();
    let turns: Vec<(&Dir, &Dir)> = (0..edges.len())
        .map(|i| (&edges[(i + edges.len() - 1) % edges.len()], &edges[i]))
        .collect();

    let mut res = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if outside(turns[i], &input[i], &input[j])
                || outside(turns[j], &input[j], &input[i])
                || contains_edges(&input[i], &input[j], &input)
            {
                continue;
            }
            let val = area(&input[i], &input[j]);
            if res < val {
                res = val;
            }
        }
    }
    println!("Result2: {res}");
}
