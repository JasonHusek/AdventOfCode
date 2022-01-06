mod challenge_1;
mod challenge_2;
mod challenge_inputs;

fn main() {
    println!(
        "Challenge 1 Part 1: {}",
        challenge_1::solve_part_1(challenge_inputs::C1_INPUT.to_vec())
    );
    println!(
        "Challenge 1 Part 2: {}",
        challenge_1::solve_part_2(challenge_inputs::C1_INPUT.to_vec())
    );

    println!(
        "Challenge 2 Part 1: {}",
        challenge_2::solve_part_1(challenge_inputs::C2_INPUT.to_vec())
    );

    println!(
        "Challenge 2 Part 2: {}",
        challenge_2::solve_part_2(challenge_inputs::C2_INPUT.to_vec())
    );
}
