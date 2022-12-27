use crate::case::Case;
use thiserror::Error;
use regex::Regex;

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

        todo!("TODO: Normal flow of the function")
    }
}

trait CaseMatcher {
    fn matcher(&self) -> Regex;
}

impl CaseMatcher for Case {
    fn matcher(&self) -> Regex {
        // TODO All of this should be lazy with once_cell
        match self {
            Case::CamelCase => Regex::new(r"^[a-z]([a-z]|[A-Z])*$").unwrap(),
            Case::ShoutingSnakeCase => Regex::new(r"^([A-Z]|_)*$").unwrap(),
            Case::PascalCase => Regex::new(r"^[A-Z]([a-z]|[A-Z]*)$").unwrap(),
            Case::SnakeCase => Regex::new(r"^([a-z]|_)*$").unwrap(),
            Case::KebabCase => Regex::new(r"^([a-z]|-)*$").unwrap(),
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
        let result = Case::CamelCase.matcher().is_match("camelCase");

        assert!(result);
    }
}
