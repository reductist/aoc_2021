pub fn solve_a() -> i32 {
    let (fwd, dwn) = include_str!("input/day2a.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(fwd, dwn), (a, b)| {
            match (a, b.parse::<i32>().unwrap()) {
                ("forward", b) => (fwd + b, dwn),
                ("down", b) => (fwd, dwn + b),
                ("up", b) => (fwd, dwn - b),
                _ => unreachable!(),
            }
        });
    return fwd * dwn;
}

pub fn solve_b() -> i32 {
    let (fwd, dwn, _) = include_str!("input/day2b.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(fwd, dwn, aim), (a, b)| {
            match (a, b.parse::<i32>().unwrap()) {
                ("forward", b) => (fwd + b, dwn + aim * b, aim),
                ("down", b) => (fwd, dwn, aim + b),
                ("up", b) => (fwd, dwn, aim - b),
                _ => unreachable!(),
            }
        });
    return fwd * dwn;
}