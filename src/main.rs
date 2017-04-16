use std::fmt;

#[derive(Debug, Copy, Clone)]
struct Board {
    cells: [[Option<Player>; 3]; 3],
    next_player: Player,
}

impl Board {
    fn new(starting_player: Player) -> Board {
        Board {
            cells: [[None; 3]; 3],
            next_player: starting_player,
        }
    }

    fn choose_cell(&mut self, row: usize, column: usize) -> Result<(), &'static str> {
        if row > 2 || column > 2 {
            return Err("Index out of bounds.");
        }
        self.cells[row][column] = Some(self.next_player);
        self.next_player = match self.next_player {
            Player::O => Player::X,
            Player::X => Player::O,
        };
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    |")?;
        for i in 0..3 {
            write!(f, " {} |", i)?;
        }
        writeln!(f, "")?;

        for (i, row) in self.cells.iter().enumerate() {
            write!(f, "| {} |", i)?;
            for player in row.iter() {
                match *player {
                    Some(player) => {
                        match write!(f, " {} |", player) {
                            Ok(()) => (),
                            Err(err) => return Err(err),
                        }
                    }
                    None => write!(f, "   |")?,
                }
            }
            writeln!(f, "")?;
        }

        writeln!(f, "Next Input: {}", self.next_player)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    O,
    X,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::O => write!(f, "O"),
            Player::X => write!(f, "X"),
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let mut board = Board::new(Player::O);
    // println!("{}", board);

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();


    loop {
        println!("{}", board);
        println!("Enter the row number:");
        let row = match lines.next() {
            Some(r) => r.unwrap().parse().unwrap(),
            None => break,
        };

        println!("Enter the column number:");
        let column = match lines.next() {
            Some(c) => c.unwrap().parse().unwrap(),
            None => break, 
        };


        match Board::choose_cell(&mut board, row, column) {
            Ok(()) => (),
            Err(e) => println!("Input row and column again {}", e),
            // Err(_) => println!("Input row and column again"),
        };

    }


}
