pub struct Board {
    state: [Option<Player>; 9],
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
        self.state[0]
            .and_then(|res| {
                if Some(res) == self.state[1] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[2] {
                    Some(res)
                } else {
                    None
                }
            })
    }

    fn check_row2(&self) -> Option<Player> {
        self.state[3]
            .and_then(|res| {
                if Some(res) == self.state[4] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[5] {
                    Some(res)
                } else {
                    None
                }
            })
    }

    fn check_row3(&self) -> Option<Player> {
        self.state[6]
            .and_then(|res| {
                if Some(res) == self.state[7] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[8] {
                    Some(res)
                } else {
                    None
                }
            })
    }

    fn check_columns(&self) -> Option<Player> {
        self.check_col1()
            .or_else(|| self.check_col2())
            .or_else(|| self.check_col3())
    }

    fn check_col1(&self) -> Option<Player> {
        self.state[0]
            .and_then(|res| {
                if Some(res) == self.state[3] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[6] {
                    Some(res)
                } else {
                    None
                }
            })
    }

    fn check_col2(&self) -> Option<Player> {
        self.state[1]
            .and_then(|res| {
                if Some(res) == self.state[4] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[7] {
                    Some(res)
                } else {
                    None
                }
            })
    }

    fn check_col3(&self) -> Option<Player> {
        self.state[2]
            .and_then(|res| {
                if Some(res) == self.state[5] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[8] {
                    Some(res)
                } else {
                    None
                }
            })
    }

    fn check_diagonals(&self) -> Option<Player> {
        self.check_diag1().or_else(|| self.check_diag2())
    }

    fn check_diag1(&self) -> Option<Player> {
        self.state[0]
            .and_then(|res| {
                if Some(res) == self.state[4] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[8] {
                    Some(res)
                } else {
                    None
                }
            })
    }

    fn check_diag2(&self) -> Option<Player> {
        self.state[2]
            .and_then(|res| {
                if Some(res) == self.state[4] {
                    Some(res)
                } else {
                    None
                }
            })
            .and_then(|res| {
                if Some(res) == self.state[6] {
                    Some(res)
                } else {
                    None
                }
            })
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Player {
    O,
    X,
}
