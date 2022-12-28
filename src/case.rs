use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum Case {
    // Uppercase
    PascalCase,
    ShoutingSnakeCase,

    // Lowercase
    CamelCase,
    SnakeCase,
    KebabCase,
}
