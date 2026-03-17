#[aoc(day3, part1)]
fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| calculate_jolt(line))
        .sum::<isize>()
}

fn calculate_jolt(line: &str) -> isize {
    let numbers = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>();
    let max = numbers.iter().take(numbers.len() - 1).max().unwrap();
    let max2 = numbers
        .iter()
        .skip_while(|n| *n < max)
        .skip(1)
        .max()
        .unwrap();
    let x = max * 10 + max2;
    x
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let size = 12;
            let numbers = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
            calculate_jolt2(&numbers, 0, numbers.len() - size, size)
        })
        .sum::<usize>()
}

fn calculate_jolt2(
    numbers: &Vec<usize>,
    upper: usize,
    possible_count: usize,
    multiplier: usize,
) -> usize {
    let max = numbers
        .iter()
        .skip(upper)
        .take(possible_count)
        .max()
        .unwrap();
    let max = *max;

    if multiplier == 1 {
        max
    } else {
        let new_upper = numbers.iter().skip(upper).take_while(|&&n| n < max).count();
        let new_upper = new_upper + 1 + upper;
        let to_draw = multiplier - 1;
        (max * 10usize.pow((multiplier - 1) as u32))
            + calculate_jolt2(
                numbers,
                new_upper,
                numbers.len() - new_upper - to_draw + 1,
                multiplier - 1,
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"\
        987654321111111
        811111111111119
        234234234234278
        818181911112111
"};

    #[test]
    fn sample_p1() {
        let result = part1(SAMPLE);

        assert_eq!(result, 357);
    }

    #[test]
    fn sample_p2() {
        let result = part2(SAMPLE);

        assert_eq!(result, 3121910778619);
    }
}
