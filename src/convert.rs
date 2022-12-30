use crate::case::Case;
use std::io::{Write, BufRead, Read};
use heck::*;

pub trait ConvertCaseTo {
    fn convert_case_to(&self, target_case: Case) -> String;
}

impl ConvertCaseTo for str {
    fn convert_case_to(&self, target_case: Case) -> String {
        match target_case {
            Case::PascalCase => self.to_upper_camel_case(),
            Case::ShoutingSnakeCase => self.to_shouty_snake_case(),
            Case::CamelCase => self.to_lower_camel_case(),
            Case::SnakeCase => self.to_snake_case(),
            Case::KebabCase => self.to_kebab_case(),
        }
    }
}

pub trait BufferedConvert {
    fn buffered_convert<const SIZE: usize>(from_to_cases: [(Case, Case); SIZE], target: Box<dyn Write>);
}

impl <T: BufRead> BufferedConvert for T {
    fn buffered_convert<const SIZE: usize>(from_to_cases: [(Case, Case); SIZE], target: Box<dyn Write>) {
       todo!("Actual implementation"); 
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_kebab_case() {
        assert_eq!("camel-case", "camelCase".convert_case_to(Case::KebabCase));
    }
}
