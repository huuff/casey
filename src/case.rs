use strum_macros::EnumIter;
use std::fmt::{Formatter, Display, Result as FormatResult};
use clap::ValueEnum;

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy, ValueEnum)]
// TODO: Some clap help
pub enum Case {
    // Uppercase
    #[value(aliases = ["PascalCase", "pc"]) ]
    PascalCase,
    #[value(aliases = ["SHOUTING_SNAKE_CASE", "ssc"]) ]
    ShoutingSnakeCase,

    // Lowercase
    #[value(aliases = ["camelCase", "cc"])]
    CamelCase,
    #[value(aliases = ["snake_case", "sc"])]
    SnakeCase,
    #[value(aliases = ["kebab-case", "kc"])]
    KebabCase,
}

impl Display for Case {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", match self {
            Case::PascalCase => "PascalCase",
            Case::ShoutingSnakeCase => "SHOUTING_SNAKE_CASE",
            Case::CamelCase => "camelCase",
            Case::SnakeCase => "snake_case",
            Case::KebabCase => "kebab-case",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_camel_case() {
        assert_eq!(format!("{}", Case::CamelCase), "camelCase");
    }
}
