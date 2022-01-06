// https://adventofcode.com/2021/day/1

//Counts each instance where input[idx] > input[idx-1]
pub fn solve_part_1(input: Vec<i32>) -> i32 {
    input.iter().enumerate().skip(1).fold(
        0,
        |acc, (i, x)| if x > &input[i - 1] { acc + 1 } else { acc },
    )
}

//Creates a vec containing sums of 3 element sliding window of input
pub fn solve_part_2(input: Vec<i32>) -> i32 {
    let chunks = input
        .iter()
        .enumerate()
        .take_while(|(i, _)| i < &(input.len() - 2))
        .map(|(i, v)| v + input[i + 1] + input[i + 2])
        .collect();
    solve_part_1(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_1_example() {
        let expected_output = 7;
        assert_eq!(solve_part_1(EXAMPLE_INPUT.to_vec()), expected_output);
    }

    #[test]
    fn part_2_example() {
        let expected_output = 5;
        assert_eq!(solve_part_2(EXAMPLE_INPUT.to_vec()), expected_output);
    }
}
