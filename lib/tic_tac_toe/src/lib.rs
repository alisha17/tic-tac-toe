use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    cells: [[Option<Player>; 3]; 3],
    next_player: Player,
}

impl Board {
    pub fn new(starting_player: Player) -> Board {
        Board {
            cells: [[None; 3]; 3],
            next_player: starting_player,
        }
    }

    pub fn next_player(&self) -> Player {
        self.next_player
    }

    pub fn choose_cell(&mut self, row: usize, column: usize) -> Result<(), &'static str> {
        if row > 2 || column > 2 {
            return Err("Index out of bounds.");
        } else if self.cells[row][column] != None {
            return Err("The cell is not empty. Try another row and column.");
        }

        self.cells[row][column] = Some(self.next_player);
        self.next_player = match self.next_player {
            Player::O => Player::X,
            Player::X => Player::O,
        };
        Ok(())
    }

    pub fn winning_player(&mut self) -> Option<Player> {
        let win_elem = Some(match self.next_player {
                                Player::O => Player::X,
                                Player::X => Player::O,
                            });

        if (self.cells[0][0] == win_elem && self.cells[0][1] == win_elem &&
            self.cells[0][2] == win_elem) ||
           (self.cells[0][2] == win_elem && self.cells[1][2] == win_elem &&
            self.cells[2][2] == win_elem) ||
           (self.cells[2][0] == win_elem && self.cells[2][1] == win_elem &&
            self.cells[2][2] == win_elem) ||
           (self.cells[0][0] == win_elem && self.cells[1][0] == win_elem &&
            self.cells[2][0] == win_elem) ||
           (self.cells[1][0] == win_elem && self.cells[1][1] == win_elem &&
            self.cells[1][2] == win_elem) ||
           (self.cells[0][0] == win_elem && self.cells[1][1] == win_elem &&
            self.cells[2][2] == win_elem) ||
           (self.cells[0][2] == win_elem && self.cells[1][1] == win_elem &&
            self.cells[2][0] == win_elem) {
            win_elem
        } else {
            None
        }
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
            //enumerate is used for cell indexes
            write!(f, "| {} |", i)?;
            for player in row.iter() {
                match *player {
                    Some(player) => {
                        match write!(f, " {} |", player) {
                            Ok(()) => (),
                            Err(err) => return Err(err),
                        }
                    }
                    None => write!(f, "   |")?, //'?' is used for the same Ok,Err code done above
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
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