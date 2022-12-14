use crate::case::Case;
use thiserror::Error;
use strum::IntoEnumIterator;
use crate::matchers::CaseMatcher;

#[derive(Error, Debug)]
pub enum DetectError {
   #[error("input '{0}' is not a token")]
    InvalidInputError(String)
}

pub trait CaseDetect {
    fn detect(token: &str) -> Result<Option<Case>, DetectError>;
}

impl CaseDetect for Case {
    fn detect(token: &str) -> Result<Option<Case>, DetectError> {
        if token.chars().any(|c| c.is_whitespace()) {
            return Err(DetectError::InvalidInputError(String::from(token)));
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

    #[test]
    fn detects_camel_case() {
        let result = Case::detect("camelCase");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result, Case::CamelCase);
    }

    #[test]
    fn unable_to_detect_single_lowercase_word() {
        let result = Case::detect("word");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_none());
    }

}
