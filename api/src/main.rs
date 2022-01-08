mod constants;
mod controllers;
mod error;
mod middleware;
mod models;
mod state;

use lib::utils::load_env;
use std::env;
use tide::{
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
};

use crate::{middleware::auth::Authorization, state::State};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let _ = load_env();

    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8080".to_string(),
    };

    let host = match env::var("HOST") {
        Ok(host) => host,
        Err(_) => "0.0.0.0".to_string(),
    };

    tide::log::start();
    let mut app = tide::with_state(State::new());

    app.with(
        CorsMiddleware::new()
            .allow_methods(
                "GET, POST, PUT, DELETE, OPTIONS"
                    .parse::<HeaderValue>()
                    .unwrap(),
            )
            .allow_origin(Origin::from("*")),
    );

    let authorization = Authorization::new(
        env::var("WORDBANK_AUTH_TOKEN").expect("'WORDBANK_AUTH_TOKEN' env expected"),
    );

    // ---- WORD ENDPOINTS ----

    app.at("/api/v1/words")
        .with(authorization.clone())
        .get(controllers::words_list);

    app.at("/api/v1/words")
        .with(authorization.clone())
        .post(controllers::word_create);

    app.at("/api/v1/words/:id")
        .with(authorization.clone())
        .put(controllers::word_update);

    app.at("/api/v1/words/:id")
        .with(authorization.clone())
        .delete(controllers::words_delete);

    // ---- FOLDERS ENDPOINTS ----

    app.at("/api/v1/folders")
        .with(authorization.clone())
        .get(controllers::folders_list);

    app.at("/api/v1/folders")
        .with(authorization.clone())
        .post(controllers::folder_create);

    app.at("/api/v1/folders/:id")
        .with(authorization.clone())
        .get(controllers::folder_get);

    app.at("/api/v1/folders/:id")
        .with(authorization.clone())
        .put(controllers::folder_update);

    app.at("/api/v1/folders/:id")
        .with(authorization.clone())
        .delete(controllers::folder_delete);

    app.at("/api/v1/quiz")
        .with(authorization.clone())
        .post(controllers::quiz_initialize);

    app.listen(format!("{}:{}", host, port)).await?;

    Ok(())
}
