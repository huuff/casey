use crate::{case::Case, detect::CaseDetect};
use std::io::{Write, BufRead};
use heck::*;
use split_preserve::*;
use std::error::Error;

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
    fn buffered_convert<'a, const SIZE: usize>(&mut self, from_to_cases: [(Case, Case); SIZE], output: Box<&mut (dyn Write + 'a)>) -> Result<(), Box<dyn Error>>;
}

impl <T: BufRead> BufferedConvert for T {
    // TODO: I guess I should be outputting newlines too
    fn buffered_convert<'a, const SIZE: usize>(&mut self, from_to_cases: [(Case, Case); SIZE], output: Box<&mut (dyn Write + 'a)>) -> Result<(), Box<dyn Error>> {
        for line in self.lines() {
            let line = line?;
            let converted_line = SplitPreserveWS::new(line.as_str())
                .map_words(|w| {
                    let case = Case::detect(w).unwrap();
                    if let Some(case) = case {
                        for (source_case, target_case) in from_to_cases {
                           if source_case == case {
                                return w.convert_case_to(target_case);
                           } 
                        }
                    } 
                    return String::from(w);
                })
                .collect::<String>()
            ;
            (*output).write_all(converted_line.as_bytes())?;
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use indoc::indoc;

    #[test]
    fn to_kebab_case() {
        assert_eq!("camel-case", "camelCase".convert_case_to(Case::KebabCase));
    }

    #[test]
    fn converts_single_line_single_case() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let mut input = BufReader::new(
            "snake_case text with lowercase_words_separated_by_underscore".as_bytes()
        );
        let mut output = vec![];

        // ACT
        input.buffered_convert([(Case::SnakeCase, Case::CamelCase)], Box::new(&mut output))?;
        let output = String::from_utf8(output)?;


        // ASSERT
        assert_eq!(output, String::from("snakeCase text with lowercaseWordsSeparatedByUnderscore"));
        Ok(())
    }
    #[test]
    fn works_with_multiple_newlines() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let mut input = BufReader::new(indoc! {r#"
            ThisIsSome text with
            SomePascalCased words and
            also some NewLines interspesed
        "#}.trim().as_bytes());
        let mut output = vec![];

        // ACT
        input.buffered_convert([(Case::PascalCase, Case::KebabCase)], Box::new(&mut output))?;
        let output = String::from_utf8(output)?;


        // ASSERT
        assert_eq!(output, indoc! {r#"
            this-is-some text with
            some-pascal-cased words and
            also some new-lines interspesed
        "#}.trim());
        Ok(())
    }
}
