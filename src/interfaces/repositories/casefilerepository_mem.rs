use std::collections::HashMap;

use crate::domains::entities::case_file::CaseFileEntity;

use super::casefilerepository::CaseFileRepository;

pub struct InMemoryCaseFileRepository {
    case_files: HashMap<u32, CaseFileEntity>,
}

impl InMemoryCaseFileRepository {
    pub fn new() -> Box<Self> {
        Box::new(InMemoryCaseFileRepository {
            case_files: HashMap::new(),
        })
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.case_files.insert(case_file.case_number, case_file);
        Ok(())
    }
}
