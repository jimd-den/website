use std::collections::HashMap;
use crate::domains::entities::case_file::CaseFileEntity;
use crate::interfaces::repositories::casefilerepository::CaseFileRepository;
use std::error::Error;

use super::casefilerepository::CaseFileRepositoryNew;

#[derive(Debug, Clone)]
pub struct InMemoryCaseFileRepository {
    case_files: HashMap<u32, CaseFileEntity>,
}

impl CaseFileRepository for InMemoryCaseFileRepository {
    fn save_case_file(&mut self, case_file: CaseFileEntity) -> Result<CaseFileEntity, Box<dyn Error>> {
        let case_number = case_file.case_number;
        self.case_files.insert(case_number, case_file);
        Ok(self.case_files.get(&case_number).unwrap().clone())
    }
    fn get(&self, case_number: u32) -> Result<CaseFileEntity, Box<dyn Error>> {
        let case_file = self.case_files.get(&case_number);
        match case_file {
            Some(case_file) => Ok(case_file.clone()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Case file not found",
            ))),
        }
    }
}

impl CaseFileRepositoryNew for InMemoryCaseFileRepository {
    fn new() -> Self {
        Self {
            case_files: HashMap::new(),
        }
    }
}