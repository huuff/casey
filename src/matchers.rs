use regex::Regex;
use crate::case::Case;

pub trait CaseMatcher {
    fn matcher(&self) -> Regex;
}

impl CaseMatcher for Case {
    fn matcher(&self) -> Regex {
        // TODO All of this should be lazy with once_cell
        match self {
            Case::CamelCase => Regex::new(r"^[a-z]([a-z]|[A-Z])*$").unwrap(),
            Case::ShoutingSnakeCase => Regex::new(r"^([A-Z]|_)*$").unwrap(),
            Case::PascalCase => Regex::new(r"^[A-Z]([a-z]|[A-Z])*$").unwrap(),
            Case::SnakeCase => Regex::new(r"^([a-z]|_)*$").unwrap(),
            Case::KebabCase => Regex::new(r"^([a-z]|-)*$").unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_camel_case() {
        let result = Case::CamelCase.matcher().is_match("camelCase");

        assert!(result);
    }

    #[test]
    fn detects_not_camel_case() {
        let result = Case::CamelCase.matcher().is_match("camel_case");

        assert!(!result);
    }

    #[test]
    fn detects_shouting_snake_case() {
        let result = Case::ShoutingSnakeCase.matcher().is_match("SHOUTING_SNAKE");

        assert!(result);
    }

    #[test]
    fn detects_not_shouting_snake() {
        let result = Case::ShoutingSnakeCase.matcher().is_match("shoutingSnake");

        assert!(!result);
    }

    #[test]
    fn detects_pascal_case() {
        let result = Case::PascalCase.matcher().is_match("PascalCase");

        assert!(result);
    }

    #[test]
    fn detects_not_pascal_case() {
        let result = Case::PascalCase.matcher().is_match("pascal-case");

        assert!(!result);
    }

    #[test]
    fn detects_snake_case() {
        let result = Case::SnakeCase.matcher().is_match("snake_case");

        assert!(result);
    }

    #[test]
    fn detects_not_snake_case() {
        let result = Case::SnakeCase.matcher().is_match("snake-case");
        
        assert!(!result);
    }

    #[test]
    fn detects_kebab_case() {
        let result = Case::KebabCase.matcher().is_match("kebab-case");

        assert!(result);
    }

    #[test]
    fn detects_not_kebab_case() {
        let result = Case::KebabCase.matcher().is_match("kebab_case");

        assert!(!result);
    }
}
