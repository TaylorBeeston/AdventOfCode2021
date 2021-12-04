mod input;

use crate::input::get_input;

fn main() {
    let input = get_input();

    let (horizontal, depth, _) = calculate_pos(input);

    println!("pos: {}, {}", horizontal, depth);
    println!("answer: {}", horizontal * depth);
}

fn calculate_pos(input: Vec<(&str, usize)>) -> (usize, usize, usize) {
    input.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), (cmd, distance)| match *cmd {
            "forward" => (horizontal + distance, depth + (distance * aim), aim),
            "up" => (horizontal, depth, aim - distance),
            "down" => (horizontal, depth, aim + distance),
            _ => (horizontal, depth, aim),
        },
    )
}
