use crate::case::Case;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use crate::detect::CaseDetect;
use num_traits::Num;
use thiserror::Error as ThisError;
use std::fmt::{Formatter, Display, Result as FormatResult};
use itertools::Itertools;

#[derive(ThisError, PartialEq, Debug)]
pub enum CaseReportError {
    #[error("source report frequency '{0}' is not in the 0..1 range, so it can't be converted to percentages")]
    PercentageConversionError(f32),
}

#[derive(Debug)]
pub struct CaseReport<T> {
    pub frequencies: HashMap<Case, T>,
}

pub type FrequencyCaseReport = CaseReport<u32>;
type ProportionCaseReport = CaseReport<f32>;

impl <T: Num + Display + PartialOrd> Display for CaseReport<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let result = self.frequencies.iter()
                                     .sorted_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
                                     .rev()
                                     .map(|(case, freq)| format!("{case}: {freq}"))
                                     .join("\n")
                                     ;

        write!(f, "{}", result)
    }
}

impl FrequencyCaseReport {
    pub fn from<T: BufRead>(input: &mut T) -> Result<Self, Box<dyn Error>> {
        let mut frequencies: HashMap<Case, u32> = HashMap::new(); 
        for line in input.lines() {
            for token in line?.split_whitespace() {
                if let Some(case) = Case::detect(token)? {
                    *frequencies.entry(case).or_insert(0) += 1;
                }
            }
        }

        Ok(CaseReport { frequencies })
    }

    pub fn proportions(&self) -> ProportionCaseReport {
       let total_frequencies: u32 = self.frequencies.values().sum();

       ProportionCaseReport {
           frequencies: self.frequencies.clone()
                                        .into_iter()
                                        .map(|(case, occ)| (case, (occ as f32/total_frequencies as f32)))
                                        .collect()
       }
    }
}

impl <T: Num + Ord> CaseReport<T> {
    // TODO: What if there's a tie?
    pub fn main(&self) -> Option<&Case> {
            self.frequencies.iter()
                            .max_by(|x, y| x.1.cmp(y.1))
                            .map(|it| it.0)
    }
}

#[derive(Debug)]
pub struct PercentageCaseReport(ProportionCaseReport);

impl Display for PercentageCaseReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let result = self.0.frequencies.iter()
                                     .sorted_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
                                     .rev()
                                     .map(|(case, freq)| format!("{case}: {freq}%"))
                                     .join("\n")
                                     ;

        write!(f, "{}", result)
    }
}

impl ProportionCaseReport {
    pub fn as_percentages(&self) -> Result<PercentageCaseReport, CaseReportError> {
        for proportion in self.frequencies.values() {
            if !(0f32..=1f32).contains(proportion) {
                return Err(CaseReportError::PercentageConversionError(*proportion))
            }
        }

        Ok(PercentageCaseReport(
            ProportionCaseReport {
                frequencies: self.frequencies.clone()
                                             .into_iter()
                                             .map(|(x, y)| (x, y * 100_f32))
                                             .collect()
                    
            })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::collections::{HashSet, HashMap};
    use approx::assert_relative_eq;
    use indoc::indoc;

    #[test]
    fn finds_single_camel_case() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let mut reader = BufReader::new("camelCase".as_bytes());

        // ACT
        let report = CaseReport::from(&mut reader)?;

        // ASSERT
        let present_cases: Vec<&Case> = report.frequencies.keys().collect();
        assert_eq!(present_cases.len(), 1);
        assert_eq!(present_cases[0], &Case::CamelCase);
        assert_eq!(report.frequencies[&Case::CamelCase], 1);

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
        let present_cases: HashSet<&Case> = report.frequencies.keys().collect();
        assert_eq!(present_cases.len(), 3);
        assert_eq!(report.frequencies[&Case::CamelCase], 3);
        assert_eq!(report.frequencies[&Case::SnakeCase], 3);
        assert_eq!(report.frequencies[&Case::PascalCase], 1);

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
        assert_eq!(proportions.frequencies[&Case::CamelCase], 0.42857143_f32);
        assert_eq!(proportions.frequencies[&Case::SnakeCase], 0.42857143_f32);
        assert_eq!(proportions.frequencies[&Case::PascalCase], 0.14285715_f32);

        Ok(())
    }

    #[test]
    fn as_percentages() -> Result<(), Box<dyn Error>> {
        // ARRANGE
        let proportion_report = ProportionCaseReport {
            frequencies: HashMap::from([
                (Case::CamelCase, 0.25_f32),
                (Case::PascalCase, 0.3_f32),
                (Case::SnakeCase, 0.45_f32),
            ]),
        };

        // ACT
        let percentages_report = proportion_report.as_percentages()?;

        // ASSERT
        assert_relative_eq!(percentages_report.0.frequencies[&Case::CamelCase], 25_f32);
        assert_relative_eq!(percentages_report.0.frequencies[&Case::PascalCase], 30_f32);
        assert_relative_eq!(percentages_report.0.frequencies[&Case::SnakeCase], 45_f32);

        Ok(())
    }

    #[test]
    fn as_percentages_bad_input() {
        // ARRANGE
        let proportion_report = ProportionCaseReport {
            frequencies: HashMap::from([(Case::CamelCase, 100_f32)])
        };

        // ACT
        let as_percentages = proportion_report.as_percentages();

        // ASSERT
        assert!(as_percentages.is_err());
        assert_eq!(as_percentages.unwrap_err(), CaseReportError::PercentageConversionError(100_f32));
    }

    #[test]
    fn frequency_case_report_display() {
        // ARRANGE
        let report = CaseReport {
            frequencies: HashMap::from([
                (Case::CamelCase, 1),
                (Case::SnakeCase, 2),
                (Case::PascalCase, 3)
            ]),
        };

        // ACT
        let result = format!("{}", report);

        // ASSERT
        assert_eq!(result, indoc! {r#"
            PascalCase: 3
            snake_case: 2
            camelCase: 1
        "#}.trim());
    }

    #[test]
    fn percentage_case_report_display() {
        // ARRANGE
        let report = PercentageCaseReport(
            ProportionCaseReport {
                frequencies: HashMap::from([
                    (Case::SnakeCase, 22.5_f32),
                    (Case::KebabCase, 43.3_f32),
                    (Case::PascalCase, 19.99_f32),
                ]),
            }
        );

        // ACT
        let display = format!("{}", report);

        // ASSERT
        assert_eq!(display, indoc! {r#"
            kebab-case: 43.3%
            snake_case: 22.5%
            PascalCase: 19.99%
        "#}.trim());
    }

}

