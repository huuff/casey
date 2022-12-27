use crate::case::Case;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetectError<'a> {
   #[error("input {0} is not a word")]
    InvalidInputError(&'a str)
}

trait CaseDetect {
    fn detect(word: &str) -> Result<Option<Case>, DetectError>;
}

impl CaseDetect for Case {
    fn detect(word: &str) -> Result<Option<Case>, DetectError> {
        if word.chars().any(|c| c.is_whitespace()) {
            return Err(DetectError::InvalidInputError(word));
        }

        todo!("TODO: Normal flow of the function")
    }
}
