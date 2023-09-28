use std::collections::HashMap;

use crate::domains::{entities::case_file::CaseFileEntity, usecases::casefileusecase::CaseFileUseCase};

use super::casefilerepository::CaseFileRepository;

pub struct InMemoryCaseFileRepository {
    case_files: HashMap<u32, CaseFileEntity>,
}

impl InMemoryCaseFileRepository {
    pub fn new() -> Box<dyn CaseFileUseCase> {
        Box::new(InMemoryCaseFileRepository {
            case_files: HashMap::new(),
        })
    }
}
impl CaseFileUseCase for InMemoryCaseFileRepository {
    fn create_case_file(
            &self,
            title: String,
            datetime: u64,
        ) -> Result<CaseFileEntity, Box<dyn std::error::Error>> {
        let case_file = CaseFileEntity::new(0,title, datetime,Vec::new());
        Ok(case_file)
    }

    fn get_case_file(&self, case_number: u32) -> Result<CaseFileEntity, Box<dyn std::error::Error>> {
        todo!()
    }

    fn add_evidence(&self, case_number: u32, evidence_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn add_note(&self, case_number: u32, note_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl CaseFileRepository for InMemoryCaseFileRepository {
    fn get_case_file_by_id(
        &self,
        id: u32,
    ) -> Result<Option<CaseFileEntity>, Box<dyn std::error::Error>> {
        Ok(self.case_files.get(&id).cloned())
    }
    fn save_case_file(
        &mut self,
        case_file: CaseFileEntity,
    ) -> Result<CaseFileEntity, Box<dyn std::error::Error>> {
        self.case_files.insert(case_file.case_number, case_file.clone());
        Ok(case_file)
    }
}
