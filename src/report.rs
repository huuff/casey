use crate::case::Case;
use std::collections::HashMap;
use std::io::Read;

pub struct CaseReport {
    pub case_to_occurrences: HashMap<Case, u32>,
}

impl CaseReport {
    pub fn from<T: Read>(input: &mut T) -> CaseReport {
        todo!("Gonna need a better BufReader read_until for this...");
    }
}
