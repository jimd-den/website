#[derive(Debug, Clone)]
//The CaseFileEntity is how we hold evidence and notes for each case

pub struct CaseFileEntity {
    pub case_number: u32,
    pub title: String,
    pub datetime: u64,
    pub evidence: Vec<u32>,
}

impl CaseFileEntity {
    pub fn new(case_number: u32, title: String, datetime: u64, evidence: Vec<u32>) -> Self {
        Self {
            case_number,
            title,
            datetime,
            evidence,
        }
    }
}
