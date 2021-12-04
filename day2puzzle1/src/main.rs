mod input;

use crate::input::get_input;

fn main() {
    let input = get_input();

    let (h, d) = calculate_pos(input);

    println!("pos: {}, {}", h, d);
    println!("answer: {}", h * d);
}

fn calculate_pos(input: Vec<(&str, usize)>) -> (usize, usize) {
    input
        .iter()
        .fold((0, 0), |(horizontal, depth), (cmd, distance)| match *cmd {
            "forward" => (horizontal + distance, depth),
            "up" => (horizontal, depth - distance),
            "down" => (horizontal, depth + distance),
            _ => (horizontal, depth),
        })
}
