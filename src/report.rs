use crate::case::Case;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
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

