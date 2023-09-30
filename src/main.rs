
use website::{domains::{entities::case_file::CaseFileEntity, usecases::createcasefile::CreateCaseFile}, interfaces::repositories::{casefilerepository_mem::InMemoryCaseFileRepository, casefilerepository::CaseFileRepositoryNew}};
use website::interfaces::repositories::casefilerepository::CaseFileRepository;
fn main() {
    let casefile = CaseFileEntity {
        case_number: 0,
        title: "Hi".to_string(),
        datetime: 0,
        evidence: Vec::new(),
    };
    let mut casefile_repo = InMemoryCaseFileRepository::new();  
    let mut casefile_use_case = CreateCaseFile::new(Box::new(casefile_repo));
    let result = casefile_use_case.create_case_file(casefile);
    println!("{:?}", result);

}