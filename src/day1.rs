pub fn solve_a() -> usize {
    let n_inc = include_str!("input/day1a.txt")
        .lines()
        .map(|ln| ln.parse().unwrap())
        .collect::<Vec<u16>>()
        .array_windows()
        .filter(|[a, b]| a < b)
        .count();

    return n_inc;
}

pub fn solve_b() -> usize {
    let inc_sums = include_str!("input/day1b.txt")
        .lines()
        .map(|ln| ln.parse().unwrap())
        .collect::<Vec<u16>>()
        .array_windows()
        .filter(|[a, _, _, d]| a < d)
        .count();

    return inc_sums;
}