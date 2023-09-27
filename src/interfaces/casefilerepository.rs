use crate::entities::case_file::CaseFileEntity;
use std::error::Error;
pub trait CaseFileRepository {
    fn save_case_file(&self, case_file: CaseFileEntity) -> Result<(), Box<dyn Error>>;
    fn get_case_file_by_id(&self, id: u32) -> Result<Option<CaseFileEntity>, Box<dyn Error>>;
}