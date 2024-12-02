#![warn(clippy::all, clippy::nursery)]

use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut lhs, mut rhs) = parse(input);
    lhs.sort_by_key(|it| it.0);
    rhs.sort_by_key(|it| it.0);

    Some(
        lhs.iter()
            .zip(rhs)
            .map(|((a, _), (b, _))| a.abs_diff(b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (lhs, rhs) = parse_2(input);
    Some(lhs.iter().map(|it| it * rhs.get(it).unwrap_or(&0)).sum())
}

fn parse_2(inp: &str) -> (Vec<u32>, HashMap<u32, u32>) {
    let mut freq = HashMap::new();
    let mut lhs = Vec::new();
    for (l, r) in inp.lines().map(parse_line) {
        lhs.push(l);
        *freq.entry(r).or_insert(0) += 1;
    }

    (lhs, freq)
}

type ListWithIndex = Vec<(u32, usize)>;
fn parse(inp: &str) -> (ListWithIndex, ListWithIndex) {
    inp.lines()
        .map(parse_line)
        .enumerate()
        .map(|(idx, (a, b))| ((a, idx), (b, idx)))
        .unzip()
}

fn parse_line(line: &str) -> (u32, u32) {
    let (a, b) = line.split_once("   ").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
