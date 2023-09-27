use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, body};
use actix_files::Files;
use website::entities::case_file::CaseFileEntity;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

async fn bldhnd_intro() -> impl Responder {
    "intro"
    
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let crime = CaseFileEntity {
        case_number: 0,
        datetime: 0,
        evidence: Vec::new(),
    };
    dbg!(crime);
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