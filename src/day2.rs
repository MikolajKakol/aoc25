use std::ops::Range;

#[aoc(day2, part1)]
fn part1(input: &str) -> isize {
    let ranges = parse_ranges(input);
    ranges.into_iter().map(move |r| invalid_count(r)).sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> isize {
    let ranges = parse_ranges(input);
    ranges.into_iter().map(move |r| invalid_count2(r)).sum()
}

fn invalid_count(range: Range<isize>) -> isize {
    let mut count = 0;
    range.for_each(|n| {
        let s = n.to_string();
        if s.len() % 2 == 0 {
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                count += n;
            }
        }
    });
    count
}

fn invalid_count2(range: Range<isize>) -> isize {
    let mut count = 0;
    range.for_each(|n| {
        let s = n.to_string();
        let s = s.into_bytes();
        for split in 1..s.len() {
            if s.len() % split == 0 {
                let mut chunks = s.chunks(split);
                let same = match chunks.next() {
                    Some(first) => chunks.all(|chunk| chunk == first),
                    None => false,
                };
                if same {
                    count += n;
                    break
                }
            }
        }
    });
    count
}

fn parse_ranges(input: &str) -> Vec<Range<isize>> {
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse::<isize>().unwrap() + 1,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"};

    #[test]
    fn sample_p1() {
        let result = part1(SAMPLE);

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn sample_p2() {
        let result = part2(SAMPLE);

        assert_eq!(result, 4174379265);
    }
}
