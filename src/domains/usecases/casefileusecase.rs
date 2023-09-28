use crate::domains::entities::case_file::CaseFileEntity;
use std::error::Error;

pub trait CaseFileUseCase {
    fn create_case_file(
        &self,
        title: String,
        datetime: u64,
    ) -> Result<(CaseFileEntity), Box<dyn Error>>;
    fn get_case_file(&self, case_number: u32) -> Result<CaseFileEntity, Box<dyn Error>>;
    fn add_evidence(&self, case_number: u32, evidence_id: u64) -> Result<(), Box<dyn Error>>;
    fn add_note(&self, case_number: u32, note_id: u64) -> Result<(), Box<dyn Error>>;
}
