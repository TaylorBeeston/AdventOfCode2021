mod bingo;
mod input;

use crate::bingo::Board;
use crate::input::{get_boards, get_input};

fn main() {
    let mut boards: Vec<Board> = get_boards().iter().map(|input| Board::new(input)).collect();

    let raw_input = get_input();
    let mut input = raw_input.iter();
    let mut number = Some(&0);

    let mut winners = vec![];

    while winners.len() < boards.len() {
        number = input.next();

        println!("Checking {}", number.unwrap());

        boards = boards
            .iter()
            .map(|board| board.check_num(*number.unwrap()))
            .collect();

        boards.iter().enumerate().for_each(|(index, board)| {
            if board.check_won() && !winners.contains(&index) {
                println!("Pushing winner #{} ({})", winners.len(), index);
                winners.push(index);
            }
        })
    }

    let board = &boards[*winners.last().unwrap()];

    println!("We have a loser! {}", board.unmarked_sum());
    println!("Last number was {}", number.unwrap());
    println!("Answer is {}", number.unwrap() * board.unmarked_sum());
    board.print();
}
