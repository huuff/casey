use crate::case::Case;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};
use crate::detect::CaseDetect;

pub struct CaseReport {
    pub occurrences: HashMap<Case, u32>,
}

impl CaseReport {
    pub fn from<T: BufRead>(input: &mut T) -> Result<CaseReport, Box<dyn Error>> {
        let mut occurrences: HashMap<Case, u32> = HashMap::new(); 
        for line in input.lines() {
            for token in line?.split_whitespace() {
                if let Some(case) = Case::detect(token)? {
                    *occurrences.entry(case).or_insert(0) += 1;
                }
            }
        }

        Ok(CaseReport { occurrences })
    }

    pub fn main(&self) -> Option<&Case> {
            self.occurrences.iter()
                            .max_by(|x, y| x.1.cmp(y.1))
                            .map(|it| it.0)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn finds_single_camel_case() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let mut reader = BufReader::new("camelCase".as_bytes());

        // ACT
        let report = CaseReport::from(&mut reader)?;

        // ASSERT
        let present_cases: Vec<&Case> = report.occurrences.keys().collect();
        assert_eq!(present_cases.len(), 1);
        assert_eq!(present_cases[0], &Case::CamelCase);
        assert_eq!(report.occurrences[&Case::CamelCase], 1);

        Ok(())
    }

    #[test]
    fn finds_several_cases() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let mut reader = BufReader::new(r#"
            camelCaseFirst snake_case_first
            camelCaseSecond camelCaseThird
            snake_case_second snake_case_third PascalCase
        "#.as_bytes());

        // ACT
        let report = CaseReport::from(&mut reader)?;

        // ASSERT
        let present_cases: HashSet<&Case> = report.occurrences.keys().collect();
        assert_eq!(present_cases.len(), 3);
        assert_eq!(report.occurrences[&Case::CamelCase], 3);
        assert_eq!(report.occurrences[&Case::SnakeCase], 3);
        assert_eq!(report.occurrences[&Case::PascalCase], 1);

        Ok(())
    }

    #[test]
    fn finds_most_used_case() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let mut reader = BufReader::new(r#"
            camelCaseFirst snake_case_first
            camelCaseSecond camelCaseThird
            snake_case_second snake_case_third PascalCase
            camelCase
        "#.as_bytes());

        // ACT
        let report = CaseReport::from(&mut reader)?;
        let main_case = report.main();

        // ASSERT
        assert!(main_case.is_some());
        assert_eq!(main_case.unwrap(), &Case::CamelCase);

        Ok(())
    }
}

