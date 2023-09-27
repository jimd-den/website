#[derive(Debug)]
pub struct CaseFileEntity {
    pub case_number: u32,
    pub datetime: u64,
    pub evidence: Vec<u32>,
}

impl CaseFileEntity {
    pub fn new(case_number: u32, datetime: u64, evidence: Vec<u32>) -> Self { Self { case_number, datetime, evidence } }
}

