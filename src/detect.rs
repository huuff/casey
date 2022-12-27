use crate::case::Case;
use thiserror::Error;
use strum::IntoEnumIterator;
use crate::matchers::CaseMatcher;

#[derive(Error, Debug)]
pub enum DetectError<'a> {
   #[error("input '{0}' is not a token")]
    InvalidInputError(&'a str)
}

trait CaseDetect {
    fn detect(token: &str) -> Result<Option<Case>, DetectError>;
}

impl CaseDetect for Case {
    fn detect(token: &str) -> Result<Option<Case>, DetectError> {
        if token.chars().any(|c| c.is_whitespace()) {
            return Err(DetectError::InvalidInputError(token));
        }

        let mut matching_cases: Vec<Case> = Case::iter().filter(|c| c.matcher().is_match(token)).collect();

        if matching_cases.len() == 1 {
            return Ok(Some(matching_cases.remove(0)));
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn errors_on_non_token_input() {
        let result = Case::detect("not token");

        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "input 'not token' is not a token");
    }

}
