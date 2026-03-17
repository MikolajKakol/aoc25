use std::collections::HashSet;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.count_valid_places()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.count_removals()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn adjacent(&self) -> [Point; 8] {
        let up = self.up();
        let down = self.down();
        [
            up.left(),
            up.right(),
            up,
            down.left(),
            down.right(),
            down,
            self.left(),
            self.right(),
        ]
    }
}

struct Grid {
    paper_pile: HashSet<Point>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut paper_pile: Vec<Point> = Vec::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.char_indices().for_each(|(x, c)| {
                if c == '@' {
                    paper_pile.push(Point::new(x as isize, y as isize));
                }
            })
        });

        Self {
            paper_pile: paper_pile.into_iter().collect(),
        }
    }

    fn count_valid_places(&self) -> usize {
        self.paper_pile
            .iter()
            .map(|p| {
                let adj: HashSet<_> = p.adjacent().into_iter().collect();

                let count = self.paper_pile.intersection(&adj).count();

                count < 4
            })
            .filter(|x| *x)
            .count()
    }

    fn count_removals(&mut self) -> usize {
        let mut removed = 0;

        loop {
            let to_remove: Vec<_> = self
                .paper_pile
                .iter()
                .map(|p| {
                    let adj: HashSet<_> = p.adjacent().into_iter().collect();

                    let count = self.paper_pile.intersection(&adj).count();

                    if count < 4 {
                        Option::Some(p.clone())
                    } else {
                        Option::None
                    }
                })
                .flatten()
                .collect();

            if to_remove.is_empty() {
                break;
            }
            self.paper_pile.retain(|x| !to_remove.contains(&x));
            removed += to_remove.len();
        }

        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"};

    #[test]
    fn sample_p1() {
        let result = part1(SAMPLE);

        assert_eq!(result, 13);
    }

    #[test]
    fn sample_p2() {
        let result = part2(SAMPLE);

        assert_eq!(result, 43);
    }
}
