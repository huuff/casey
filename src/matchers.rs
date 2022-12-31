use regex::Regex;
use crate::case::Case;
use once_cell_regex::regex;

pub trait CaseMatcher {
    fn matcher(&self) -> &Regex;
}

impl CaseMatcher for Case {
    fn matcher(&self) -> &Regex {
        match self {
            Case::CamelCase => regex!(r"^[a-z]([a-z]|[A-Z])*$"),
            Case::ShoutingSnakeCase => regex!(r"^([A-Z]|_)+$"),
            Case::PascalCase => regex!(r"^[A-Z]([a-z]|[A-Z])*$"),
            Case::SnakeCase => regex!(r"^([a-z]|_)+$"),
            Case::KebabCase => regex!(r"^([a-z]|-)+$"),
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
