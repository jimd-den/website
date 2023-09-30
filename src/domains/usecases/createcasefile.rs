use std::error::Error;

use crate::{interfaces::repositories::casefilerepository::CaseFileRepository, domains::entities::case_file::CaseFileEntity};

pub struct CreateCaseFile {
    case_file_repository: Box<dyn CaseFileRepository>,
}

impl CreateCaseFile {
    pub fn new(case_file_repository: Box<dyn CaseFileRepository>) -> Self {
        Self {
            case_file_repository,
        }
    }
    pub fn create_case_file(&mut self, case_file: CaseFileEntity) -> Result<CaseFileEntity, Box<dyn Error>> {
        self.case_file_repository.save_case_file(case_file)
    }
}