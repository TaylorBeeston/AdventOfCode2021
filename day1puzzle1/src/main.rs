mod input;

use crate::input::get_input;

fn main() {
    println!("Answer: {}", count_increasing(get_input()));
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
