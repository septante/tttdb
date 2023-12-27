use crate::error::TTTError;

#[derive(Default)]
pub struct Board {
    state: [Space; 9],
}

impl Board {
    pub fn check_win(&self) -> Option<Player> {
        self.check_rows()
            .or_else(|| self.check_columns())
            .or_else(|| self.check_diagonals())
    }

    fn check_rows(&self) -> Option<Player> {
        self.check_row1()
            .or_else(|| self.check_row2())
            .or_else(|| self.check_row3())
    }

    fn check_row1(&self) -> Option<Player> {
        let x = self.state[0];
        if x.is_empty() {
            return None;
        }

        if x == self.state[1] && x == self.state[2] {
            x.player()
        } else {
            None
        }
    }

    fn check_row2(&self) -> Option<Player> {
        let x = self.state[3];
        if x.is_empty() {
            return None;
        }

        if x == self.state[4] && x == self.state[5] {
            x.player()
        } else {
            None
        }
    }

    fn check_row3(&self) -> Option<Player> {
        let x = self.state[6];
        if x.is_empty() {
            return None;
        }

        if x == self.state[7] && x == self.state[8] {
            x.player()
        } else {
            None
        }
    }

    fn check_columns(&self) -> Option<Player> {
        self.check_col1()
            .or_else(|| self.check_col2())
            .or_else(|| self.check_col3())
    }

    fn check_col1(&self) -> Option<Player> {
        let x = self.state[0];
        if x.is_empty() {
            return None;
        }

        if x == self.state[3] && x == self.state[6] {
            x.player()
        } else {
            None
        }
    }

    fn check_col2(&self) -> Option<Player> {
        let x = self.state[1];
        if x.is_empty() {
            return None;
        }

        if x == self.state[4] && x == self.state[7] {
            x.player()
        } else {
            None
        }
    }

    fn check_col3(&self) -> Option<Player> {
        let x = self.state[2];
        if x.is_empty() {
            return None;
        }

        if x == self.state[5] && x == self.state[8] {
            x.player()
        } else {
            None
        }
    }

    fn check_diagonals(&self) -> Option<Player> {
        self.check_diag1().or_else(|| self.check_diag2())
    }

    fn check_diag1(&self) -> Option<Player> {
        let x = self.state[0];
        if x.is_empty() {
            return None;
        }

        if x == self.state[4] && x == self.state[8] {
            x.player()
        } else {
            None
        }
    }

    fn check_diag2(&self) -> Option<Player> {
        let x = self.state[2];
        if x.is_empty() {
            return None;
        }

        if x == self.state[4] && x == self.state[6] {
            x.player()
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Board {
    type Error = TTTError;

    fn try_from(s: &str) -> Result<Self, TTTError> {
        let mut board = Board::default();

        if s.len() != 9 {
            return Err(TTTError::InputError);
        }

        let chars = s.chars();
        let spaces = chars.map(Space::try_from);
        for (i, space) in spaces.enumerate() {
            if let Ok(player) = space {
                board.state[i] = player;
            } else {
                return Err(TTTError::InputError);
            }
        }

        Ok(board)
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Player {
    O,
    X,
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum Space {
    Filled {
        player: Player,
    },
    #[default]
    Empty,
}

impl TryFrom<char> for Player {
    type Error = TTTError;

    fn try_from(c: char) -> Result<Self, TTTError> {
        if c == 'O' || c == 'o' {
            Ok(Player::O)
        } else if c == 'X' || c == 'x' {
            Ok(Player::X)
        } else {
            Err(TTTError::InputError)
        }
    }
}

impl TryFrom<&str> for Player {
    type Error = TTTError;

    fn try_from(s: &str) -> Result<Self, TTTError> {
        if s == "O" || s == "o" {
            Ok(Player::O)
        } else if s == "X" || s == "x" {
            Ok(Player::X)
        } else {
            Err(TTTError::InputError)
        }
    }
}

impl Space {
    fn is_empty(&self) -> bool {
        self == &Space::Empty
    }

    fn player(&self) -> Option<Player> {
        match self {
            Self::Empty => None,
            Self::Filled { player } => Some(player.to_owned()),
        }
    }
}

impl From<Player> for Space {
    fn from(player: Player) -> Self {
        Self::Filled { player }
    }
}

impl TryFrom<char> for Space {
    type Error = TTTError;

    fn try_from(c: char) -> Result<Self, TTTError> {
        if c == ' ' {
            return Ok(Self::Empty);
        }

        match Player::try_from(c) {
            Ok(player) => Ok(player.into()),
            Err(e) => Err(e),
        }
    }
}

impl TryFrom<&str> for Space {
    type Error = TTTError;

    fn try_from(s: &str) -> Result<Self, TTTError> {
        if s.is_empty() || s == " " || s == "-" {
            return Ok(Self::Empty);
        }

        match Player::try_from(s) {
            Ok(player) => Ok(player.into()),
            Err(e) => Err(e),
        }
    }
}
