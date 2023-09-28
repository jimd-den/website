use std::error::Error;

use crate::domains::usecases::casefileusecase::CaseFileUseCase;
use crate::domains::entities::case_file::CaseFileEntity;

use super::models::CreateCaseFileInput;

pub struct CaseFileController {
    use_case: Box<dyn CaseFileUseCase>,
}

impl CaseFileController {
    pub fn new(use_case: Box<dyn CaseFileUseCase>) -> Self {
        Self { use_case }
    }
    pub fn create(&self, case_file: CaseFileEntity) -> Result<CaseFileEntity, Box<dyn Error>> {
        let title = case_file.title;
        let datetime = case_file.datetime;

        
        self.use_case.create_case_file(title, datetime)
    }
}