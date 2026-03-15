static LOCK_LEN: isize = 100;
struct Safe {
    pin_position: isize,
    zero_hits: isize,
}

impl Safe {
    fn new(pin_position: isize) -> Self {
        Safe {
            pin_position,
            zero_hits: 0,
        }
    }

    fn part1(&mut self, source: &str) -> isize {
        source
            .lines()
            .map(|line| Move::new(line))
            .for_each(|m| self.spin(&m));
        self.zero_hits
    }

    fn part2(&mut self, source: &str) -> isize {
        source
            .lines()
            .map(|line| Move::new(line))
            .for_each(|m| self.spin2(&m));
        self.zero_hits
    }

    fn spin(&mut self, m: &Move) {
        self.pin_position += m.get_direction();
        self.pin_position %= LOCK_LEN;
        if self.pin_position < 0 {
            self.pin_position += LOCK_LEN;
        }

        if self.pin_position == 0 {
            self.zero_hits += 1;
        }
    }

    fn spin2(&mut self, m: &Move) {
        let dir = m.get_direction();
        let full_rotations = (dir / LOCK_LEN).abs();
        let relevant_rotation = dir % LOCK_LEN;
        if full_rotations > 0 {
            dbg!(full_rotations);
            self.zero_hits += full_rotations;
        }

        let was_zero = self.pin_position == 0;
        self.pin_position += relevant_rotation;
        if self.pin_position >= LOCK_LEN {
            if !was_zero {
                self.zero_hits += 1;
            }
            self.pin_position -= LOCK_LEN;
        } else if self.pin_position < 0 {
            if !was_zero {
                self.zero_hits += 1;
            }
            self.pin_position += LOCK_LEN;
        } else if self.pin_position == 0 {
            self.zero_hits += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    distance: usize,
}

impl Move {
    fn new(line: &str) -> Self {
        let (direction, distance) = line.split_at(1);
        let distance = distance.trim().parse().unwrap();
        Move {
            direction: match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            },
            distance,
        }
    }

    fn get_direction(&self) -> isize {
        -match self.direction {
            Direction::Left => self.distance.cast_signed(),
            Direction::Right => -self.distance.cast_signed(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    const REAL_DATA: &str = include_str!("input01.txt");

    #[test]
    fn sample_p1() {
        let mut safe = Safe::new(50);
        let result = safe.part1(SAMPLE);

        assert_eq!(result, 3);
    }

    #[test]
    fn real_p1() {
        let mut safe = Safe::new(50);
        let result = safe.part1(REAL_DATA);

        assert_eq!(result, 1018);
    }
    #[test]
    fn sample_p2() {
        let mut safe = Safe::new(50);
        let result = safe.part2(SAMPLE);

        assert_eq!(result, 6);
    }

    #[test]
    fn real_p2() {
        let mut safe = Safe::new(50);
        let result = safe.part2(REAL_DATA);

        assert_eq!(result, 5815);
    }
}

fn main() {}