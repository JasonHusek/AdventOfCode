// https://adventofcode.com/2021/day/2

/*
    Helpful regex for converting input to tuples

    From:
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    Find: ([a-z]+)+
    Replace: ("$1",
    Produces:
        ("forward", 5
        ("down", 5
        ("forward", 8
        ("up", 3
        ("down", 8
        ("forward", 2
    Find:
        (\d)
    Replace:
        $1),
    Produces:
        ("forward", 5),
        ("down", 5),
        ("forward", 8),
        ("up", 3),
        ("down", 8),
        ("forward", 2),
*/

pub fn solve_part_1(input: Vec<(&str, i32)>) -> i32 {
    let mut sub = Submarine::new();
    for cmd in input {
        sub.perform_aimless_move(cmd)
    }
    sub.product()
}

pub fn solve_part_2(input: Vec<(&str, i32)>) -> i32 {
    let mut sub = Submarine::new();
    for cmd in input {
        sub.perform_aimed_move(cmd)
    }
    sub.product()
}

pub struct Submarine {
    horizontal_pos: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn perform_aimless_move(&mut self, command: (&str, i32)) {
        match command.0 {
            "forward" => self.horizontal_pos += command.1,
            "down" => self.depth += command.1,
            "up" => self.depth -= command.1,
            _ => (),
        }
    }

    fn perform_aimed_move(&mut self, command: (&str, i32)) {
        match command.0 {
            "forward" => {
                self.horizontal_pos += command.1;
                self.depth += self.aim * command.1;
            }
            "down" => self.aim += command.1,
            "up" => self.aim -= command.1,
            _ => (),
        }
    }

    fn product(&self) -> i32 {
        self.horizontal_pos * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [(&str, i32); 6] = [
        ("forward", 5),
        ("down", 5),
        ("forward", 8),
        ("up", 3),
        ("down", 8),
        ("forward", 2),
    ];

    #[test]
    fn part_1_example() {
        let expected_output = 150;
        assert_eq!(solve_part_1(EXAMPLE_INPUT.to_vec()), expected_output);
    }

    #[test]
    fn part_2_example() {
        let expected_output = 900;
        assert_eq!(solve_part_2(EXAMPLE_INPUT.to_vec()), expected_output);
    }
}
