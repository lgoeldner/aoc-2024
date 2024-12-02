use std::hint::assert_unchecked;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse(input);
    Some(
        parsed
            .into_iter()
            .filter_map(|it| is_safe(it).then_some(()))
            .count()
            .try_into()
            .unwrap(),
    )
}

fn is_safe_with_dampener(mut iter: impl Iterator<Item = i32>) -> bool {
    let mut last = iter.next().unwrap();
    let mut sign = None;
    let mut still_safe = true;

    for i in iter {
        if !(1..=3).contains(&last.abs_diff(i)) {
            if !still_safe {
                return false;
            } else {
                still_safe = false;
            }
        }

        if sign.is_none() {
            sign = Some((last - i).signum());
            last = i;
            continue;
        }

        if Some((last - i).signum()) != sign {
            if !still_safe {
                return false;
            } else {
                still_safe = false;
            }
        }
        last = i;
    }

    true
}

fn is_safe(mut iter: impl Iterator<Item = i32>) -> bool {
    let mut last = iter.next().unwrap();
    let mut sign = None;

    for i in iter {
        if !(1..=3).contains(&last.abs_diff(i)) {
            return false;
        }

        if sign.is_none() {
            sign = Some((last - i).signum());
            last = i;
            continue;
        }

        if Some((last - i).signum()) != sign {
            return false;
        }
        last = i;
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse(input);
    Some(
        parsed
            .into_iter()
            .filter_map(|it| is_safe_with_dampener(it).then_some(()))
            .count()
            .try_into()
            .unwrap(),
    )
}

fn parse(inp: &str) -> impl Iterator<Item = impl Iterator<Item = i32> + use<'_>> {
    inp.lines()
        .map(|it| it.split(' ').map(|it| unsafe { parse_unsigned(it) }))
}

unsafe fn parse_unsigned(it: &str) -> i32 {
    let bytes = it.as_bytes();
    let mut res = 0;
    for it in bytes {
        res = res * 10 + (*it - b'0');
    }

    debug_assert_eq!(res as i32, it.parse::<i32>().unwrap());
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
