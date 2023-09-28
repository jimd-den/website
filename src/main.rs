use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, body};
use actix_files::Files;
use website::{domains::{entities::case_file::CaseFileEntity, usecases::casefileusecase::CaseFileUseCase}, interfaces::repositories::casefilerepository_mem::InMemoryCaseFileRepository, controllers::casefilecontroller::CaseFileController};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

async fn bldhnd_intro() -> impl Responder {
    "intro"
    
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let casefile = CaseFileEntity {
        case_number: 0,
        title: "Hi".to_string(),
        datetime: 0,
        evidence: Vec::new(),
    };
    let use_case: Box<dyn CaseFileUseCase> = InMemoryCaseFileRepository::new();
    let controller = CaseFileController::new(use_case);
    match controller.create(casefile) {
        Ok(case_file) => println!("Case file created: {:?}", case_file),
        Err(e) => println!("Error creating case_file {}", e),
        }

    let path = "./static";
    let abs_path = std::fs::canonicalize(&path).unwrap();
    dbg!(abs_path);
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./static"))
            .service(
                web::scope("/bloodhound")
                .route("/intro", web::get().to(bldhnd_intro)),
            )
        })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}