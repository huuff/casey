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
                    *occurrences.entry(case).or_insert(1) += 1;
                }
            }
        }

        Ok(CaseReport { occurrences })
    }
}

#[cfg(test)]
mod tests {
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

        Ok(())
    }
}

