mod input;

use crate::input::get_input;

fn main() {
    let sums = window_sums(get_input());
    println!("Answer: {}", count_increasing(sums));
}

fn window_sums(input: Vec<usize>) -> Vec<usize> {
    input.windows(3).map(|window| window.iter().sum()).collect()
}

fn count_increasing(input: Vec<usize>) -> usize {
    input.windows(2).fold(0, |acc, current| {
        let prev = current[0];
        let next = current[1];

        if next > prev {
            acc + 1
        } else {
            acc
        }
    })
}
