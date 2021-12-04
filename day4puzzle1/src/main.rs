mod bingo;
mod input;

use crate::bingo::Board;
use crate::input::{get_boards, get_input};

fn main() {
    let mut boards: Vec<Board> = get_boards().iter().map(|input| Board::new(input)).collect();
    let raw_input = get_input();
    let mut input = raw_input.iter();

    let mut number = input.next();

    while number.is_some() && !boards.iter().any(|board| board.check_won()) {
        println!("Checking {}", number.unwrap());

        boards = boards
            .iter()
            .map(|board| board.check_num(*number.unwrap()))
            .collect();

        number = input.next();
    }

    boards.iter().for_each(|board| {
        if board.check_won() {
            println!("We have a winner! Total score is {}", board.unmarked_sum());
            println!("Last number was {}", number.unwrap());
            println!("Answer is {}", number.unwrap() * board.unmarked_sum());
        }
        // board.print();
    });
}
