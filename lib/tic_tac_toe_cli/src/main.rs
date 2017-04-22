extern crate tic_tac_toe;
use tic_tac_toe::{Board, Player};

fn main() {
    use std::io::{self, BufRead}; //import std::io as well as std::io::BufRead

    let mut board = Board::new(Player::O);
    // println!("{}", board);

    let stdin = io::stdin(); //for user input
    let handle = stdin.lock();

    let mut lines = handle
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty());

    loop {
        println!("{}", board);

        match board.winning_player() {
            Some(winner) => {
                println!("Congrats, {} is the winner!", winner);
                break;
            } 
            None => println!("Next Input: {}", board.next_player()),
        };

        println!("Enter the row number:");

        let row = match lines.next() {
            //Some(Ok(ref r)) if r.is_empty() => continue,
            Some(r) => r.parse().unwrap(),
            None => break,
        };

        println!("Enter the column number:");

        let column = match lines.next() {
            //Some(Ok(ref c)) if c.is_empty() => continue,
            Some(c) => c.parse().unwrap(),
            None => break,
        };

        match board.choose_cell(row, column) {
            Ok(()) => (),
            Err(e) => println!("Input row and column again {}", e),
            // Err(_) => println!("Input row and column again"),
        };

        //Board::decide_winner(&mut board);
    }
}
