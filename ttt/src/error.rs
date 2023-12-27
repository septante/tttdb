use thiserror::Error;

#[derive(Error, Debug)]
pub enum TTTError {
    #[error("bad input")]
    InputError,
}
