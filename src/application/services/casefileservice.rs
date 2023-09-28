use crate::{interfaces::repositories::casefilerepository::CaseFileRepository, domains::{usecases::casefileusecase::CaseFileUseCase, entities::case_file::CaseFileEntity}};
use std::error::Error;
pub struct CaseFileService{
    repository: Box<dyn CaseFileRepository>,
    create_use_case: Box<dyn CaseFileUseCase>
}

impl CaseFileService {
    pub fn new(repository: Box<dyn CaseFileRepository>, create_use_case: Box<dyn CaseFileUseCase>) -> Self {
        Self {
            repository,
            create_use_case
        }
    }

    pub fn create_case_file(&mut self, casefile: CaseFileEntity) -> Result<CaseFileEntity, Box<dyn Error>> {
        self.repository.save_case_file(casefile)
    }

}