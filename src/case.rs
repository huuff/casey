use strum_macros::EnumIter;
use std::fmt::{Formatter, Display, Result as FormatResult};

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone)]
pub enum Case {
    // Uppercase
    PascalCase,
    ShoutingSnakeCase,

    // Lowercase
    CamelCase,
    SnakeCase,
    KebabCase,
}

// TODO: Test
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
