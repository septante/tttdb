use crate::{
    error::TTTError,
    state::{Board, Player, Space},
};

#[derive(Default, Debug)]
pub struct Game {
    pub(crate) board: Board,
    pub(crate) moves: Vec<usize>,
    pub(crate) turn: Player,
}

impl Game {
    pub fn turn(&self, x: usize, y: usize) -> Result<(), TTTError> {
        if x > 2 || y > 2 {
            return Err(TTTError::IllegalMove);
        }

        let i = y * 3 + x;
        if self.board.state[i] != Space::Empty {
            return Err(TTTError::IllegalMove);
        }

        Ok(())
    }
}
