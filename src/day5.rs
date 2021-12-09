use atoi::atoi;
use nom::*;

pub fn solve_a() -> i32 {
    let (mut lns, mut overlap) = (vec![0u8; 1000 * 1000], 0);

    include_bytes!("input/day5a.txt")
        .split(|b| *b == b'\n')
        .map(|dg| {
            let ((x, y), (x2, y2)) = line(dg).unwrap().1;
            (x.min(x2), y.min(y2), x.max(x2), y.max(y2))
        })
        .for_each(|(x, y, x2, y2)| {
            let mut mark = |x, y| {
                if lns[(x + y * 1000) as usize] == 1 {
                    overlap += 1;
                }
                lns[(x + y * 1000) as usize] += 1;
            };
            if x == x2 {
                (y..=y2).for_each(|y| mark(x, y));
            } else if y == y2 {
                (x..=x2).for_each(|x| mark(x, y));
            }
        });
    
    return overlap;
}

named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (usize, usize)>, separated_pair!(usize, char!(','), usize));
named!(line<&[u8], ((usize, usize), (usize, usize))>, separated_pair!(coord, tag!(" -> "), coord));