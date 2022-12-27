use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Case {
    // Uppercase
    PascalCase,
    ShoutingSnakeCase,

    // Lowercase
    CamelCase,
    SnakeCase,
    KebabCase,
}
