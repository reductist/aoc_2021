use std::collections::BTreeMap;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

pub fn solve_a() -> u32 {
    let (nums, boards) = include_str!("input/day4a.txt").split_once("\n\n").unwrap();

    let mut bds = build_boards(boards);

    let (board, idx, nm) = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .find_map(|n| {
            bds.iter_mut().find_map(|(bd, mp)| {
                bd.get(&n)
                    .map(|i| *mp |= 1 << *i)
                    .filter(|_| (0..5).any(|i| *mp >> i & COL == COL || *mp >> (i * 5) & ROW == ROW))
                    .map(|_| (bd.clone(), *mp, n))
            })
        })
        .unwrap();

    return calc_sum(board, idx, nm);
}

pub fn solve_b() -> u32 {
    let (nums, boards) = include_str!("input/day4a.txt").split_once("\n\n").unwrap();

    let mut bds = build_boards(boards);

    let (board, idx, nm) = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .filter_map(|n| {
            bds
                .drain_filter(|(bd, mp)| {
                    bd.get(&n)
                        .map(|i| *mp |= 1 << *i)
                        .map(|_| (0..5).any(|i| *mp >> i & COL == COL || *mp >> (i * 5) & ROW == ROW))
                        .unwrap_or(false)
                })
                .map(|(bd, mp)| (bd, mp, n))
                .next()
        })
        .last()
        .unwrap();

    return calc_sum(board, idx, nm);
}

fn build_boards(brds: &str) -> Vec<(BTreeMap<u8, usize>, u32)> {
    return brds
        .split("\n\n")
        .map(|bd| {
            (
                bd.split_ascii_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();
}

fn calc_sum(b: BTreeMap<u8, usize>, idx: u32, nm: u8) -> u32 {
    return b
        .into_iter()
        .map(|(n, i)| (idx >> i & 1 ^ 1) * n as u32 * nm as u32)
        .sum::<u32>();
}