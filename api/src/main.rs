use actix_web::{web, App, HttpResponse, HttpServer, Result};

use lib::Word;
mod words;

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(Word::from_value("health"))
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await   

}
