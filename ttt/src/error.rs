use thiserror::Error;

#[derive(Error, Debug)]
pub enum TTTError {
    #[error("Bad input")]
    InputError,
    #[error("Illegal move")]
    IllegalMove,
}
