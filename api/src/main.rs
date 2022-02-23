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
        .get(controllers::words::list);

    app.at("/api/v1/words")
        .with(authorization.clone())
        .post(controllers::words::create);

    app.at("/api/v1/words/:id")
        .with(authorization.clone())
        .put(controllers::words::update);

    app.at("/api/v1/words/:id")
        .with(authorization.clone())
        .delete(controllers::words::delete);

    app.at("/api/v1/words/random")
        .with(authorization.clone())
        .get(controllers::words::random);

    // ---- FOLDERS ENDPOINTS ----

    app.at("/api/v1/folders")
        .with(authorization.clone())
        .get(controllers::folders::list);

    app.at("/api/v1/folders")
        .with(authorization.clone())
        .post(controllers::folders::create);

    app.at("/api/v1/folders/:id")
        .with(authorization.clone())
        .get(controllers::folders::get);

    app.at("/api/v1/folders/:id")
        .with(authorization.clone())
        .put(controllers::folders::update);

    app.at("/api/v1/folders/:id")
        .with(authorization.clone())
        .delete(controllers::folders::delete);

    // ---- QUIZ ENDPOINTS ----

    app.at("/api/v1/quiz")
        .with(authorization.clone())
        .post(controllers::quiz::initialize);

    app.at("/api/v1/quiz/result")
        .with(authorization.clone())
        .post(controllers::quiz::insert_result);

    // ---- STAT ENDPOINTS ----
    app.at("/api/v1/stats")
        .with(authorization.clone())
        .get(controllers::stats::get_user_statistics);

    app.listen(format!("{}:{}", host, port)).await?;

    Ok(())
}
