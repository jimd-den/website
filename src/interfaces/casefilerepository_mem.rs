use std::collections::HashMap;

use crate::entities::case_file::CaseFileEntity;

use super::casefilerepository::CaseFileRepository;

pub struct InMemoryCaseFileRepository {
    case_files: HashMap<u32, CaseFileEntity>
}

impl dyn CaseFileRepository {
    pub fn new() -> Self {
        InMemoryCaseFileRepository {
            case_files: HashMap::new(),
        }
    }
}