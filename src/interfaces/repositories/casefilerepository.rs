use crate::domains::entities::case_file::CaseFileEntity;
use std::error::Error;
pub trait CaseFileRepository {
    fn save_case_file(&mut self, case_file: CaseFileEntity) -> Result<CaseFileEntity, Box<dyn Error>>;
    fn get(&self, id: u32) -> Result<CaseFileEntity, Box<dyn Error>>;
    // Other methods...
}

pub trait CaseFileRepositoryNew {
    fn new() -> Self;
}