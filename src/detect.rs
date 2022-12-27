use crate::case::Case;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetectError<'a> {
   #[error("input '{0}' is not a word")]
    InvalidInputError(&'a str)
}

// TOOD: Rename "word" to "token"
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn errors_on_non_token_input() {
        let result= Case::detect("not token");

        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "input 'not token' is not a word");
    }
}
