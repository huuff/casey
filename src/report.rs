use crate::case::Case;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use crate::detect::CaseDetect;

pub struct CaseReport<T> {
    pub occurrences: HashMap<Case, T>,
}

type IntegerCaseReport = CaseReport<u32>;
type ProportionCaseReport = CaseReport<f32>;

impl IntegerCaseReport {
    pub fn from<T: BufRead>(input: &mut T) -> Result<Self, Box<dyn Error>> {
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

    pub fn proportions(&self) -> ProportionCaseReport {
       let total_occurrences: u32 = self.occurrences.values().sum();

       ProportionCaseReport {
           occurrences: self.occurrences.clone()
                                        .into_iter()
                                        .map(|(case, occ)| (case, (occ as f32/total_occurrences as f32)))
                                        .collect()
       }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    use std::collections::HashSet;

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

    #[test]
    fn correct_proportions() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let mut reader = BufReader::new(r#"
            camelCaseFirst snake_case_first
            camelCaseSecond camelCaseThird
            snake_case_second snake_case_third PascalCase
        "#.as_bytes());
        
        // ACT
        let report = CaseReport::from(&mut reader)?;
        let proportions = report.proportions();

        // ASSERT
        assert_eq!(proportions.occurrences[&Case::CamelCase], 0.42857143_f32);
        assert_eq!(proportions.occurrences[&Case::SnakeCase], 0.42857143_f32);
        assert_eq!(proportions.occurrences[&Case::PascalCase], 0.14285715_f32);

        Ok(())
    }
}

